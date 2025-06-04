mod plain;

use clap::ValueEnum;
use plain::render_plain_front_matter;
use strum::Display;

pub struct FrontMatter {
    pub title: Option<String>,
}

#[derive(Clone, Copy, Debug, Display, ValueEnum, PartialEq, Eq)]
pub enum FrontMatterFormat {
    Markdown,
}

pub fn render_front_matter(title: &Option<String>, format: FrontMatterFormat) -> String {
    match format {
        FrontMatterFormat::Markdown => render_plain_front_matter(title),
    }
}
