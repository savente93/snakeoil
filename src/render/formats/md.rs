use crate::render::formats::Renderer;

#[derive(Default)]
pub struct MdRenderer {}
impl MdRenderer {
    pub fn new() -> Self {
        Self {}
    }
}
impl Renderer for MdRenderer {
    fn render_header(&self, content: &str, level: usize) -> String {
        let mut out = String::new();
        out.push_str(&"#".repeat(level));
        out.push(' ');
        out.push_str(content);
        out.push('\n');
        out
    }

    fn render_front_matter(&self, title: Option<&str>) -> String {
        if let Some(t) = title {
            self.render_header(t, 1)
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod test {
    use color_eyre::Result;

    use crate::render::formats::{Renderer, md::MdRenderer};
    #[test]
    fn test_render_md_header() -> Result<()> {
        let text = String::from("foo");
        let out = MdRenderer::new().render_header(&text, 1);
        assert_eq!(out, String::from("# foo\n"));
        Ok(())
    }
}
