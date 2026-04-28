#[derive(Debug)]
pub enum Token {
    Muted,
    FloatValue(f32),
}

pub(crate) fn lex(stdout: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for tok in stdout.split_whitespace() {
        if let Ok(v) = tok.parse::<f32>() {
            tokens.push(Token::FloatValue(v));
        }
        if tok.contains("[MUTED]") {
            tokens.push(Token::Muted);
        }
    }

    tokens
}
