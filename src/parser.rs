use crate::{guild::*, Token, TokenKind};

fn file(p: &mut Parser, node: &mut File) {
    while !p.is_eof() {
        strukt(p, &mut node.strukt());
    }
}

fn strukt(p: &mut Parser, strukt: &mut Struct) {
    strukt.struct_token(p.expect_keyword("struct"));
    strukt.name(p.expect(TokenKind::IDENT));
    strukt.l_curly_token(p.expect(TokenKind::L_CURLY));
    while !p.is_eof() && !p.at(TokenKind::R_CURLY) {
        field(p, &mut strukt.field());
    }
    strukt.r_curly_token(p.expect(TokenKind::R_CURLY));
}

fn field(p: &mut Parser, field: &mut Field) {
    field.name(p.expect(TokenKind::IDENT));
    field.colon_token(p.expect(TokenKind::COLON));
    field.ty(p.expect(TokenKind::IDENT));
    if let Some(token) = p.eat(TokenKind::COMMA) {
        field.colon_token(Some(token));
    }
}

struct Parser {
    f: i32,
}

impl Parser {
    fn is_eof(&self) -> bool {
        false
    }
    fn at(&self, kind: TokenKind) -> bool {
        false
    }

    fn eat(&mut self, kind: TokenKind) -> Option<Token> {
        None
    }
    fn expect_keyword(&mut self, kw: &str) -> Option<Token> {
        None
    }

    fn expect(&mut self, kind: TokenKind) -> Option<Token> {
        None
    }
}
