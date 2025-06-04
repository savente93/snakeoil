pub fn render_zola_front_matter(title: &Option<String>) -> String {
    let mut out = String::new();
    out.push_str("+++\n");
    if let Some(t) = title {
        out.push_str(&format!("title = \"{}\"\n", t));
    };
    out.push_str("+++\n");
    out
}

#[cfg(test)]
mod test {
    use super::*;
    use color_eyre::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_empty_zola_header() -> Result<()> {
        assert_eq!(
            render_zola_front_matter(&None),
            r"+++
+++
"
        );
        Ok(())
    }

    #[test]
    fn test_zola_header_with_header() -> Result<()> {
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
