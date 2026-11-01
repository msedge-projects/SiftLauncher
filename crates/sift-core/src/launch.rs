//! Pure launch-plan builders — port of the argument/script generation in
//! `MinecraftInstance.cpp` (+ `Commandline::splitArgs`). No process is
//! spawned here; `sift-net`/the GUI turn a [`LaunchArgs`] into a command.

use crate::version::LaunchProfile;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// Authentication data injected into the launch arguments
/// (`AuthSession` subset used by argument generation).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuthSession {
    /// Player name (`auth_player_name`).
    pub player_name: String,
    /// Player UUID (`auth_uuid`).
    pub uuid: String,
    /// Access token (`auth_access_token`).
    pub access_token: String,
    /// Legacy session id (`auth_session`).
    pub session: String,
    /// `user_type` (`msa` / `legacy` / ...).
    pub user_type: String,
    /// Serialized user properties (`user_properties`).
    pub user_properties: String,
    /// Launch in demo mode (`--demo`).
    pub demo: bool,
}

/// Server/world to join at launch (`MinecraftTarget`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LaunchTarget {
    /// Server address (empty = no server join).
    pub address: String,
    /// Server port.
    pub port: u16,
    /// Quick-play world name (empty = none).
    pub world: String,
}

/// Window parameters for the launch script.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowParams {
    /// Window width.
    pub width: i64,
    /// Window height.
    pub height: i64,
    /// Launch maximized (legacy instances print `maximized`).
    pub maximized: bool,
}

/// Inputs for [`java_arguments`], mirroring the settings/traits read in
/// `MinecraftInstance::javaArguments` and `extraArguments`.
#[derive(Debug, Clone)]
pub struct JavaArgsOptions {
    /// Profile traits (checked for `FirstThreadOnMacOS`).
    pub traits: BTreeSet<String>,
    /// `JvmArgs` setting (split with [`split_command_args`]).
    pub jvm_args: String,
    /// `MinMemAlloc` in MiB.
    pub min_mem: i64,
    /// `MaxMemAlloc` in MiB.
    pub max_mem: i64,
    /// `PermGen` in MiB.
    pub perm_gen: i64,
    /// Whether jar mods are present (adds the FML ignore flags).
    pub jar_mods_present: bool,
    /// Profile `+jvmArgs` (token-replaced).
    pub addn_jvm_arguments: Vec<String>,
    /// Agents as `(jar_path, argument)` pairs.
    pub agents: Vec<(String, String)>,
    /// Major version of the selected Java runtime.
    pub java_major: i64,
    /// Whether the `legacyServices` online-fixes flag applies.
    pub online_fixes: bool,
    /// Host platform.
    pub platform: crate::paths::System,
    /// Linux: whether `xrandr` exists (disables the LWJGL2 workaround).
    pub xrandr_available: bool,
    /// Linux: whether the `org.lwjgl` (LWJGL2) component is present.
    pub has_lwjgl2: bool,
    /// Custom/system OpenAL path (macOS/Linux native workarounds).
    pub native_openal: Option<String>,
    /// Custom/system GLFW path.
    pub native_glfw: Option<String>,
    /// Custom/system SDL path.
    pub native_sdl: Option<String>,
    /// Instance window title (macOS dock name).
    pub window_title: String,
    /// Token map for `+jvmArgs` substitution.
    pub token_map: BTreeMap<String, String>,
}

/// Substitute `${token}` occurrences. Unknown tokens are kept verbatim
/// (`replaceTokensIn`); matching is non-greedy, i.e. `${a}${b}` resolves
/// both separately.
pub fn replace_tokens(text: &str, map: &BTreeMap<String, String>) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("${") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        match after.find('}') {
            Some(end) => {
                let key = &after[..end];
                // Non-greedy nesting: if the key itself contains a nested
                // `${`, the outer `${` is not a real token start. Emit it
                // literally and rescan so the inner `${b}` resolves first.
                // `${a${b}}` -> `${` + `a` + `B` + `}` = `${aB}`.
                if key.contains("${") {
                    out.push_str("${");
                    rest = after;
                    continue;
                }
                match map.get(key) {
                    Some(v) => out.push_str(v),
                    None => {
                        out.push_str("${");
                        out.push_str(key);
                        out.push('}');
                    }
                }
                rest = &after[end + 1..];
            }
            None => {
                out.push_str("${");
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

/// Shell-like argument splitting (`Commandline::splitArgs`): double quotes
/// toggle quoting and are removed; whitespace outside quotes splits.
pub fn split_command_args(args: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quoted = false;
    for c in args.chars() {
        if c == '"' {
            in_quoted = !in_quoted;
        } else if !in_quoted && c.is_whitespace() {
            if !current.is_empty() {
                out.push(std::mem::take(&mut current));
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

/// The profile variable mapping (`makeProfileVarMapping`).
pub fn profile_var_map(
    profile: &LaunchProfile,
    instance_name: &str,
    instance_id: &str,
    _instance_root: &Path,
    game_root: &Path,
    game_assets: &Path,
    assets_root: &Path,
    library_directory: &Path,
) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    m.insert("profile_name".into(), instance_name.to_string());
    m.insert("version_name".into(), profile.minecraft_version.clone());
    m.insert(
        "version_type".into(),
        profile.minecraft_version_type.clone(),
    );
    m.insert(
        "game_directory".into(),
        game_root.to_string_lossy().into_owned(),
    );
    m.insert(
        "game_assets".into(),
        game_assets.to_string_lossy().into_owned(),
    );
    m.insert(
        "assets_root".into(),
        assets_root.to_string_lossy().into_owned(),
    );
    m.insert("assets_index_name".into(), profile.assets_or_default().id);
    m.insert(
        "library_directory".into(),
        library_directory.to_string_lossy().into_owned(),
    );
    let _ = instance_id; // kept in the signature to mirror the C++ context
    m
}

/// The full variable mapping for `profile` running in `instance`, with every
/// *shared* directory taken from the data root.
///
/// This exists because the primitive above takes five paths that all look
/// plausible and only one of which is right. `${assets_root}` and
/// `${library_directory}` name the **launcher-wide** `assets/` and
/// `libraries/` — the ones `sift-desktop`'s install step downloads into and
/// the ones every instance shares — while `${game_directory}` and
/// `${game_assets}` are inside the instance. A caller that passed the instance
/// folder for all of them built a command line that pointed the game at
/// `instances/<id>/assets`, where the asset index is not, and Minecraft exits
/// with a failed asset index before it draws a frame.
///
/// `MinecraftInstance::makeProfileVarMapping` is the reference; the rules it
/// encodes are:
///
/// * `${game_directory}` — the instance's game root (`minecraft/`);
/// * `${game_assets}` — [`crate::assets::game_assets_dir`], which is the
///   instance's `resources/` for a `legacy`/`pre-1.6` index and the shared
///   `assets/<index id>` for every modern one;
/// * `${assets_root}` — the shared `assets/`;
/// * `${library_directory}` — the shared `libraries/`.
pub fn instance_var_map(
    paths: &crate::paths::SiftPaths,
    instance: &crate::instance::Instance,
    profile: &LaunchProfile,
) -> BTreeMap<String, String> {
    let assets_root = paths.assets_dir();
    profile_var_map(
        profile,
        &instance.name(),
        &instance.id(),
        instance.root(),
        &instance.game_root(),
        &crate::assets::game_assets_dir(
            &assets_root,
            &profile.assets_or_default().id,
            &instance.resources_dir(),
        ),
        &assets_root,
        &paths.libraries_dir(),
    )
}

/// `--username ${auth_player_name} ...` style argument list
/// (`processMinecraftArgs`). `vars` must already contain the profile
/// mapping; session tokens are added here.
pub fn process_minecraft_args(
    profile: &LaunchProfile,
    session: Option<&AuthSession>,
    target: Option<&LaunchTarget>,
    vars: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut args: Vec<String> = profile
        .minecraft_arguments
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    for tweaker in &profile.tweakers {
        args.push("--tweakClass".into());
        args.push(tweaker.clone());
    }
    if let Some(target) = target {
        if !target.address.is_empty() {
            if profile.has_trait("feature:is_quick_play_multiplayer") {
                args.push("--quickPlayMultiplayer".into());
                args.push(format!("{}:{}", target.address, target.port));
            } else {
                args.push("--server".into());
                args.push(target.address.clone());
                args.push("--port".into());
                args.push(target.port.to_string());
            }
        } else if !target.world.is_empty()
            && profile.has_trait("feature:is_quick_play_singleplayer")
        {
            args.push("--quickPlaySingleplayer".into());
            args.push(target.world.clone());
        }
    }
    let mut map = vars.clone();
    if let Some(session) = session {
        map.insert("auth_session".into(), session.session.clone());
        map.insert("auth_access_token".into(), session.access_token.clone());
        map.insert("auth_player_name".into(), session.player_name.clone());
        map.insert("auth_uuid".into(), session.uuid.clone());
        map.insert("user_properties".into(), session.user_properties.clone());
        map.insert("user_type".into(), session.user_type.clone());
        if session.demo {
            args.push("--demo".into());
        }
    }
    args.iter().map(|a| replace_tokens(a, &map)).collect()
}

/// The `${...}` tokens a launch fills, and the only ones the version-file
/// translation keeps in an argument list.
///
/// Two functions do the filling: [`profile_var_map`] supplies the profile's own
/// tokens and [`process_minecraft_args`] adds the session's. An argument whose
/// value is a token outside this list reaches the game as the literal text
/// `${that_token}`, because [`replace_tokens`] leaves an unknown token alone —
/// so the translation drops such an argument (and the flag it belongs to)
/// rather than pass it on. Mojang's `--clientId ${clientid} --xuid ${auth_xuid}`
/// pair, published for 1.19 and later, is the only one measured, and the mirror
/// this launcher used to read drops the same pair.
///
/// Kept beside the two functions that fill it so that a reader adding a token
/// here sees the whole rule at once.
pub const FILLED_TOKENS: &[&str] = &[
    "profile_name",
    "version_name",
    "version_type",
    "game_directory",
    "game_assets",
    "assets_root",
    "assets_index_name",
    "library_directory",
    "auth_session",
    "auth_access_token",
    "auth_player_name",
    "auth_uuid",
    "user_properties",
    "user_type",
];

/// The `INST_*` environment variables Prism exports
/// (`MinecraftInstance::getVariables`).
pub fn instance_env_vars(
    instance_name: &str,
    instance_id: &str,
    instance_root: &Path,
    game_root: &Path,
    java_path: &Path,
    java_args_joined: &str,
) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    m.insert("INST_NAME".into(), instance_name.to_string());
    m.insert("INST_ID".into(), instance_id.to_string());
    m.insert(
        "INST_DIR".into(),
        instance_root.to_string_lossy().into_owned(),
    );
    m.insert(
        "INST_MC_DIR".into(),
        game_root.to_string_lossy().into_owned(),
    );
    m.insert("INST_JAVA".into(), java_path.to_string_lossy().into_owned());
    m.insert("INST_JAVA_ARGS".into(), java_args_joined.to_string());
    m.insert("NO_COLOR".into(), "1".to_string());
    m
}

/// Build the JVM argument list (`javaArguments` + `extraArguments`).
/// Argument order matches the C++ exactly.
pub fn java_arguments(opts: &JavaArgsOptions) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    args.push("-Duser.language=en".into());
    // extraArguments():
    args.extend(split_command_args(&opts.jvm_args));
    if opts.jar_mods_present {
        args.push("-Dfml.ignoreInvalidMinecraftCertificates=true".into());
        args.push("-Dfml.ignorePatchDiscrepancies=true".into());
    }
    for arg in &opts.addn_jvm_arguments {
        args.push(replace_tokens(arg, &opts.token_map));
    }
    for (jar, arg) in &opts.agents {
        let mut a = format!("-javaagent:{jar}");
        if !arg.is_empty() {
            a.push('=');
            a.push_str(arg);
        }
        args.push(a);
    }
    for (flag, path) in [
        ("-Dorg.lwjgl.openal.libname=", &opts.native_openal),
        ("-Dorg.lwjgl.glfw.libname=", &opts.native_glfw),
        ("-Dorg.lwjgl.sdl.libname=", &opts.native_sdl),
    ] {
        if let Some(p) = path {
            if !p.is_empty() {
                args.push(format!("{flag}{p}"));
            }
        }
    }
    // javaArguments() platform workarounds:
    if opts.platform == crate::paths::System::MacOS {
        args.push("-Xdock:icon=icon.png".into());
        args.push(format!("-Xdock:name=\"{}\"", opts.window_title));
        if opts.traits.contains("FirstThreadOnMacOS") {
            args.push("-XstartOnFirstThread".into());
        }
    }
    if opts.platform == crate::paths::System::Windows {
        args.push("-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump".into());
    }
    if opts.platform == crate::paths::System::Linux && opts.has_lwjgl2 && !opts.xrandr_available {
        args.push("-DLWJGL_DISABLE_XRANDR=true".into());
    }
    let (min, max) = (opts.min_mem, opts.max_mem);
    if min < max {
        args.push(format!("-Xms{min}m"));
        args.push(format!("-Xmx{max}m"));
    } else {
        args.push(format!("-Xms{max}m"));
        args.push(format!("-Xmx{min}m"));
    }
    if opts.java_major < 8 {
        // requiresPermGen; the default (64) emits no flag
        if opts.perm_gen != 64 {
            args.push(format!("-XX:PermSize={}m", opts.perm_gen));
        }
    }
    if opts.java_major >= 9 && opts.online_fixes {
        args.push("--add-opens".into());
        args.push("java.base/java.net=ALL-UNNAMED".into());
    }
    args
}

/// Generate the Prism launch-script text
/// (`MinecraftInstance::createLaunchScript`): a line-based protocol consumed
/// by the external launcher parts.
pub fn create_launch_script(
    profile: &LaunchProfile,
    session: Option<&AuthSession>,
    target: Option<&LaunchTarget>,
    mc_args: &[String],
    window: WindowParams,
    launcher_brand: &str,
    launcher_version: &str,
    instance_name: &str,
) -> String {
    let mut out = String::new();
    if !profile.main_class.is_empty() {
        out.push_str(&format!("mainClass {}\n", profile.main_class));
    }
    if !profile.applet_class.is_empty() {
        out.push_str(&format!("appletClass {}\n", profile.applet_class));
    }
    if let Some(target) = target {
        if !target.address.is_empty() {
            out.push_str(&format!("serverAddress {}\n", target.address));
            out.push_str(&format!("serverPort {}\n", target.port));
        } else if !target.world.is_empty() {
            out.push_str(&format!("worldName {}\n", target.world));
        }
    }
    for param in mc_args {
        out.push_str(&format!("param {param}\n"));
    }
    out.push_str(&format!("windowTitle {launcher_brand}: {instance_name}\n"));
    let window_params = if window.maximized && profile.is_legacy() {
        "maximized".to_string()
    } else {
        format!("{}x{}", window.width, window.height)
    };
    out.push_str(&format!("windowParams {window_params}\n"));
    out.push_str(&format!("launcherBrand {launcher_brand}\n"));
    out.push_str(&format!("launcherVersion {launcher_version}\n"));
    out.push_str(&format!("instanceName {instance_name}\n"));
    // Quirk faithfully ported: the C++ writes the instance *name* here too.
    out.push_str(&format!("instanceIconKey {instance_name}\n"));
    out.push_str("instanceIconPath icon.png\n");
    if let Some(session) = session {
        out.push_str(&format!("userName {}\n", session.player_name));
        out.push_str(&format!("sessionId {}\n", session.session));
    }
    for t in &profile.traits {
        out.push_str(&format!("traits {t}\n"));
    }
    if profile.has_trait("legacyServices") {
        out.push_str("onlineFixes true\n");
    }
    let launcher = if profile.is_legacy() {
        "legacy"
    } else {
        "standard"
    };
    out.push_str(&format!("launcher {launcher}\n"));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::version::{AssetIndexInfo, Library};

    fn profile() -> LaunchProfile {
        LaunchProfile {
            minecraft_version: "1.20.4".into(),
            minecraft_version_type: "release".into(),
            minecraft_assets: Some(AssetIndexInfo::bare("17")),
            minecraft_arguments: "--username ${auth_player_name} --version ${version_name} --gameDir ${game_directory}".into(),
            main_class: "net.minecraft.client.main.Main".into(),
            ..Default::default()
        }
    }

    fn vars() -> BTreeMap<String, String> {
        let mut m = BTreeMap::new();
        m.insert("version_name".to_string(), "1.20.4".into());
        m.insert("game_directory".to_string(), "/inst/minecraft".into());
        m
    }

    /// A data root with one usable instance in it.
    fn rooted() -> (
        tempfile::TempDir,
        crate::paths::SiftPaths,
        crate::instance::Instance,
    ) {
        let dir = tempfile::tempdir().unwrap();
        let paths = crate::paths::SiftPaths::at(dir.path());
        std::fs::create_dir_all(paths.instances_dir()).unwrap();
        let instance =
            crate::instance::Instance::create(&paths.instances_dir(), "Rooted", "1.21.1")
                .expect("instance fixture");
        (dir, paths, instance)
    }

    #[test]
    fn shared_directories_come_from_the_data_root_and_not_the_instance() {
        // The bug this function exists to make impossible: `${assets_root}` and
        // `${library_directory}` name the launcher-wide folders, where the
        // install step actually put those files. Pointing the game at
        // `instances/<id>/assets` makes Minecraft fail its asset index before it
        // draws a frame, and the command line looks entirely reasonable.
        let (_dir, paths, instance) = rooted();
        let map = instance_var_map(&paths, &instance, &profile());

        assert_eq!(map["assets_root"], paths.assets_dir().to_string_lossy());
        assert_eq!(
            map["library_directory"],
            paths.libraries_dir().to_string_lossy()
        );
        assert_eq!(
            map["game_directory"],
            instance.game_root().to_string_lossy()
        );
        assert_eq!(map["assets_index_name"], "17");
        // A modern index is hash-addressed, so `${game_assets}` is the shared
        // folder for that index id — not the instance, and not `resources/`.
        assert_eq!(
            map["game_assets"],
            paths.assets_dir().join("17").to_string_lossy()
        );

        // And the paths are not merely "somewhere else": they are the folders
        // the shared files live in, while the instance's own are different.
        assert_ne!(
            map["assets_root"],
            instance.root().join("assets").to_string_lossy()
        );
        assert_ne!(
            map["library_directory"],
            instance.local_libraries_dir().to_string_lossy()
        );
        assert!(!paths.assets_dir().starts_with(instance.root()));
        assert!(!paths.libraries_dir().starts_with(instance.root()));
    }

    #[test]
    fn a_legacy_index_points_game_assets_at_the_instance_resources() {
        // `legacy`/`pre-1.6` predate the hash-addressed layout: their files are
        // reconstructed by logical name *inside the instance*, so the folder the
        // game is told to read is the instance's, and the install step's
        // reconstruction target has to be the same one.
        let (_dir, paths, instance) = rooted();
        let mut legacy = profile();
        legacy.minecraft_assets = Some(AssetIndexInfo::bare("legacy"));
        let map = instance_var_map(&paths, &instance, &legacy);
        assert_eq!(
            map["game_assets"],
            instance.resources_dir().to_string_lossy()
        );
        assert_eq!(
            map["game_assets"],
            instance.game_root().join("resources").to_string_lossy()
        );
        // The shared folders do not move for it.
        assert_eq!(map["assets_root"], paths.assets_dir().to_string_lossy());
    }

    #[test]
    fn the_mapping_is_what_the_argument_substitution_reads() {
        // End to end through the one call the launch line makes: the argument
        // the game actually receives must name the shared assets folder.
        let (_dir, paths, instance) = rooted();
        let mut p = profile();
        p.minecraft_arguments =
            "--assetsDir ${assets_root} --assetIndex ${assets_index_name}".into();
        let map = instance_var_map(&paths, &instance, &p);
        let args = process_minecraft_args(&p, None, None, &map);
        assert_eq!(
            args,
            vec![
                "--assetsDir".to_string(),
                paths.assets_dir().to_string_lossy().into_owned(),
                "--assetIndex".to_string(),
                "17".to_string(),
            ]
        );
    }

    #[test]
    fn the_token_list_is_exactly_what_a_launch_fills() {
        // The list is the translation's other half: it decides which of Mojang's
        // arguments survive. Both directions are checked here, because either
        // one being wrong is silent -- a missing token drops an argument the
        // launcher could have filled, and an extra one keeps an argument it
        // cannot, which reaches the game as `${clientid}`.
        let (_dir, paths, instance) = rooted();
        let mut p = profile();
        p.minecraft_arguments = FILLED_TOKENS
            .iter()
            .map(|token| format!("--{token} ${{{token}}}"))
            .collect::<Vec<_>>()
            .join(" ");
        let map = instance_var_map(&paths, &instance, &p);

        // Without a session, the profile's map is all there is: what is left
        // unfilled is the session's own six, named here rather than read off the
        // list, so that this half of the check is a statement about
        // `process_minecraft_args` and not a restatement of `FILLED_TOKENS`.
        let without = process_minecraft_args(&p, None, None, &map);
        let mut survived: Vec<&str> = without
            .iter()
            .filter_map(|arg| arg.strip_prefix("${").and_then(|t| t.strip_suffix('}')))
            .collect();
        survived.sort_unstable();
        assert_eq!(
            survived,
            [
                "auth_access_token",
                "auth_player_name",
                "auth_session",
                "auth_uuid",
                "user_properties",
                "user_type",
            ],
            "the session's tokens are the only ones a profile alone does not fill"
        );

        // With one, nothing in the list is left unfilled -- the direction that
        // matters, since a token here that the session does not supply would be
        // dropped from every translated file.
        let with = process_minecraft_args(&p, Some(&AuthSession::default()), None, &map);
        assert!(
            !with.iter().any(|arg| arg.contains("${")),
            "the session did not fill: {:?}",
            with.iter()
                .filter(|arg| arg.contains("${"))
                .collect::<Vec<_>>()
        );

        // And the other direction, for the profile's half: `profile_var_map`
        // filling a token the list does not name would drop an argument the
        // launcher could have filled.
        for key in map.keys() {
            assert!(
                FILLED_TOKENS.contains(&key.as_str()),
                "{key} is filled but not in FILLED_TOKENS"
            );
        }
    }

    #[test]
    fn token_replacement_is_non_greedy_and_keeps_unknowns() {
        let mut map = BTreeMap::new();
        map.insert("a".to_string(), "A".into());
        map.insert("b".to_string(), "B".into());
        assert_eq!(replace_tokens("${a}${b}", &map), "AB");
        assert_eq!(replace_tokens("x${nope}y", &map), "x${nope}y");
        assert_eq!(replace_tokens("unclosed ${a", &map), "unclosed ${a");
        assert_eq!(replace_tokens("plain", &map), "plain");
        assert_eq!(replace_tokens("${a}", &map), "A");
        assert_eq!(replace_tokens("${a${b}}", &map), "${aB}"); // first } closes
    }

    #[test]
    fn split_args_handles_quotes_like_commandline() {
        assert_eq!(
            split_command_args("-Xmx1G -Dfoo=bar"),
            vec!["-Xmx1G", "-Dfoo=bar"]
        );
        assert_eq!(
            split_command_args("-Xms\"1 2G\" -Xmx4G"),
            vec!["-Xms1 2G", "-Xmx4G"]
        );
        assert_eq!(split_command_args("   "), Vec::<String>::new());
        assert_eq!(split_command_args("\"a b\"c"), vec!["a bc"]);
        assert_eq!(split_command_args("-X\"m\"x"), vec!["-Xmx"]);
    }

    #[test]
    fn minecraft_args_expand_tokens_tweakers_and_join_target() {
        let p = profile();
        let session = AuthSession {
            player_name: "Steve".into(),
            uuid: "uuid-1".into(),
            access_token: "tok".into(),
            session: "sess".into(),
            user_type: "msa".into(),
            user_properties: "{}".into(),
            demo: false,
        };
        let target = LaunchTarget {
            address: "mc.example.com".into(),
            port: 25565,
            world: String::new(),
        };
        let args = process_minecraft_args(&p, Some(&session), Some(&target), &vars());
        assert!(args.contains(&"--username".to_string()) && args.contains(&"Steve".to_string()));
        assert!(args.contains(&"1.20.4".to_string()));
        assert!(args.contains(&"/inst/minecraft".to_string()));
        assert_eq!(*args.iter().rev().nth(3).unwrap(), "--server");
        assert_eq!(*args.last().unwrap(), "25565");
    }

    #[test]
    fn quick_play_used_when_trait_present() {
        let mut p = profile();
        p.traits.insert("feature:is_quick_play_multiplayer".into());
        let target = LaunchTarget {
            address: "s".into(),
            port: 1234,
            world: String::new(),
        };
        let args = process_minecraft_args(&p, None, Some(&target), &vars());
        assert!(args
            .windows(2)
            .any(|w| w[0] == "--quickPlayMultiplayer" && w[1] == "s:1234"));
        let mut p = profile();
        p.traits.insert("feature:is_quick_play_singleplayer".into());
        let target = LaunchTarget {
            address: String::new(),
            port: 0,
            world: "World".into(),
        };
        let args = process_minecraft_args(&p, None, Some(&target), &vars());
        assert!(args
            .windows(2)
            .any(|w| w[0] == "--quickPlaySingleplayer" && w[1] == "World"));
    }

    #[test]
    fn demo_flag_added_for_demo_sessions() {
        let session = AuthSession {
            demo: true,
            ..Default::default()
        };
        let args = process_minecraft_args(&profile(), Some(&session), None, &vars());
        assert!(args.contains(&"--demo".to_string()));
    }

    #[test]
    fn jvm_args_match_prism_order_and_defaults() {
        let opts = JavaArgsOptions {
            traits: BTreeSet::new(),
            jvm_args: "-XX:+UseG1GC".into(),
            min_mem: 128,
            max_mem: 4096,
            perm_gen: 64,
            jar_mods_present: false,
            addn_jvm_arguments: vec!["-Dcustom=${version_name}".into()],
            agents: vec![("/lib/agent.jar".into(), "opt".into())],
            java_major: 17,
            online_fixes: false,
            platform: crate::paths::System::Linux,
            xrandr_available: true,
            has_lwjgl2: true,
            native_openal: None,
            native_glfw: None,
            native_sdl: None,
            window_title: "Sift Launcher: T".into(),
            token_map: vars(),
        };
        let args = java_arguments(&opts);
        let expected = vec![
            "-Duser.language=en",
            "-XX:+UseG1GC",
            "-Dcustom=1.20.4",
            "-javaagent:/lib/agent.jar=opt",
            "-Xms128m",
            "-Xmx4096m",
        ];
        assert_eq!(args, expected);
    }

    #[test]
    fn jvm_memory_swaps_when_min_ge_max() {
        let mut opts = base_opts(crate::paths::System::Linux);
        opts.min_mem = 4096;
        opts.max_mem = 4096;
        let args = java_arguments(&opts);
        assert!(args.contains(&"-Xms4096m".to_string()) && args.contains(&"-Xmx4096m".to_string()));
        opts.min_mem = 8192;
        opts.max_mem = 1024;
        let args = java_arguments(&opts);
        assert!(args.contains(&"-Xms1024m".to_string()) && args.contains(&"-Xmx8192m".to_string()));
    }

    fn base_opts(platform: crate::paths::System) -> JavaArgsOptions {
        JavaArgsOptions {
            traits: BTreeSet::new(),
            jvm_args: String::new(),
            min_mem: 128,
            max_mem: 4096,
            perm_gen: 64,
            jar_mods_present: false,
            addn_jvm_arguments: vec![],
            agents: vec![],
            java_major: 17,
            online_fixes: false,
            platform,
            xrandr_available: true,
            has_lwjgl2: false,
            native_openal: None,
            native_glfw: None,
            native_sdl: None,
            window_title: "T".into(),
            token_map: BTreeMap::new(),
        }
    }

    #[test]
    fn platform_workarounds_applied_conditionally() {
        let mut mac = base_opts(crate::paths::System::MacOS);
        mac.traits.insert("FirstThreadOnMacOS".into());
        let mac_args = java_arguments(&mac);
        assert!(mac_args.contains(&"-Xdock:icon=icon.png".to_string()));
        assert!(mac_args.contains(&"-Xdock:name=\"T\"".to_string()));
        assert!(mac_args.contains(&"-XstartOnFirstThread".to_string()));
        // trait absent -> no flag
        let mut mac2 = base_opts(crate::paths::System::MacOS);
        mac2.window_title = "W".into();
        assert!(!java_arguments(&mac2).contains(&"-XstartOnFirstThread".to_string()));

        let win = java_arguments(&base_opts(crate::paths::System::Windows));
        assert!(win
            .iter()
            .any(|a| a.starts_with("-XX:HeapDumpPath=MojangTricks")));

        let mut linux = base_opts(crate::paths::System::Linux);
        linux.has_lwjgl2 = true;
        linux.xrandr_available = false;
        assert!(java_arguments(&linux).contains(&"-DLWJGL_DISABLE_XRANDR=true".to_string()));
        linux.xrandr_available = true;
        assert!(!java_arguments(&linux).contains(&"-DLWJGL_DISABLE_XRANDR=true".to_string()));
    }

    #[test]
    fn perm_gen_emitted_only_for_old_java_and_non_default() {
        let mut old = base_opts(crate::paths::System::Linux);
        old.java_major = 7;
        assert!(!java_arguments(&old)
            .iter()
            .any(|a| a.starts_with("-XX:PermSize"))); // default 64
        old.perm_gen = 128;
        assert!(java_arguments(&old).contains(&"-XX:PermSize=128m".to_string()));
        old.java_major = 8;
        assert!(!java_arguments(&old)
            .iter()
            .any(|a| a.starts_with("-XX:PermSize")));
    }

    #[test]
    fn online_fixes_add_opens_on_modular_java() {
        let mut opts = base_opts(crate::paths::System::Linux);
        opts.online_fixes = true;
        opts.java_major = 17;
        let a = java_arguments(&opts);
        assert!(a
            .windows(2)
            .any(|w| w[0] == "--add-opens" && w[1] == "java.base/java.net=ALL-UNNAMED"));
        opts.java_major = 7;
        assert!(!java_arguments(&opts).contains(&"--add-opens".to_string()));
    }

    #[test]
    fn jar_mod_flags_and_native_paths() {
        let mut opts = base_opts(crate::paths::System::Linux);
        opts.jar_mods_present = true;
        opts.native_openal = Some("/usr/lib/libopenal.so".into());
        let a = java_arguments(&opts);
        assert!(a.contains(&"-Dfml.ignoreInvalidMinecraftCertificates=true".to_string()));
        assert!(a.contains(&"-Dfml.ignorePatchDiscrepancies=true".to_string()));
        assert!(a.contains(&"-Dorg.lwjgl.openal.libname=/usr/lib/libopenal.so".to_string()));
    }

    #[test]
    fn launch_script_matches_line_protocol() {
        let mut p = profile();
        p.traits.insert("first".into());
        p.traits.insert("second".into());
        let session = AuthSession {
            player_name: "Steve".into(),
            session: "sess".into(),
            ..Default::default()
        };
        let mc_args = process_minecraft_args(&p, Some(&session), None, &vars());
        let script = create_launch_script(
            &p,
            Some(&session),
            None,
            &mc_args,
            WindowParams {
                width: 854,
                height: 480,
                maximized: false,
            },
            crate::PRODUCT_NAME,
            "9.0",
            "My Inst",
        );
        let lines: Vec<&str> = script.lines().collect();
        assert_eq!(lines[0], "mainClass net.minecraft.client.main.Main");
        assert!(script.contains("param --username\nparam Steve\n"));
        // The title carries the *writer's* name: a launcher that labels the
        // window it started with somebody else's brand is mislabelling it.
        assert!(script.contains("windowTitle Sift Launcher: My Inst\n"));
        assert!(script.contains("windowParams 854x480\n"));
        assert!(script.contains("launcherBrand Sift Launcher\n"));
        assert!(script.contains("launcherVersion 9.0\n"));
        assert!(script.contains("instanceName My Inst\n"));
        assert!(script.contains("instanceIconKey My Inst\n")); // quirk: name, not icon key
        assert!(script.contains("instanceIconPath icon.png\n"));
        assert!(script.contains("userName Steve\n"));
        assert!(script.contains("sessionId sess\n"));
        assert!(script.contains("traits first\n"));
        assert!(script.contains("launcher standard\n"));
        assert!(!script.contains("onlineFixes"));
    }

    #[test]
    fn launch_script_legacy_traits_switch_launcher() {
        let mut p = profile();
        p.traits.insert("legacyLaunch".into());
        let script = create_launch_script(
            &p,
            None,
            None,
            &[],
            WindowParams {
                width: 854,
                height: 480,
                maximized: true,
            },
            "B",
            "1",
            "L",
        );
        assert!(script.contains("windowParams maximized\n"));
        assert!(script.contains("launcher legacy\n"));
    }

    #[test]
    fn env_vars_match_getvariables() {
        let v = instance_env_vars(
            "Name",
            "id",
            Path::new("/i"),
            Path::new("/i/minecraft"),
            Path::new("/usr/bin/java"),
            "-Xmx1G",
        );
        assert_eq!(v["INST_NAME"], "Name");
        assert_eq!(v["INST_ID"], "id");
        assert_eq!(v["INST_MC_DIR"], "/i/minecraft");
        assert_eq!(v["INST_JAVA_ARGS"], "-Xmx1G");
        assert_eq!(v["NO_COLOR"], "1");
    }

    #[test]
    fn profile_var_map_contains_prism_tokens() {
        let mut p = profile();
        p.libraries.push(Library::default());
        let m = profile_var_map(
            &p,
            "Name",
            "id",
            Path::new("/i"),
            Path::new("/i/minecraft"),
            Path::new("/assets/17"),
            Path::new("/assets"),
            Path::new("/libraries"),
        );
        assert_eq!(m["profile_name"], "Name");
        assert_eq!(m["version_name"], "1.20.4");
        assert_eq!(m["version_type"], "release");
        assert_eq!(m["game_directory"], "/i/minecraft");
        assert_eq!(m["game_assets"], "/assets/17");
        assert_eq!(m["assets_index_name"], "17");
        assert_eq!(m["library_directory"], "/libraries");
    }


    /// The profile name and the folder id are different strings once a second
    /// instance takes a name the first one already had: `instance_var_map` is
    /// given the display name, which is what the game shows, and the id -- the
    /// folder's own name -- is not a token at all.
    #[test]
    fn a_duplicated_name_keeps_its_profile_name_and_its_folder_id_apart() {
        let dir = tempfile::tempdir().unwrap();
        let paths = crate::paths::SiftPaths::at(dir.path());
        std::fs::create_dir_all(paths.instances_dir()).unwrap();
        let first = crate::instance::Instance::create(&paths.instances_dir(), "Same", "1.21.1")
            .expect("first instance");
        let second = crate::instance::Instance::create(&paths.instances_dir(), "Same", "1.21.1")
            .expect("second instance");
        assert_eq!(first.id(), "Same");
        assert_eq!(second.id(), "Same_1");
        assert_eq!(second.name(), "Same");

        let map = instance_var_map(&paths, &second, &profile());
        assert_eq!(map["profile_name"], "Same");
        assert_ne!(map["profile_name"], second.id());
    }

    // ---- the join target -------------------------------------------------

    /// A server target is two pairs of arguments, placed after whatever the
    /// profile and its tweakers contributed, and the port is its own argument
    /// rather than glued to the address.
    ///
    /// `LaunchTarget` keeps the address and the port apart and this layer never
    /// looks inside the address: `sift-game`'s `parse_server_address` is what
    /// splits `host:port` before a target exists, so a bare host and its port
    /// arrive here as two values and leave as two arguments.
    #[test]
    fn a_server_target_becomes_the_server_and_port_pair_after_the_game_arguments() {
        let target = LaunchTarget {
            address: "mc.example.com".into(),
            port: 25565,
            world: String::new(),
        };
        let args = process_minecraft_args(&profile(), None, Some(&target), &vars());
        assert_eq!(
            args,
            vec![
                "--username",
                // No session in this call, so the token is the only thing the game
                // could receive for it -- see `replace_tokens`' rule.
                "${auth_player_name}",
                "--version",
                "1.20.4",
                "--gameDir",
                "/inst/minecraft",
                "--server",
                "mc.example.com",
                "--port",
                "25565",
            ]
        );
    }

    /// The address is what turns a join on: a target with a port and no address
    /// asks for nothing, which is the shape a rejected address leaves behind.
    ///
    /// `sift-game`'s parser answers `None` for `""`, `"   "`,
    /// `"host:notaport"` and `":25565"`, and the launcher then passes no
    /// target at all -- so this is the second of the two places that rule is
    /// visible from.
    #[test]
    fn an_empty_address_asks_for_no_join_even_when_a_port_is_set() {
        let target = LaunchTarget {
            address: String::new(),
            port: 25565,
            world: String::new(),
        };
        let args = process_minecraft_args(&profile(), None, Some(&target), &vars());
        assert!(
            !args.iter().any(|arg| arg == "--server" || arg == "--port"),
            "{args:?}"
        );
        assert!(!args.iter().any(|arg| arg.starts_with("--quickPlay")));
    }

    /// `0` is a port the game is told about, not a "no port" marker: the address
    /// alone decides whether a join is asked for, and the port is written as it
    /// arrives. (The shipping caller parses one first, and its default is 25565.)
    #[test]
    fn a_port_of_zero_is_still_an_argument() {
        let target = LaunchTarget {
            address: "mc.example.com".into(),
            port: 0,
            world: String::new(),
        };
        let args = process_minecraft_args(&profile(), None, Some(&target), &vars());
        assert_eq!(args[args.len() - 2], "--port");
        assert_eq!(args[args.len() - 1], "0");
    }

    /// An address that spells its own port is passed through whole, with the
    /// separate `--port` still beside it. That is not a bug to fix in this
    /// layer: the layer that owns the spelling is the one that splits it, and
    /// this test is what says so if a caller ever hands `host:port` straight to
    /// a launch.
    #[test]
    fn an_address_that_names_its_own_port_is_not_split_here() {
        let target = LaunchTarget {
            address: "mc.example.com:25570".into(),
            port: 25570,
            world: String::new(),
        };
        let args = process_minecraft_args(&profile(), None, Some(&target), &vars());
        let tail = &args[args.len() - 4..];
        assert_eq!(tail[0], "--server");
        assert_eq!(tail[1], "mc.example.com:25570");
        assert_eq!(tail[2], "--port");
        assert_eq!(tail[3], "25570");
    }

    /// Nothing here trims or validates the address either: whitespace is a
    /// non-empty string, so it is sent as one. The rejection happens in
    /// `parse_server_address`, before a target exists.
    #[test]
    fn a_whitespace_address_is_neither_trimmed_nor_rejected_here() {
        let target = LaunchTarget {
            address: "   ".into(),
            port: 25565,
            world: String::new(),
        };
        let args = process_minecraft_args(&profile(), None, Some(&target), &vars());
        assert_eq!(args[args.len() - 3], "   ");
    }

    /// The quick-play form is one value, `address:port`, because that is the
    /// shape the game's own quick-play argument takes -- the port is appended
    /// rather than sent as a second argument, and there is no `--server` at all.
    #[test]
    fn quick_play_multiplayer_is_one_address_and_port_value() {
        let mut p = profile();
        p.traits.insert("feature:is_quick_play_multiplayer".into());
        let target = LaunchTarget {
            address: "mc.example.com".into(),
            port: 25570,
            world: String::new(),
        };
        let args = process_minecraft_args(&p, None, Some(&target), &vars());
        assert_eq!(args[args.len() - 2], "--quickPlayMultiplayer");
        assert_eq!(args[args.len() - 1], "mc.example.com:25570");
        assert!(!args.iter().any(|arg| arg == "--server" || arg == "--port"));
    }

    /// Without the quick-play trait the same target is named the old way, which
    /// is what a profile that predates 1.20 can read. The other quick-play trait
    /// is not a substitute: the two are separate features.
    #[test]
    fn a_server_is_named_the_old_way_when_the_profile_cannot_quick_play() {
        let mut p = profile();
        p.traits.insert("feature:is_quick_play_singleplayer".into());
        let target = LaunchTarget {
            address: "s".into(),
            port: 1234,
            world: String::new(),
        };
        let args = process_minecraft_args(&p, None, Some(&target), &vars());
        assert!(!args.iter().any(|arg| arg.starts_with("--quickPlay")));
        assert_eq!(args[args.len() - 4], "--server");
    }

    /// A world needs the single-player quick-play trait to be named at all; a
    /// profile without it gets no argument, because `--quickPlaySingleplayer`
    /// is an argument the game would not understand.
    #[test]
    fn a_world_is_only_named_when_the_profile_can_quick_play_it() {
        let target = LaunchTarget {
            address: String::new(),
            port: 0,
            world: "Harbour".into(),
        };
        let args = process_minecraft_args(&profile(), None, Some(&target), &vars());
        assert!(
            !args.iter().any(|arg| arg == "--quickPlaySingleplayer"),
            "{args:?}"
        );

        let mut p = profile();
        p.traits.insert("feature:is_quick_play_singleplayer".into());
        let args = process_minecraft_args(&p, None, Some(&target), &vars());
        assert_eq!(args[args.len() - 2], "--quickPlaySingleplayer");
        assert_eq!(args[args.len() - 1], "Harbour");
    }

    /// A target that names both a server and a world joins the server: the
    /// address is tested first, and a world is only considered when there is no
    /// address. (A world is written by the shell for exactly one launch, so the
    /// two are never meant to meet -- this is what happens if they do.)
    #[test]
    fn the_address_wins_when_a_target_names_both() {
        let mut p = profile();
        p.traits.insert("feature:is_quick_play_multiplayer".into());
        p.traits.insert("feature:is_quick_play_singleplayer".into());
        let target = LaunchTarget {
            address: "s".into(),
            port: 1234,
            world: "Harbour".into(),
        };
        let args = process_minecraft_args(&p, None, Some(&target), &vars());
        assert!(args
            .windows(2)
            .any(|w| w[0] == "--quickPlayMultiplayer" && w[1] == "s:1234"));
        assert!(!args
            .iter()
            .any(|arg| arg == "--quickPlaySingleplayer" || arg == "Harbour"));
    }

    /// The demo flag is the last argument -- after the join target, whatever it
    /// was -- and only a demo session adds it.
    #[test]
    fn the_demo_flag_is_added_last_and_only_for_a_demo_session() {
        let target = LaunchTarget {
            address: "mc.example.com".into(),
            port: 25565,
            world: String::new(),
        };
        let demo = AuthSession {
            demo: true,
            ..Default::default()
        };
        let args = process_minecraft_args(&profile(), Some(&demo), Some(&target), &vars());
        assert_eq!(args[args.len() - 1], "--demo");
        assert_eq!(args.iter().filter(|arg| *arg == "--demo").count(), 1);

        let ordinary = AuthSession::default();
        let args = process_minecraft_args(&profile(), Some(&ordinary), Some(&target), &vars());
        assert!(!args.iter().any(|arg| arg == "--demo"));
    }

    /// A session's own values replace whatever the profile's map already held for
    /// the same token: the account is the truth about the account, and the map is
    /// only the profile's half of it.
    #[test]
    fn session_tokens_replace_anything_the_map_already_held() {
        let mut vars = vars();
        vars.insert("auth_player_name".into(), "stale-from-the-map".into());
        let session = AuthSession {
            player_name: "Steve".into(),
            ..Default::default()
        };
        let args = process_minecraft_args(&profile(), Some(&session), None, &vars);
        assert!(args.contains(&"Steve".to_string()));
        assert!(!args.contains(&"stale-from-the-map".to_string()));
    }

    /// Expansion happens inside an argument, not only on arguments that are
    /// nothing but a token, and a token nothing fills is left as written -- which
    /// is why `FILLED_TOKENS` exists and why the translation drops what it
    /// cannot fill instead of passing `${clientid}` on to the game.
    #[test]
    fn tokens_are_expanded_inside_an_argument_and_unknown_ones_survive() {
        let mut p = profile();
        p.minecraft_arguments =
            "-Dpack=${version_name} --clientId ${clientid} --short=${a}".into();
        let args = process_minecraft_args(&p, None, None, &vars());
        assert_eq!(
            args,
            vec!["-Dpack=1.20.4", "--clientId", "${clientid}", "--short=${a}"]
        );
    }

    /// An empty legacy argument string contributes no parameters at all, spaces
    /// or not: the split is on a single space and the empty pieces are dropped.
    #[test]
    fn an_empty_argument_list_contributes_no_parameters() {
        let mut p = profile();
        p.minecraft_arguments = String::new();
        assert!(process_minecraft_args(&p, None, None, &vars()).is_empty());
        p.minecraft_arguments = "   ".into();
        assert!(process_minecraft_args(&p, None, None, &vars()).is_empty());
    }

    /// Tweakers are appended after the legacy arguments, each as its own
    /// `--tweakClass` pair, in the order the merged profile lists them.
    #[test]
    fn tweakers_follow_the_legacy_arguments_in_the_order_they_are_given() {
        let mut p = profile();
        p.minecraft_arguments = "--a 1".into();
        p.tweakers = vec!["a.Tweaker".into(), "b.Tweaker".into()];
        assert_eq!(
            process_minecraft_args(&p, None, None, &vars()),
            vec![
                "--a",
                "1",
                "--tweakClass",
                "a.Tweaker",
                "--tweakClass",
                "b.Tweaker"
            ]
        );
    }

    /// Substitution is one pass: a value that itself looks like a token is
    /// written out as it is, and only a second call would expand it. An empty
    /// token name is looked up like any other, so `${}` is kept.
    #[test]
    fn an_expanded_value_is_not_expanded_again() {
        let mut map = BTreeMap::new();
        map.insert("a".to_string(), "${b}".into());
        map.insert("b".to_string(), "B".into());
        assert_eq!(replace_tokens("${a}", &map), "${b}");
        assert_eq!(replace_tokens(&replace_tokens("${a}", &map), &map), "B");
        assert_eq!(replace_tokens("${}", &map), "${}");
        // a dollar or a brace on its own is text
        assert_eq!(replace_tokens("$a}", &map), "$a}");
        assert_eq!(replace_tokens("a${b", &map), "a${b");
        assert_eq!(replace_tokens("}", &map), "}");
    }

    /// Quotes mark whitespace as part of the word and are themselves removed,
    /// wherever in the word they fall.
    #[test]
    fn quoted_whitespace_is_one_argument_and_the_quotes_are_dropped() {
        assert_eq!(
            split_command_args(r#"-Dx="a b" -Dy=2"#),
            vec!["-Dx=a b", "-Dy=2"]
        );
        assert_eq!(split_command_args(r#"a" "b"#), vec!["a b"]);
        assert_eq!(split_command_args(r#"-X"m"x"#), vec!["-Xmx"]);
    }

    /// An empty quoted argument is dropped rather than pushed as an empty string:
    /// the splitter only emits a word it has characters for. That matters because
    /// an empty argument is not the same as no argument to a JVM.
    #[test]
    fn an_empty_quoted_argument_contributes_nothing() {
        assert_eq!(split_command_args(r#""""#), Vec::<String>::new());
        assert_eq!(split_command_args(r#"a "" b"#), vec!["a", "b"]);
    }

    /// An unclosed quote runs to the end of the line, and a closing quote on its
    /// own ends the word rather than starting one. A space *before* the quote has
    /// already finished the word: quoting joins what follows it, it does not
    /// reach backwards.
    #[test]
    fn an_unclosed_quote_joins_the_rest_of_the_line() {
        assert_eq!(split_command_args(r#"--x "a b"#), vec!["--x", "a b"]);
        assert_eq!(split_command_args(r#"a "b"#), vec!["a", "b"]);
        assert_eq!(split_command_args(r#"a "b c"#), vec!["a", "b c"]);
        assert_eq!(split_command_args(r#"abc""#), vec!["abc"]);
    }

    /// Every kind of whitespace separates, leading and trailing whitespace leaves
    /// no empty argument behind, and the rule is Rust's own (which includes the
    /// non-breaking space) rather than an ASCII-only one.
    #[test]
    fn every_kind_of_whitespace_separates_and_trailing_space_leaves_nothing() {
        assert_eq!(split_command_args("a\tb\nc\rd"), vec!["a", "b", "c", "d"]);
        assert_eq!(split_command_args("-Xmx4G \t\n"), vec!["-Xmx4G"]);
        assert_eq!(split_command_args("  lead"), vec!["lead"]);
        assert_eq!(split_command_args("one  two"), vec!["one", "two"]);
        assert_eq!(split_command_args("a\u{a0}b"), vec!["a", "b"]);
    }

    // ---- JVM arguments ---------------------------------------------------

    /// An agent without an argument is `-javaagent:jar` with no `=`; one with
    /// an argument keeps it after a single `=`, and both keep their order.
    #[test]
    fn agents_are_written_with_their_argument_only_when_there_is_one() {
        let mut opts = base_opts(crate::paths::System::Linux);
        opts.agents = vec![
            ("/lib/a.jar".into(), String::new()),
            ("/lib/b.jar".into(), "-Dx=1".into()),
        ];
        let args = java_arguments(&opts);
        assert!(args.contains(&"-javaagent:/lib/a.jar".to_string()));
        assert!(args.contains(&"-javaagent:/lib/b.jar=-Dx=1".to_string()));
        assert!(!args.iter().any(|arg| arg == "-javaagent:/lib/a.jar="));
        let a = args
            .iter()
            .position(|arg| arg.ends_with("/lib/a.jar"))
            .unwrap();
        let b = args
            .iter()
            .position(|arg| arg.ends_with("/lib/b.jar=-Dx=1"))
            .unwrap();
        assert!(a < b);
    }

    /// Each native override names the library it replaces, in the documented
    /// order, and an empty path is not a flag.
    #[test]
    fn each_native_override_names_the_library_it_replaces() {
        let mut opts = base_opts(crate::paths::System::Linux);
        opts.native_openal = Some("/o/libopenal.so".into());
        opts.native_glfw = Some("/g/libglfw.so".into());
        opts.native_sdl = Some("/s/libSDL2.so".into());
        let args = java_arguments(&opts);
        assert!(args.contains(&"-Dorg.lwjgl.openal.libname=/o/libopenal.so".to_string()));
        assert!(args.contains(&"-Dorg.lwjgl.glfw.libname=/g/libglfw.so".to_string()));
        assert!(args.contains(&"-Dorg.lwjgl.sdl.libname=/s/libSDL2.so".to_string()));
        let pos = |prefix: &str| {
            args.iter()
                .position(|arg| arg.starts_with(prefix))
                .unwrap_or(usize::MAX)
        };
        assert!(pos("-Dorg.lwjgl.openal") < pos("-Dorg.lwjgl.glfw"));
        assert!(pos("-Dorg.lwjgl.glfw") < pos("-Dorg.lwjgl.sdl"));

        opts.native_glfw = Some(String::new());
        assert!(!java_arguments(&opts)
            .iter()
            .any(|arg| arg.starts_with("-Dorg.lwjgl.glfw")));
    }

    /// macOS names the dock even on a profile that does not ask for the first
    /// thread: the two are separate flags, and only the second is a trait.
    #[test]
    fn macos_names_the_dock_without_the_first_thread_trait() {
        let args = java_arguments(&base_opts(crate::paths::System::MacOS));
        assert!(args.contains(&"-Xdock:icon=icon.png".to_string()));
        assert!(args.contains(&"-Xdock:name=\"T\"".to_string()));
        assert!(!args.contains(&"-XstartOnFirstThread".to_string()));
    }


    /// The `JvmArgs` setting is a command line, not one argument: it is split
    /// by the same rules as everything else, so a quoted path stays one argument
    /// and the pieces arrive in order right after the language flag.
    #[test]
    fn the_jvm_args_setting_is_split_before_it_is_passed_on() {
        let mut opts = base_opts(crate::paths::System::Linux);
        opts.jvm_args = "-Dx=\"a b\"  -Dy=2".into();
        let args = java_arguments(&opts);
        assert_eq!(args[0], "-Duser.language=en");
        assert_eq!(args[1], "-Dx=a b");
        assert_eq!(args[2], "-Dy=2");
        // the two spaces between them produced no argument at all
        assert_eq!(args[3], "-Xms128m");
    }


    /// The online-fixes flag is the one that arrived with the module system:
    /// Java 9 is the first version that has `--add-opens`, and 8 is the last
    /// that does not.
    #[test]
    fn the_online_fixes_flag_starts_at_java_nine() {
        for (major, expected) in [(8, false), (9, true)] {
            let mut opts = base_opts(crate::paths::System::Linux);
            opts.online_fixes = true;
            opts.java_major = major;
            assert_eq!(
                java_arguments(&opts).contains(&"--add-opens".to_string()),
                expected,
                "java {major}"
            );
        }
    }

    /// The profile's own `+jvmArgs` are token-replaced one at a time; a token
    /// nothing fills is left as written rather than dropped, because the
    /// argument may still be the one a loader needs.
    #[test]
    fn a_profile_jvm_argument_keeps_the_tokens_nothing_fills() {
        let mut opts = base_opts(crate::paths::System::Linux);
        opts.addn_jvm_arguments = vec![
            "-Dprofile=${profile_name}".into(),
            "-Dunknown=${nope}".into(),
        ];
        opts.token_map = BTreeMap::from([("profile_name".to_string(), "Fabric".to_string())]);
        let args = java_arguments(&opts);
        assert!(args.contains(&"-Dprofile=Fabric".to_string()));
        assert!(args.contains(&"-Dunknown=${nope}".to_string()));
    }

    // ---- the launch script -----------------------------------------------

    fn window() -> WindowParams {
        WindowParams {
            width: 854,
            height: 480,
            maximized: false,
        }
    }

    /// A legacy profile is started through its applet class, and a profile with
    /// no main class writes no `mainClass` line rather than an empty one.
    #[test]
    fn a_legacy_profile_names_its_applet_class_and_no_main_class() {
        let mut p = profile();
        p.main_class = String::new();
        p.applet_class = "net.minecraft.client.MinecraftApplet".into();
        let script = create_launch_script(&p, None, None, &[], window(), "B", "1", "I");
        assert_eq!(
            script.lines().next().unwrap(),
            "appletClass net.minecraft.client.MinecraftApplet"
        );
        assert!(!script.contains("mainClass"));
    }

    /// A server target is two lines of the script; a world target is one, and it
    /// is written whether or not the profile could quick-play it -- the script's
    /// protocol carries the name, and `process_minecraft_args` is what decides
    /// whether an argument that uses it exists.
    #[test]
    fn a_server_target_is_two_script_lines_and_a_world_target_is_one() {
        let p = profile();
        let server = LaunchTarget {
            address: "mc.example.com".into(),
            port: 25570,
            world: String::new(),
        };
        let script = create_launch_script(&p, None, Some(&server), &[], window(), "B", "1", "I");
        assert!(script.contains("serverAddress mc.example.com\nserverPort 25570\n"));
        assert!(!script.contains("worldName"));

        let world = LaunchTarget {
            address: String::new(),
            port: 0,
            world: "Harbour".into(),
        };
        let script = create_launch_script(&p, None, Some(&world), &[], window(), "B", "1", "I");
        assert!(script.contains("worldName Harbour\n"));
        assert!(!script.contains("serverAddress"));
        assert!(!script.contains("serverPort"));
    }

    /// The traits are written in the set's own (sorted) order, and
    /// `legacyServices` adds the online-fixes line the launcher reads.
    #[test]
    fn the_script_writes_its_traits_sorted_and_its_online_fixes() {
        let mut p = profile();
        p.traits = BTreeSet::from([
            "zeta".to_string(),
            "alpha".to_string(),
            "legacyServices".to_string(),
        ]);
        let script = create_launch_script(&p, None, None, &[], window(), "B", "1", "I");
        assert!(script.contains("traits alpha\ntraits legacyServices\ntraits zeta\n"));
        assert!(script.contains("onlineFixes true\n"));
        // without the trait the line is absent rather than false
        assert!(
            !create_launch_script(&profile(), None, None, &[], window(), "B", "1", "I")
                .contains("onlineFixes")
        );
    }

    /// A maximised window is only spelled `maximized` for a legacy profile; a
    /// modern one keeps its size. `alphaLaunch` is the other legacy trait.
    #[test]
    fn a_maximised_window_keeps_its_size_unless_the_profile_is_legacy() {
        let maximised = WindowParams {
            width: 1280,
            height: 720,
            maximized: true,
        };
        let script = create_launch_script(&profile(), None, None, &[], maximised, "B", "1", "I");
        assert!(script.contains("windowParams 1280x720\n"));

        let mut legacy = profile();
        legacy.traits.insert("alphaLaunch".into());
        let script = create_launch_script(&legacy, None, None, &[], maximised, "B", "1", "I");
        assert!(script.contains("windowParams maximized\n"));
        assert!(script.contains("launcher legacy\n"));
    }

    /// No session means no user lines at all; a session with empty strings writes
    /// both lines empty, because the protocol has no way to say "absent".
    #[test]
    fn a_script_without_a_session_names_no_user() {
        let script = create_launch_script(&profile(), None, None, &[], window(), "B", "1", "I");
        assert!(!script.contains("userName"));
        assert!(!script.contains("sessionId"));

        let script = create_launch_script(
            &profile(),
            Some(&AuthSession::default()),
            None,
            &[],
            window(),
            "B",
            "1",
            "I",
        );
        assert!(script.contains("userName \n"));
        assert!(script.contains("sessionId \n"));
    }

    /// The exported variables are the six Prism writes plus `NO_COLOR`, keyed by
    /// the names a launch script's `$INST_*` references read.
    #[test]
    fn environment_variables_name_the_directories_and_the_java() {
        let v = instance_env_vars(
            "Name",
            "id",
            Path::new("/i"),
            Path::new("/i/minecraft"),
            Path::new("/usr/bin/java"),
            "-Xmx1G",
        );
        assert_eq!(v["INST_DIR"], "/i");
        assert_eq!(v["INST_MC_DIR"], "/i/minecraft");
        assert_eq!(v["INST_JAVA"], "/usr/bin/java");
        assert_eq!(
            v.keys().map(String::as_str).collect::<Vec<_>>(),
            [
                "INST_DIR",
                "INST_ID",
                "INST_JAVA",
                "INST_JAVA_ARGS",
                "INST_MC_DIR",
                "INST_NAME",
                "NO_COLOR",
            ]
        );
    }
}
