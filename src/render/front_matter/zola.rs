pub fn render_zola_front_matter(title: &Option<String>) -> String {
    let mut out = String::new();
    out.push_str("+++\n");
    if let Some(t) = title {
        out.push_str(&format!("title = \"{}\"\n", t));
    };
    out.push_str("+++\n");
    out
}

pub fn render_zola_header(content: String, header_level: usize) -> String {
    let anchor = render_zola_anchor(&content);
    format!("{} {} {}", "#".repeat(header_level), &content, anchor)
}

pub fn render_zola_link(content: String) -> String {
    let anchor = render_zola_anchor_id(&content);
    format!("[{}](#{})", &content, anchor)
}

pub fn render_zola_anchor(content: &str) -> String {
    format!("{{#{}}}", render_zola_anchor_id(content))
}
pub fn render_zola_anchor_id(content: &str) -> String {
    content
        .to_ascii_lowercase()
        .replace(".", "")
        .replace("-", "")
}

#[cfg(test)]
mod test {
    use super::*;
    use color_eyre::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_zola_header() -> Result<()> {
        assert_eq!(
            render_zola_header(String::from("foo.bar.nasty-names"), 2),
            r"## foo.bar.nasty-names {#foobarnastynames}"
        );
        Ok(())
    }
    #[test]
    fn test_zola_link() -> Result<()> {
        assert_eq!(
            render_zola_link(String::from("foo.bar.nasty-names")),
            r"[foo.bar.nasty-names](#foobarnastynames)"
        );
        Ok(())
    }

    #[test]
    fn test_empty_zola_front_matter() -> Result<()> {
        assert_eq!(
            render_zola_front_matter(&None),
            r"+++
+++
"
        );
        Ok(())
    }

    #[test]
    fn test_zola_front_matter_with_title() -> Result<()> {
        assert_eq!(
            render_zola_front_matter(&Some("foo".to_string())),
            r#"+++
title = "foo"
+++
"#
        );
        Ok(())
    }
}
