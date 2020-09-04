mod guild;
mod parser;
mod ast;
mod cst;

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
