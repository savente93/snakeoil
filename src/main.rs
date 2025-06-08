use color_eyre::eyre::Result;
use snakedown::{
    render::{
        SSG,
        formats::{Renderer, md::MdRenderer, zola::ZolaRenderer},
    },
    render_docs,
};
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

    let renderer: Box<dyn Renderer> = match args.ssg {
        SSG::Markdown => Box::new(MdRenderer::new()),
        SSG::Zola => Box::new(ZolaRenderer::new()),
    };

    render_docs(
        &args.pkg_path,
        &args.output_dir,
        args.skip_private,
        args.skip_undoc,
        args.exclude,
        &renderer,
    )?;

    Ok(())
}
