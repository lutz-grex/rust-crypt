pub fn format_error<E: std::fmt::Display>(context: &str, error: E) -> Box<dyn std::error::Error> {
    format!("{}: {}", context, error).into()
}
