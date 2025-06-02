use color_eyre::eyre::Result;
use snakeoil::render_docs;
use tracing::subscriber::set_global_default;

mod cli;

use crate::cli::Args;
use clap::Parser;

#[allow(clippy::missing_errors_doc)]
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(args.verbose.tracing_level_filter())
        .finish();

    set_global_default(subscriber)?;

    render_docs(
        &args.pkg_path,
        &args.output_dir,
        args.skip_private,
        args.skip_undoc,
        args.exclude,
    )?;

    Ok(())
}
