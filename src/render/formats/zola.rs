use crate::render::formats::Renderer;

pub struct ZolaRenderer {}
impl Default for ZolaRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl ZolaRenderer {
    pub fn new() -> Self {
        Self {}
    }
}
impl Renderer for ZolaRenderer {
    fn render_header(&self, content: &str, level: usize) -> String {
        let mut out = String::new();
        out.push_str(&"#".repeat(level));
        out.push(' ');
        out.push_str(content);
        out.push('\n');
        out
    }

    fn render_front_matter(&self, title: Option<&str>) -> String {
        let mut out = String::new();
        out.push_str("+++\n");
        if let Some(t) = title {
            out.push_str(&format!("title = \"{}\"\n", t));
        };
        out.push_str("+++\n");
        out
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use color_eyre::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_zola_header() -> Result<()> {
        let renderer = ZolaRenderer::new();
        let obj_name = String::from("foo.bar.nasty-names_with_underscores_and_emoji_🙈");
        assert_eq!(
            renderer.render_header(&obj_name, 2),
            "## foo.bar.nasty-names_with_underscores_and_emoji_🙈\n"
        );
        Ok(())
    }

    #[test]
    fn test_empty_zola_front_matter() -> Result<()> {
        assert_eq!(
            ZolaRenderer::new().render_front_matter(None),
            r"+++
+++
"
        );
        Ok(())
    }

    #[test]
    fn test_zola_front_matter_with_title() -> Result<()> {
        assert_eq!(
            ZolaRenderer::new().render_front_matter(Some("foo")),
            r#"+++
title = "foo"
+++
"#
        );
        Ok(())
    }
}
