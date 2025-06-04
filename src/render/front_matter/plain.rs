pub fn render_plain_front_matter(title: &Option<String>) -> String {
    match title {
        Some(t) => format!("# {}\n", t),
        None => String::new(),
    }
}
