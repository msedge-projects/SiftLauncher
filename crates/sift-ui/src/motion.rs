//! Tweens: the app's first motion, on egui's own clock.
//!
//! The reference is a browser, so every state change it draws is a CSS transition with a
//! duration and a curve. This crate had the durations and the curves as data (`theme::MOTION`,
//! `theme::EASE_IN_OUT`, `theme::EASE_OUT_EXPO`) and no evaluator, which is why every hover,
//! press and modal in the old build was a hard cut. This module is the evaluator: one tween
//! per id, stored in egui's own memory, driven by `Context::input().time`.
//!
//! Two rules it exists to keep:
//!
//! * **A tween owns the frame.** While a tween is in flight it calls
//!   `Context::request_repaint()`, because this app only draws on demand and
//!   `request_repaint_after` has been observed not to wake it (docs/NOTES.md).
//! * **A tween is monotone in its target.** Retargeting starts a new tween from the value
//!   on screen, not from the old target, so an interrupted hover does not jump.
//!
//! The curve is CSS's `cubic-bezier(x1, y1, x2, y2)`: the control points are solved for
//! x = progress, then evaluated for y, exactly as a browser does it.

use eframe::egui::{self, Id};

/// A tween's state, as egui memory holds it.
#[derive(Clone, Copy, Debug)]
struct Tween {
    from: f32,
    to: f32,
    start: f64,
    millis: f32,
    curve: [f32; 4],
}

/// Solve a cubic bezier for `t`, where the curve's x goes 0..1 as `t` goes 0..1.
///
/// CSS fixes the end points at (0,0) and (1,1) and takes the two middle control points,
/// which is why this needs only the four numbers.
pub fn bezier(curve: [f32; 4], progress: f32) -> f32 {
    let p = progress.clamp(0.0, 1.0);
    if p <= 0.0 {
        return 0.0;
    }
    if p >= 1.0 {
        return 1.0;
    }
    let [x1, y1, x2, y2] = curve;
    let sample = |t: f32, a: f32, b: f32| -> f32 {
        let u = 1.0 - t;
        3.0 * u * u * t * a + 3.0 * u * t * t * b + t * t * t
    };
    // Bisection is enough: every curve this app uses is monotone in x, and 24 halvings put
    // the answer inside a ten-thousandth of a step.
    let (mut lo, mut hi) = (0.0_f32, 1.0_f32);
    for _ in 0..24 {
        let mid = 0.5 * (lo + hi);
        if sample(mid, x1, x2) < p {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    sample(0.5 * (lo + hi), y1, y2)
}

/// The value of `id`'s tween this frame, moving toward `target`.
///
/// `millis` and `curve` are the transition the reference's stylesheet states for whatever is
/// moving; they come from `theme` where the sheet's numbers live, so a call site cannot
/// quietly invent a duration.
pub fn value(ctx: &egui::Context, id: Id, target: f32, millis: u32, curve: [f32; 4]) -> f32 {
    let now = ctx.input(|input| input.time);
    let millis = millis.max(1) as f32;
    let tween = ctx.data_mut(|data| {
        let tween = data.get_temp_mut_or_insert_with(id, || Tween {
            from: target,
            to: target,
            start: now,
            millis,
            curve,
        });
        let running = tween.from + (tween.to - tween.from) * bezier(
            tween.curve,
            ((now - tween.start) as f32 / (tween.millis / 1000.0)).clamp(0.0, 1.0),
        );
        if (tween.to - target).abs() > f32::EPSILON || tween.curve != curve {
            // Retarget from what is on screen, so an interrupted move continues from the
            // pixel the reader is looking at.
            *tween = Tween { from: running, to: target, start: now, millis, curve };
        }
        *tween
    });
    let t = ((now - tween.start) as f32 / (millis / 1000.0)).clamp(0.0, 1.0);
    if t < 1.0 {
        // The next frame belongs to this tween: an idle app that repaints every vsync for a
        // finished transition fails the idle budget the crate is held to.
        ctx.request_repaint();
    }
    tween.from + (tween.to - tween.from) * bezier(tween.curve, t)
}

/// The same, for a boolean: 0.0 while false, 1.0 while true.
pub fn bool_value(ctx: &egui::Context, id: Id, on: bool, millis: u32, curve: [f32; 4]) -> f32 {
    value(ctx, id, if on { 1.0 } else { 0.0 }, millis, curve)
}

/// Milliseconds since `id` first became true, or `None` while it is false.
///
/// A tooltip's delay is not a transition: nothing moves for 300 ms and then the pill
/// appears. That needs a start time, not a progress value, so it has its own state.
pub fn held_for(ctx: &egui::Context, id: Id, on: bool) -> Option<f32> {
    let now = ctx.input(|input| input.time);
    let start = ctx.data_mut(|data| -> Option<f64> {
        let key = id.with("held-since");
        if on {
            let entry = data.get_temp_mut_or_insert_with(key, || now);
            if *entry == 0.0 {
                *entry = now;
            }
            Some(*entry)
        } else {
            data.remove_temp::<f64>(key);
            None
        }
    });
    start.map(|start| ((now - start) * 1000.0) as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ends are fixed and the curve is monotone between them: a transition that overshot
    /// its endpoints would show a plate bigger than its button.
    #[test]
    fn a_bezier_starts_at_zero_ends_at_one_and_never_goes_backwards() {
        let curves = [
            crate::theme::EASE_IN_OUT.curve,
            crate::theme::EASE_OUT_EXPO.curve,
            [0.0, 0.0, 1.0, 1.0],
        ];
        for curve in curves {
            assert_eq!(bezier(curve, 0.0), 0.0);
            assert_eq!(bezier(curve, 1.0), 1.0);
            let mut last = 0.0;
            for step in 0..=100 {
                let y = bezier(curve, step as f32 / 100.0);
                assert!(y >= last - 1e-4, "went backwards at {step}: {y} after {last}");
                last = y;
            }
        }
    }

    /// A linear curve must be the identity, which is what proves the solver is solving for
    /// x and not just re-parameterising t.
    #[test]
    fn a_linear_curve_is_the_identity() {
        for step in 0..=20 {
            let t = step as f32 / 20.0;
            assert!((bezier([0.0, 0.0, 1.0, 1.0], t) - t).abs() < 1e-3, "at {t}");
        }
    }

    /// ease-out-expo leads linear for the whole middle of the transition -- that is what
    /// makes a plate feel like it arrives rather than grows.
    #[test]
    fn ease_out_expo_leads_linear() {
        let curve = crate::theme::EASE_OUT_EXPO.curve;
        for step in 1..20 {
            let t = step as f32 / 20.0;
            assert!(bezier(curve, t) > t, "at {t}");
        }
    }
}
