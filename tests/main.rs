use assert_cmd::Command;
use color_eyre::eyre::Result;
use dir_diff::is_different;
use tempfile::tempdir;

#[test]
fn test_cli_with_all_options() -> Result<()> {
    let tempdir = tempdir()?;

    let mut cmd = Command::cargo_bin("snakeoil")?;
    cmd.arg("tests/test_pkg")
        .arg(tempdir.path())
        .arg("--skip-undoc")
        .arg("--skip-private")
        .arg("-e")
        .arg("test_pkg/excluded_file.py")
        .arg("--exclude")
        .arg("test_pkg/excluded_module")
        .arg("-vv");
    let assertion = cmd.assert();

    assertion.success();

    assert!(!is_different(tempdir.path(), "tests/rendered_no_private")?);

    Ok(())
}
