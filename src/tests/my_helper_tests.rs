use super::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn setup_script_file(script_contents: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let script_path = std::env::temp_dir().join(format!("git-completer-{unique}.sh"));

    fs::write(&script_path, script_contents).unwrap();
    let mut perms = fs::metadata(&script_path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms).unwrap();

    script_path
}

#[test]
fn custom_git_completion_returns_expected_candidates() {
    let script_path = setup_script_file("#!/usr/bin/env sh\nprintf '%s\\n' add commit push\n");

    let mut state = ShellState::new();
    state.add_completion_script("git", script_path.clone());
    let helper = MyHelper {
        commands: vec![],
        state: Rc::new(RefCell::new(state)),
    };

    let (_, pairs) = helper.complete_inner("git ", 4).unwrap();
    let values: Vec<String> = pairs.into_iter().map(|p: Pair| p.replacement).collect();

    assert_eq!(
        values,
        vec![
            "add ".to_string(),
            "commit ".to_string(),
            "push ".to_string()
        ]
    );

    let _ = fs::remove_file(script_path);
}

#[test]
fn custom_git_completion_returns_subsequent_matches() {
    let script_path = setup_script_file("#!/usr/bin/env sh\nprintf '%s\\n' add append push\n");

    let mut state = ShellState::new();
    state.add_completion_script("git", script_path.clone());
    let helper = MyHelper {
        commands: vec![],
        state: Rc::new(RefCell::new(state)),
    };

    let (_, pairs) = helper.complete_inner("git a", 5).unwrap();
    let values: Vec<String> = pairs.into_iter().map(|p: Pair| p.replacement).collect();

    assert_eq!(values, vec!["add ".to_string(), "append ".to_string()]);

    let _ = fs::remove_file(script_path);
}
