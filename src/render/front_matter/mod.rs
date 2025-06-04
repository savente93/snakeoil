mod plain;
mod zola;

use clap::ValueEnum;
use plain::render_plain_front_matter;
use strum::Display;
use zola::render_zola_front_matter;

pub struct FrontMatter {
    pub title: Option<String>,
}

#[derive(Clone, Copy, Debug, Display, ValueEnum, PartialEq, Eq)]
pub enum FrontMatterFormat {
    Markdown,
    Zola,
}

pub fn render_front_matter(title: &Option<String>, format: FrontMatterFormat) -> String {
    match format {
        FrontMatterFormat::Markdown => render_plain_front_matter(title),
        FrontMatterFormat::Zola => render_zola_front_matter(title),
    }
}
