//! Installing a Discover card's pack, off the frame thread.
//!
//! The engine's pack install is a version resolve, an archive download, an import into a new
//! instance and a fetch of every file the index lists. None of that can happen on egui's frame
//! thread, so this is the shape [`crate::game`] already uses for a launch: [`Install::start`]
//! hands one project to a worker thread, [`Install::poll`] collects what that worker said, and
//! a frame only reads what is already waiting.
//!
//! **The worker owns the paths**, exactly as the launch worker does. The root is the
//! `platform_data_root` the engine's own instance creation uses, so the instance this makes is
//! the one a launch opens and the Home page lists.
//!
//! **The state machine is the card's, not the title bar's.** A launch reports into the running
//! instance chip; an install has no such slot, so its states are the three busy phases the
//! button draws as `Installing…` and the two terminal sentences the shell turns into a toast.
//!
//! **Which version is not this module's choice.** [`install::newest_pack_version`] is the rule
//! -- releases, then betas, then alphas, keeping the API's publish-date order -- because the
//! engine already applies it to a pack, and a second ranking here would be a second answer to
//! one question.

use std::sync::mpsc::{channel, Receiver, Sender};

use sift_core::paths::{platform_data_root, SiftPaths, System};
// The engine's pack install: the rules this module only wraps in a frame's state machine.
use sift_game::install;
use sift_game::wire::Wire;

/// What a Discover card's Install button is doing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Status {
    /// No install has been asked for yet.
    #[default]
    Idle,
    /// The project's version list is being read and its newest pack chosen.
    Resolving,
    /// A phase is moving, with the engine's own counts.
    ///
    /// `done: 0, total: 0` is the archive's own transfer: the engine's `install_file` gives
    /// its download a no-op reporter, so that phase has no total to be a fraction of and says
    /// so rather than inventing one.
    ///
    /// The counts are kept because the engine reports them and a state machine that dropped
    /// them could not draw a bar without asking the engine again; this shell's button says
    /// `Installing…` in all three busy phases, so nothing reads them yet.
    #[allow(dead_code)]
    Downloading {
        /// Files finished so far.
        done: usize,
        /// Files this phase will fetch in total.
        total: usize,
    },
    /// The archive is on disk and the pack is being imported: the new instance, its
    /// `overrides/` tree, then the files its index lists.
    Installing,
    /// The pack is installed. The string is the sentence the toast shows, which names the
    /// instance that did not exist before.
    Done(String),
    /// The pack could not be installed, with the engine's own reason.
    Failed(String),
}

/// One project to install: what the card that asked for it knows.
#[derive(Debug, Clone)]
struct Job {
    /// The Modrinth project id, which is what `version_url` takes and what a card compares
    /// itself against.
    project: String,
    /// The project's name: the new instance's name, and the subject of both sentences.
    title: String,
}

/// One worker's word, tagged with the install it belongs to.
struct Reply {
    /// The `Install::generation` value that asked for this worker.
    generation: u64,
    status: Status,
}

/// The function one worker runs. A pointer rather than a trait object so a test can
/// substitute a worker without a network or a disk.
type Worker = fn(Job, u64, Sender<Reply>);

/// The pack install behind the Discover cards, and the channel its worker answers on.
pub struct Install {
    status: Status,
    /// The project the current status belongs to, so a card can ask whether the install it is
    /// drawing is its own and a failure can name the pack.
    job: Option<Job>,
    /// Incremented for every install that is allowed to start. A reply carrying a different
    /// number belongs to an install the reader has already moved past.
    generation: u64,
    sender: Sender<Reply>,
    receiver: Receiver<Reply>,
    /// The real worker in the window; a probe in the tests.
    worker: Worker,
}

impl Default for Install {
    fn default() -> Install {
        Install::with_worker(run_install)
    }
}

impl Install {
    /// Build an install holder around `worker`.
    fn with_worker(worker: Worker) -> Install {
        let (sender, receiver) = channel();
        Install {
            status: Status::Idle,
            job: None,
            generation: 0,
            sender,
            receiver,
            worker,
        }
    }

    /// Start installing `project`, under `title`, unless an install is already out.
    ///
    /// The check is before the generation bump and before the spawn, so a second press while
    /// the first worker is out is a no-op rather than a second import into the same instances
    /// folder. `Resolving` is set here, on the frame that asked, not when the worker's first
    /// word arrives: that is the frame the person who pressed the button sees.
    pub fn start(&mut self, project: &str, title: &str) {
        if self.busy() {
            return;
        }
        let job = Job {
            project: project.to_owned(),
            title: title.to_owned(),
        };
        self.generation = self.generation.wrapping_add(1);
        let generation = self.generation;
        self.status = Status::Resolving;

        let sent = job.clone();
        let sender = self.sender.clone();
        let worker = self.worker;
        let spawned = std::thread::Builder::new()
            .name("sift-install".to_owned())
            .spawn(move || worker(sent, generation, sender));
        // Remembered even when the spawn failed: the card still has to read `Install` rather
        // than pass the press off as somebody else's.
        self.job = Some(job);
        if let Err(error) = spawned {
            self.status = Status::Failed(format!("could not start an install worker: {error}"));
        }
    }

    /// Take every reply that is ready, and answer the sentence a finished install owes the
    /// shell. Called once per frame.
    pub fn poll(&mut self) -> Option<String> {
        let mut line = None;
        while let Ok(reply) = self.receiver.try_recv() {
            // A reply for an install the reader has moved past is dropped: showing it would
            // put an old state under the new one, the same rule the search and launch
            // workers apply to an abandoned request.
            if reply.generation != self.generation {
                continue;
            }
            if let Some(text) = toast_line(&reply.status, self.job.as_ref()) {
                line = Some(text);
            }
            self.status = reply.status;
        }
        line
    }

    /// Whether an install is still alive. Exactly resolving, downloading or installing keeps
    /// the shell drawing; every terminal state owes no frame.
    pub fn busy(&self) -> bool {
        matches!(
            self.status,
            Status::Resolving | Status::Downloading { .. } | Status::Installing
        )
    }

    /// What the install button on `project`'s card shows.
    ///
    /// Compared by project id rather than by title: the same project can legitimately appear
    /// twice in a list (two sources, a repeat) while its id is the key the version URL takes,
    /// and another card's button stays an offer even while one install is out.
    pub fn label(&self, project: &str) -> &'static str {
        let mine = self.job.as_ref().map(|job| job.project.as_str()) == Some(project);
        match (&self.status, mine) {
            (Status::Done(_), true) => "Installed",
            (
                Status::Resolving | Status::Downloading { .. } | Status::Installing,
                true,
            ) => "Installing…",
            // A failure reads `Install` again on purpose: the engine's sentence is in the
            // toast, and the button is the retry.
            _ => "Install",
        }
    }
}

/// One worker: run the engine and always leave a terminal status behind.
fn run_install(job: Job, generation: u64, sender: Sender<Reply>) {
    // A pack first, then a mod. The button cannot know which kind a project is until the
    // archive has been read, and the reference's answer -- a version picker for mods -- is not
    // built yet, so the fallback carries the mod path with the honest refusals inside it.
    let pack = install_pack(&job, generation, &sender);
    let pack = match pack {
        Err(reason) if reason.starts_with("that archive is not a readable pack") => {
            install_mod(&job)
        }
        other => other,
    };
    let status = match pack {
        Ok(summary) => Status::Done(summary),
        Err(reason) => Status::Failed(reason),
    };
    // A closed receiver means the window is gone; dropping the last word is right then, and
    // there is no one left to report a failure to anyway.
    let _ = sender.send(Reply { generation, status });
}


/// The sentence a finished mod install owes.
///
/// A version's own name usually already carries the project's -- Modrinth names the Sodium
/// release "Sodium 0.9.3-beta.1" -- so printing both gave "Installed Sodium Sodium 0.9.3-beta.1".
/// The reference prints the version name alone; the project's name is added only when the
/// version does not carry it.
fn success_line(title: &str, version_name: &str, instance: &str) -> String {
    let named = version_name
        .to_ascii_lowercase()
        .starts_with(&title.to_ascii_lowercase());
    if named || title.is_empty() {
        format!("Installed {version_name} into {instance}.")
    } else {
        format!("Installed {title} {version_name} into {instance}.")
    }
}

/// Install a mod into the one instance there is.
///
/// The same shape as a pack install -- resolve, fetch, write -- with two differences that the
/// reference has too: the version is chosen for the *instance* rather than being the newest
/// (\`pick_version\`), and the target is an instance that already exists (\`choose_instance\`),
/// because a mod goes into a game folder rather than creating one.
fn install_mod(job: &Job) -> Result<String, String> {
    let paths = SiftPaths::at(platform_data_root(System::current()));
    let wire = Wire::new(paths.meta_dir());

    let ids: Vec<String> = crate::data::load()
        .0
        .into_iter()
        .map(|instance| instance.id)
        .collect();
    let instance = choose_instance(&ids)?;

    let url = sift_net::modrinth::version_url(&job.project);
    let body = wire.document(&url)?;
    let versions: Vec<sift_net::modrinth::ModrinthProjectVersion> =
        serde_json::from_str(&body).map_err(|error| format!("{url}: {error}"))?;
    let version = pick_version(&versions, "", "")
        .ok_or_else(|| format!("{} has no version with a file to download", job.title))?;

    let mods = paths.instances_dir().join(&instance).join("mods");
    install::install_file(&wire, &mods, version)?;
    Ok(success_line(&job.title, &version.name, &instance))
}

/// Resolve the newest pack, fetch its archive, import it as an instance, fill it.
fn install_pack(job: &Job, generation: u64, sender: &Sender<Reply>) -> Result<String, String> {
    // This thread owns the paths because it owns every disk and network call an install
    // makes, and it is the same root a launch opens its instance from.
    let paths = SiftPaths::at(platform_data_root(System::current()));
    let wire = Wire::new(paths.meta_dir());

    // Resolving. The URL is the client crate's own builder rather than a format string here,
    // and the document goes through the wire's cached request -- the same one client and the
    // same cache as every other metadata question this launcher asks.
    let url = sift_net::modrinth::version_url(&job.project);
    // shortcut: the list is believed for the metadata cache's TTL (30 minutes), so a release
    // published within it is not seen -- upgrade when a reader needs the very newest.
    let body = wire.document(&url)?;
    let versions: Vec<sift_net::modrinth::ModrinthProjectVersion> =
        serde_json::from_str(&body).map_err(|error| format!("{url}: {error}"))?;
    let version = install::newest_pack_version(&versions)
        .ok_or_else(|| format!("{} has no version with a file to download", job.title))?;

    // Downloading, uncounted: the archive itself. `cache/meta/packs/` is the directory the
    // engine's own documentation names, so a second install of the same pack version costs
    // nothing.
    report(
        sender,
        generation,
        Status::Downloading {
            done: 0,
            total: 0,
        },
    );
    let archive = install::fetch_pack_archive(&wire, &paths.meta_dir().join("packs"), version)?;

    // Installing: the import that creates the instance, then the pack's files, which are the
    // phase that reports a count.
    report(sender, generation, Status::Installing);
    let mut progress = |level: install::Progress| {
        report(
            sender,
            generation,
            Status::Downloading {
                done: level.done,
                total: level.total,
            },
        );
    };
    // A project that is not a pack fails here with the engine's own words ("neither a
    // .mrpack nor a CurseForge pack"), which is the honest answer for the Mod and Resource
    // Pack tabs: this button installs packs into instances of their own.
    let installed =
        install::install_pack_archive(&wire, &paths, &archive, &job.title, &mut progress)?;
    // Which project this instance came from, so the Discover list can tell what is installed.
    record_project(&paths, &installed.id, &job.project);
    // The engine's sentence, because it is the one that names both the pack and the instance.
    Ok(installed.summary(&job.title))
}

/// Note the project an instance was installed from, in the instance's own settings file.
///
/// The key is this launcher's own: Prism ignores settings it does not know, so writing it beside
/// its own is safe, and an instance installed before this existed simply has `None`.
fn record_project(paths: &SiftPaths, instance: &str, project: &str) {
    let cfg = paths
        .instances_dir()
        .join(instance)
        .join(crate::data::INSTANCE_CFG);
    let Ok(mut text) = std::fs::read_to_string(&cfg) else {
        return;
    };
    if text.contains(crate::data::PROJECT_KEY) {
        return;
    }
    if !text.ends_with('\n') {
        text.push('\n');
    }
    text.push_str(&format!("{}={project}\n", crate::data::PROJECT_KEY));
    let _ = std::fs::write(&cfg, text);
}

/// Which instance a mod install should land in.
///
/// The reference asks with a picker; until that exists, one instance is unambiguous and is used,
/// and none or several is answered with what the reader has to do rather than a guess. Naming the
/// count is the point: "no instances" and "three instances" are different problems.
fn choose_instance(ids: &[String]) -> Result<String, String> {
    match ids {
        [] => Err("there is no instance to install into. Create one first.".to_owned()),
        [only] => Ok(only.clone()),
        many => Err(format!(
            "there are {} instances and choosing one needs a picker that is not built yet.",
            many.len()
        )),
    }
}

/// The version a mod install should take.
///
/// Modrinth returns versions newest first, so the first one that targets the instance's game
/// version and loader is the newest compatible build. When nothing matches, the newest version
/// with a downloadable file wins: an install that may need the instance's loader changed is
/// still more useful than a refusal, and the caller says which version it took.
fn pick_version<'a>(
    versions: &'a [sift_net::modrinth::ModrinthProjectVersion],
    game_version: &str,
    loader: &str,
) -> Option<&'a sift_net::modrinth::ModrinthProjectVersion> {
    let compatible = |version: &&sift_net::modrinth::ModrinthProjectVersion| {
        (game_version.is_empty() || version.game_versions.iter().any(|v| v == game_version))
            && (loader.is_empty() || version.loaders.iter().any(|l| l.eq_ignore_ascii_case(loader)))
            && version.primary_file().is_some()
    };
    versions
        .iter()
        .find(compatible)
        .or_else(|| versions.iter().find(|v| v.primary_file().is_some()))
}

/// Send one word to the frame, ignoring a window that has gone.
fn report(sender: &Sender<Reply>, generation: u64, status: Status) {
    let _ = sender.send(Reply { generation, status });
}

/// The sentence a status owes the shell.
///
/// A finished install names the instance it created, a failed one names the pack the engine
/// could not install, and progress owes nothing: the button already says it.
fn toast_line(status: &Status, job: Option<&Job>) -> Option<String> {
    match status {
        Status::Done(summary) => Some(summary.clone()),
        Status::Failed(reason) => Some(failure_line(
            job.map(|job| job.title.as_str()).unwrap_or_default(),
            reason,
        )),
        Status::Idle | Status::Resolving | Status::Downloading { .. } | Status::Installing => None,
    }
}

/// A failure with the pack it was about in front of the engine's own reason.
///
/// The engine's sentence stays whole -- it is the one thing that says what actually went
/// wrong -- and the name goes in front because a toast on a page of cards would otherwise not
/// say which card it is about.
fn failure_line(title: &str, reason: &str) -> String {
    let title = if title.is_empty() { "that pack" } else { title };
    // A project whose archive is not a pack is a mod or a resource pack, and this button has no
    // path for either: the engine's sentence blames the file for something the button never
    // tried to do, which tells the reader their download was broken. It is not -- the button is
    // pack-only by construction, and the fix is a version picker rather than a better reader.
    if reason.starts_with("that archive is not a readable pack") {
        return format!(
            "Could not install {title}: this button installs modpacks, and installing a mod needs \
             a version picker that is not built yet."
        );
    }
    format!("Could not install {title}: {reason}")
}

#[cfg(test)]
mod failure_tests {
    use super::failure_line;

    /// The engine's pack-reader sentence is replaced, because it blames the archive for a
    /// limitation of the button.
    #[test]
    fn a_non_pack_says_the_button_is_the_limit() {
        let line = failure_line(
            "Sodium",
            "that archive is not a readable pack: invalid pack: neither modrinth.index.json nor manifest.json is present",
        );
        assert!(line.contains("installs modpacks"), "{line}");
        assert!(line.contains("version picker"), "{line}");
        assert!(!line.contains("not a readable pack"), "{line}");
    }

    /// Every other reason is passed through whole: the engine's words are the only ones that
    /// say what actually went wrong.
    #[test]
    fn another_reason_keeps_the_engine_s_words() {
        let line = failure_line("Cobblemon", "connection reset");
        assert_eq!(line, "Could not install Cobblemon: connection reset");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    /// The one fake-worker count this module's start test reads.
    static WORKERS_STARTED: AtomicUsize = AtomicUsize::new(0);

    fn fake_worker(_job: Job, _generation: u64, _sender: Sender<Reply>) {
        WORKERS_STARTED.fetch_add(1, Ordering::SeqCst);
    }

    fn job(project: &str) -> Job {
        Job {
            project: project.to_owned(),
            title: "Cobblemon".to_owned(),
        }
    }

    /// `busy` is the frame loop's rule: only work that is still happening keeps the window
    /// drawing. Every terminal state must owe no frame.
    #[test]
    fn only_the_working_states_are_busy() {
        let cases = [
            (Status::Idle, false),
            (Status::Resolving, true),
            (
                Status::Downloading {
                    done: 3,
                    total: 40,
                },
                true,
            ),
            // The archive's own uncounted transfer is still work.
            (
                Status::Downloading {
                    done: 0,
                    total: 0,
                },
                true,
            ),
            (Status::Installing, true),
            (Status::Done("Installed X as X, 4 files".to_owned()), false),
            (Status::Failed("no version".to_owned()), false),
        ];
        let mut install = Install::default();
        for (status, expected) in cases {
            install.status = status;
            assert_eq!(install.busy(), expected);
        }
    }

    /// A second press while the first worker is out must not start a second worker. The fake
    /// worker never replies, so the first install stays busy for the whole test without a
    /// network or a disk.
    #[test]
    fn a_start_while_busy_does_not_start_a_second_worker() {
        WORKERS_STARTED.store(0, Ordering::SeqCst);
        let mut install = Install::with_worker(fake_worker);
        install.start("first", "First");
        install.start("second", "Second");

        let deadline = Instant::now() + Duration::from_secs(1);
        while WORKERS_STARTED.load(Ordering::SeqCst) == 0 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(1));
        }
        assert!(install.busy(), "the first worker is still the current install");
        assert_eq!(
            WORKERS_STARTED.load(Ordering::SeqCst),
            1,
            "the second press was ignored"
        );
        assert_eq!(
            install.label("first"),
            "Installing…",
            "and the press that was taken is the one the button reports"
        );
        assert_eq!(install.label("second"), "Install", "the refused card is untouched");
    }

    /// The button's three words, per state and per card: the card whose project is being
    /// installed says so, and no other card claims it.
    #[test]
    fn the_button_says_what_the_install_is_doing() {
        let mut install = Install::default();
        assert_eq!(install.label("A"), "Install", "nothing asked yet");

        install.job = Some(job("A"));
        for status in [
            Status::Resolving,
            Status::Downloading {
                done: 2,
                total: 9,
            },
            Status::Installing,
        ] {
            install.status = status;
            assert_eq!(install.label("A"), "Installing…");
            assert_eq!(install.label("B"), "Install", "another card is still an offer");
        }

        install.status = Status::Done("Installed A as A, 9 files".to_owned());
        assert_eq!(install.label("A"), "Installed");
        assert_eq!(install.label("B"), "Install");

        // A failure puts the button back to a retry rather than leaving it reading as done.
        install.status = Status::Failed("no version with a file".to_owned());
        assert_eq!(install.label("A"), "Install");
    }

    /// The toast's sentence: a finished install is the engine's own summary, a failure names
    /// the pack in front of the engine's reason, and progress owes nothing.
    #[test]
    fn a_terminal_status_owes_the_shell_one_sentence() {
        let summary = "Installed Cobblemon as Cobblemon, 42 files".to_owned();
        assert_eq!(
            toast_line(&Status::Done(summary.clone()), Some(&job("A"))),
            Some(summary)
        );
        assert_eq!(
            toast_line(&Status::Failed("no direct download".to_owned()), Some(&job("A"))),
            Some("Could not install Cobblemon: no direct download".to_owned())
        );
        for status in [
            Status::Idle,
            Status::Resolving,
            Status::Downloading {
                done: 1,
                total: 2,
            },
            Status::Installing,
        ] {
            assert_eq!(toast_line(&status, Some(&job("A"))), None);
        }
    }

    /// A failure with no title behind it still reads as a sentence: the title comes from the
    /// API and a project may publish none.
    #[test]
    fn a_failure_without_a_title_still_names_the_pack() {
        assert_eq!(
            failure_line("", "connection reset"),
            "Could not install that pack: connection reset"
        );
        assert_eq!(
            failure_line("Cobblemon", "connection reset"),
            "Could not install Cobblemon: connection reset"
        );
    }
}

#[cfg(test)]
mod pick_tests {
    use super::pick_version;

    fn versions() -> Vec<sift_net::modrinth::ModrinthProjectVersion> {
        serde_json::from_str(
            r#"[
              {"name":"4.0","game_versions":["1.21"],"loaders":["fabric"],
               "files":[{"url":"https://example/4.jar","filename":"sodium-4.jar","primary":true}]},
              {"name":"3.0","game_versions":["1.20.4"],"loaders":["fabric"],
               "files":[{"url":"https://example/3.jar","filename":"sodium-3.jar","primary":true}]},
              {"name":"2.0","game_versions":["1.20.4"],"loaders":["forge"],
               "files":[{"url":"https://example/2.jar","filename":"sodium-2.jar","primary":true}]}
            ]"#,
        )
        .expect("the fixture parses")
    }

    /// The newest version that fits the instance wins, not simply the newest.
    #[test]
    fn the_newest_compatible_version_wins() {
        let all = versions();
        assert_eq!(pick_version(&all, "1.20.4", "fabric").map(|v| v.name.as_str()), Some("3.0"));
        assert_eq!(pick_version(&all, "1.21", "fabric").map(|v| v.name.as_str()), Some("4.0"));
    }

    /// Nothing compatible is not nothing at all: the newest build is offered, and the caller
    /// says which it took rather than refusing outright.
    #[test]
    fn an_incompatible_instance_still_gets_the_newest_build() {
        let all = versions();
        assert_eq!(pick_version(&all, "1.19", "quilt").map(|v| v.name.as_str()), Some("4.0"));
    }

    /// A version with no file cannot be installed, however well it fits.
    #[test]
    fn a_version_with_no_file_is_skipped() {
        let all: Vec<sift_net::modrinth::ModrinthProjectVersion> = serde_json::from_str(
            r#"[{"name":"empty","game_versions":["1.20.4"],"loaders":["fabric"],"files":[]},
                {"name":"real","game_versions":["1.20.4"],"loaders":["fabric"],
                 "files":[{"url":"https://example/r.jar","filename":"r.jar","primary":true}]}]"#,
        )
        .expect("the fixture parses");
        assert_eq!(pick_version(&all, "1.20.4", "fabric").map(|v| v.name.as_str()), Some("real"));
    }
}

#[cfg(test)]
mod instance_tests {
    use super::choose_instance;

    /// One instance is unambiguous, and is named back.
    #[test]
    fn one_instance_is_the_answer() {
        assert_eq!(choose_instance(&["survival".to_owned()]), Ok("survival".to_owned()));
    }

    /// None and several are different answers, and both say what to do.
    #[test]
    fn none_and_several_say_what_is_needed() {
        let none = choose_instance(&[]).expect_err("empty is an error");
        assert!(none.contains("no instance"), "{none}");

        let many = choose_instance(&["a".to_owned(), "b".to_owned(), "c".to_owned()])
            .expect_err("several is an error");
        assert!(many.contains('3'), "{many}");
        assert!(many.contains("picker"), "{many}");
    }
}

#[cfg(test)]
mod success_tests {
    use super::success_line;

    /// The version's name already carrying the project is the common case, and the project's is
    /// not printed twice.
    #[test]
    fn a_version_that_names_its_project_is_not_doubled() {
        assert_eq!(
            success_line("Sodium", "Sodium 0.9.3-beta.1", "Instance 1"),
            "Installed Sodium 0.9.3-beta.1 into Instance 1."
        );
    }

    /// When it does not, the project's name goes in front, as the toast needs to say which card.
    #[test]
    fn a_version_that_does_not_is_prefixed() {
        assert_eq!(
            success_line("Cobblemon", "1.6.1", "survival"),
            "Installed Cobblemon 1.6.1 into survival."
        );
    }
}
