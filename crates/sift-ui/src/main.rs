//! Sift Launcher's interface, in egui.
//!
//! **Why egui.** The frame log measured iced at a 26-62 ms p95 against egui's 17.6-18.6 ms
//! on the same machine, same window, same list -- egui re-emits everything every frame and
//! still holds a tighter tail, because the cost is what a toolkit does between the widget
//! and the present. And the interface this launcher wants next -- dynamic elements that
//! morph in place -- is a function of state in immediate mode and a subtree diff in a
//! retained tree.
//!
//! **Why the numbers in `geometry` are measured.** Every element is placed with a
//! rectangle read off a capture of the Modrinth app with `tools/parity/edges.py`; the
//! parity requirement is +/-2 px, and a layout engine's own idea of spacing is exactly the
//! thing that would miss it.
//!
//! **Why it draws nothing when idle.** The window asks for a frame while a hover, a glide
//! or a transition is in flight and stops when it lands. The slice this grew from called
//! `ctx.request_repaint()` every frame, which is the behaviour `docs/LIGHTNESS.md` names as
//! the idle-CPU budget's failure mode.

/// The product's mark, embedded and uploaded once: the exe is the product, so its logo
/// travels inside it rather than beside it.
mod motion;
mod skins_assets;
mod drop;
mod duplicate;
// The screenshot-groups store. The page does not call it yet: its dialog could not be driven in this
// environment, and an unverified control is worse than a missing one.
#[allow(dead_code)]
mod groups;
mod account;
mod brand;
mod serverart;
mod data;
mod fonts;
mod frame_log;
mod game;
mod geometry;
mod icons;
mod install;
mod pages;
mod panel;
mod images;
mod nbt;
mod search;
mod shell;
/// The skins on this machine, and the folder they live in.
mod skins;
mod skins3d;
/// The reference's palette, geometry, type and motion as egui values.
mod theme;
/// The software backend: egui drawn by this crate's own rasteriser, straight into the X buffer. Chosen with
/// SIFT_SOFT=1 in the environment, because the OpenGL path loads 53 MiB of llvmpipe to render on a machine with no GPU.
mod soft;
mod widgets;

use egui;

#[cfg(feature = "gl")]
fn main() -> eframe::Result<()> {
    // SIFT_SOFT=1 runs the software backend: this crate's own rasteriser instead of OpenGL, which on a machine with no
    // GPU driver loads 53 MiB of llvmpipe to compile shaders for a CPU. See soft/mod.rs.
    if std::env::var("SIFT_SOFT").is_ok() {
        return match soft::run() {
            Ok(()) => Ok(()),
            Err(error) => {
                eprintln!("software backend failed: {error}");
                std::process::exit(1);
            }
        };
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([geometry::WINDOW.0, geometry::WINDOW.1])
            .with_min_inner_size([960.0, 540.0])
            .with_title("Sift Launcher")
            // The window draws its own title bar -- mark, breadcrumb, update pill, minimise, maximise and
            // close -- so the window manager's decorations are a second bar above it. The reference has
            // none: measured on the same X server, its rail's first icon sits at 71 and ours at 93, and
            // the 22-point difference is exactly the decoration this removes.
            .with_decorations(false),
        ..Default::default()
    };
    eframe::run_native(
        "Sift Launcher",
        options,
        Box::new(|cc| {
            // The generated tokens, not egui's defaults: the palette is the reference's.
            let shell = shell::Shell::default();
            // Inter at the reference's weights, then the reference's palette: a theme
            // without its type is half a theme, and the first capture showed the gap.
            fonts::install(&cc.egui_ctx);
            // The mark's texture, uploaded before the first frame, so the title bar's first
            // paint is the product's logo rather than the plate that stands in for it. Both
            // calls are process-wide setup and neither depends on the other.
            brand::install(&cc.egui_ctx);
            serverart::install(&cc.egui_ctx);
            cc.egui_ctx.set_visuals(shell.theme.colors());
            // The reference's transitions, measured from its own source rather than guessed:
            // `duration-200` appears 36 times in `apps/app-frontend/src`, `duration-150` 12, `duration-300` 8,
            // `duration-250` twice and `duration-100` once. 200 ms is the duration the app is built on, and
            // egui's default is 83 ms -- fast enough that a hover or a selected pill snaps rather than eases.
            cc.egui_ctx.all_styles_mut(|style| {
                style.animation_time = 0.2;
            });
            Ok(Box::new(App {
                shell,
                log: frame_log::FrameLog::default(),
            }))
        }),
    )
}

/// The application: one shell, and the frame log when somebody asked for it.
#[cfg(feature = "gl")]
struct App {
    shell: shell::Shell,
    log: frame_log::FrameLog,
}

#[cfg(feature = "gl")]
impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        // eframe 0.36 hands the app a bare `Ui` and its own docs say to wrap it in a panel
        // "to remedy this" -- and `CentralPanel::show` in this version takes a `&mut Ui`, not
        // a context, which is what two earlier attempts got wrong. A bare Ui paints correctly
        // and registers no interactive widgets: every `ui.interact` in the shell was a no-op,
        // which is why clicks changed nothing while the events arrived.
        let App { shell, log } = self;
        log.begin();
        let mut animating = false;
        let screen = ui.max_rect();
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
                animating = shell.ui(ui);
                // Inside the panel, on the panel's own ui: that is the shape the settings
                // modal's fields use and the only one in this codebase proven to paint a
                // widget. Placed after the panel returned, the dialog's field received input
                // and drew nothing -- egui culled it against a clip the panel had already set.
                if shell.picker.is_some() {
                    ctx.request_repaint();
                }
                // **The queued request is promoted here, before the guard.** `Action::Create` has no clock; this frame does,
                // and the dialog only draws once `create_open` is already true -- so a stamp inside `draw_create` would
                // never run for a dialog that is not yet open, which is exactly the state it has to open from.
                if shell.create_open_pending {
                    shell.create_open = true;
                    shell.create_opened = ctx.input(|input| input.time);
                    shell.create_open_pending = false;
                    ctx.request_repaint();
                }
                if shell.create_open {
                    shell.draw_create(&ctx, screen, ui);
                }
                if shell.edit.is_some() {
                    shell.draw_edit(&ctx, screen, ui);
                    ctx.request_repaint();
                }
                if shell.picker.is_some() {
                    shell.draw_picker(&ctx, screen, ui);
                }
            });
        log.end();
        // Only while something is in flight. An idle window draws nothing at all, which is
        // both the budget in docs/LIGHTNESS.md and the reason a fan does not spin.
        // A window that only redraws while something is in flight never shows a keystroke in
        // an open dialog: the modal holds no animation, so the frame that received the typing
        // is the last one drawn until something else happens. While the dialog is up, keep the
        // frames coming.
        if shell.create_open {
            ctx.request_repaint();
        }
        if animating {
            ctx.request_repaint();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shell;

    /// The shell opens on Home, like the reference: a launcher that opened on a catalogue
    /// would look like a store. This test was written when Discover was the default, and it
    /// caught the change rather than the change quietly passing.
    #[test]
    fn a_fresh_shell_opens_on_home() {
        let shell = shell::Shell::default();
        assert_eq!(shell.rail, shell::Rail::Home);
    }
}

/// **The software backend is the default.** The 20 MB gate cannot be met by OpenGL on a machine without a GPU driver --
/// measured at 111.3 MiB, of which 53 MiB was llvmpipe's shader compiler -- and this build's own rasteriser draws the same
/// interface in a quarter of that. Build with `--features gl` and set `SIFT_GL=1` to use the OpenGL path instead.
#[cfg(not(feature = "gl"))]
fn main() {
    if let Err(error) = soft::run() {
        eprintln!("software backend failed: {error}");
        std::process::exit(1);
    }
}
