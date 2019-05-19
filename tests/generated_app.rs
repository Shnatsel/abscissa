//! Generate an Abscissa application using the `abscissa` CLI tool and run
//! tests against it (also `clippy`, `rustfmt`).

use std::{env, ffi::OsStr, fs, path::Path, process::Command};
use tempfile::TempDir;

/// Name of our test application
const APP_NAME: &str = "generated_test_app";

/// Cargo commands to run against the generated application
const CARGO_COMMANDS: &[&str] = &[
    "fmt -- --check",
    "test --release",
    "run -- --version",
    "clippy",
];

/// Run tests against the generated application
#[test]
fn test_generated_app() {
    let tmp = TempDir::new().unwrap();
    let app_path = tmp.path().join(APP_NAME);

    generate_app(&app_path);
    assert!(env::set_current_dir(&app_path).is_ok());

    Command::new("cat").arg("Cargo.toml").status().unwrap();

    for test_command in CARGO_COMMANDS {
        run_cargo(test_command.split(" "));
    }
}

/// Generate the app
fn generate_app(path: &Path) {
    let cwd = env::current_dir().unwrap();
    let abscissa_crate_patch = format!("abscissa = {{ path = '{}' }}", cwd.display());

    run_cargo(&[
        "run",
        "--release",
        "--",
        "new",
        &path.display().to_string(),
        "--patch-crates-io",
        &abscissa_crate_patch,
    ]);

    let app_test_dir = path.join("tests");

    // Copy supplemental application tests into the newly generated application
    fs::create_dir_all(&app_test_dir).unwrap();

    for entry in fs::read_dir("tests/app").unwrap() {
        let test_file = entry.unwrap().path();
        fs::copy(
            &test_file,
            app_test_dir.join(&test_file.file_name().unwrap()),
        )
        .unwrap();
    }
}

/// Run the `cargo` command with the given arguments
fn run_cargo<I, S>(args: I)
where
    I: IntoIterator<Item = S> + Clone,
    S: AsRef<OsStr>,
{
    // Display the cargo command we're executing before we run it
    assert!(Command::new("echo")
        .arg("+ cargo")
        .args(args.clone())
        .status()
        .unwrap()
        .success());

    let status = Command::new("cargo").args(args).status().unwrap();
    let status_code = status.code().unwrap();

    assert_eq!(
        status_code, 0,
        "cargo exited with error status: {}",
        status_code
    );
}
