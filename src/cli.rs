use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{LogLevel, Verbosity, VerbosityFilter};

#[allow(dead_code)]
pub struct CustomLogLevel {}

impl LogLevel for CustomLogLevel {
    fn default_filter() -> VerbosityFilter {
        VerbosityFilter::Error
    }
    fn quiet_help() -> Option<&'static str> {
        Some("suppress all logging output")
    }
    fn quiet_long_help() -> Option<&'static str> {
        Some("Suppress the logging output of the application, including errors.")
    }
    fn verbose_help() -> Option<&'static str> {
        Some("Increase verbosity of the logging (can be specified multiple times).")
    }
    fn verbose_long_help() -> Option<&'static str> {
        Some(
            "Increase the logging verbosity of the application by one level (ERROR, WARN, INFO, DEBUG, TRACE)",
        )
    }
}

#[derive(Parser)]
#[command(version, about, long_about= None)]
pub struct Args {
    #[command(flatten)]
    pub verbose: Verbosity,

    #[arg(default_value = ".")]
    pub pkg_path: PathBuf,

    #[arg(default_value = "_build")]
    pub output_dir: PathBuf,

    /// Do not render undocumented functions, classes or modules
    #[arg(long, default_value_t = false)]
    pub skip_undoc: bool,

    /// Do not render private functions, classes or modules (who's name start with _)
    #[arg(long, default_value_t = false)]
    pub skip_private: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use clap_verbosity_flag::log::Level;
    use color_eyre::Result;

    #[test]
    fn test_custom_log_level_interface() -> Result<()> {
        // Explicitly call each method to ensure they are covered
        assert_eq!(CustomLogLevel::default_filter(), VerbosityFilter::Error);
        assert_eq!(
            CustomLogLevel::quiet_help(),
            Some("suppress all logging output")
        );
        assert_eq!(
            CustomLogLevel::quiet_long_help(),
            Some("Suppress the logging output of the application, including errors.")
        );
        assert_eq!(
            CustomLogLevel::verbose_help(),
            Some("Increase verbosity of the logging (can be specified multiple times).")
        );
        assert_eq!(
            CustomLogLevel::verbose_long_help(),
            Some(
                "Increase the logging verbosity of the application by one level (ERROR, WARN, INFO, DEBUG, TRACE)"
            )
        );
        Ok(())
    }

    #[test]
    fn test_args_defaults() -> Result<()> {
        let args = Args::parse_from(["mybin"]);
        assert_eq!(args.pkg_path, PathBuf::from("."));
        assert_eq!(args.output_dir, PathBuf::from("_build"));
        assert!(!args.skip_undoc);
        assert!(!args.skip_private);
        assert!(args.exclude.is_empty());
        Ok(())
    }

    #[test]
    fn test_args_all_flags() -> Result<()> {
        let args = Args::parse_from([
            "snakeoil",
            "src/pkg",
            "dist",
            "--skip-undoc",
            "--skip-private",
            "--exclude",
            "path/to/exclude1",
            "--exclude",
            "path/to/exclude2",
            "-v",
            "-v",
        ]);
        assert_eq!(args.pkg_path, PathBuf::from("src/pkg"));
        assert_eq!(args.output_dir, PathBuf::from("dist"));
        assert!(args.skip_undoc);
        assert!(args.skip_private);
        assert_eq!(
            args.exclude,
            vec![
                PathBuf::from("path/to/exclude1"),
                PathBuf::from("path/to/exclude2")
            ]
        );
        // Verbosity should be INFO with -v -v (test indirectly)
        let level = args.verbose.log_level();
        assert_eq!(level, Some(Level::Info));
        Ok(())
    }

    #[test]
    fn test_args_exclude_short_flag() -> Result<()> {
        let args = Args::parse_from(["mybin", "-e", "excluded"]);
        assert_eq!(args.exclude, vec![PathBuf::from("excluded")]);
        Ok(())
    }
}
