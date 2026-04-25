pub fn lex(stdout: &str) -> Vec<&str> {
    stdout.split_whitespace().collect()
}
