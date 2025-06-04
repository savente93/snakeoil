mod plain;

use plain::render_plain_front_matter;

pub struct FrontMatter {
    pub title: Option<String>,
}

pub enum FrontMatterFormat {
    PlainMarkdown,
}

pub fn render_front_matter(title: &Option<String>, format: FrontMatterFormat) -> String {
    match format {
        FrontMatterFormat::PlainMarkdown => render_plain_front_matter(title),
    }
}
