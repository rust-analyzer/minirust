mod guild;
mod parser;

enum TokenKind {
    IDENT,
    L_CURLY,
    R_CURLY,
    COLON,
    COMMA,
}

struct Token {
    kind: TokenKind,
}
