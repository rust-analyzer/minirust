use std::fs;
use ungrammar::{Grammar, Node, Rule};

fn main() {
    let g = minirust_grammar();
    let mut buf = String::new();
    lower(&g, &mut buf);
    fs::write("./src/guild.rs", &buf).unwrap();
}

fn minirust_grammar() -> Grammar {
    let src = include_str!("../minirust.ungram");
    src.parse().unwrap()
}

fn lower(g: &Grammar, buf: &mut String) {
    format_to!(buf, "use crate::Token;\n\n");
    for node in g.iter() {
        lower_node(buf, g, node);
    }
}

fn lower_node(buf: &mut String, g: &Grammar, node: Node) {
    let name = &g[node].name;
    format_to!(buf, "pub struct {};\n", name);

    let prod = match &g[node].rule {
        Rule::Seq(it) => it.as_slice(),
        rule => std::slice::from_ref(rule),
    };
    format_to!(buf, "impl {} {{\n", name);
    for rule in prod {
        lower_node_child(buf, g, rule);
    }
    format_to!(buf, "}}\n\n");
}

fn lower_node_child(buf: &mut String, g: &Grammar, rule: &Rule) {
    let name = match rule {
        Rule::Labeled { label, rule } => label.clone(),
        Rule::Opt(it) | Rule::Rep(it) => node_name(g, &*it).unwrap(),
        _ => node_name(g, rule).unwrap(),
    };
    let node_type = match rule {
        Rule::Labeled { rule, .. } | Rule::Opt(rule) | Rule::Rep(rule) => node_type(g, rule),
        _ => node_type(g, rule),
    };
    format_to!(buf, "    pub fn {}(&mut self", name);

    match node_type {
        Some(node) => format_to!(buf, ") -> {}", node),
        None => format_to!(buf, ", _token: Option<Token>)"),
    }
    format_to!(buf, " {{ todo!() }}\n")
}

fn node_type(g: &Grammar, rule: &Rule) -> Option<String> {
    match rule {
        Rule::Token(_) => None,
        Rule::Node(it) => Some(g[*it].name.to_string()),
        _ => todo!(),
    }
}

fn node_name(g: &Grammar, rule: &Rule) -> Option<String> {
    let res = match rule {
        Rule::Token(t) => match g[*t].name.as_str() {
            "{" => "l_curly_token".to_string(),
            "}" => "r_curly_token".to_string(),
            ":" => "colon_token".to_string(),
            "," => "comma_token".to_string(),
            "struct" => "struct_token".to_string(),
            n => todo!("n: {:?}", n),
        },
        Rule::Node(n) => match g[*n].name.as_str() {
            "Struct" => "strukt".to_string(),
            "Field" => "field".to_string(),
            _ => todo!("n: {:?}", n),
        },
        _ => return None,
    };
    Some(res)
}

macro_rules! _format_to {
    ($buf:expr) => ();
    ($buf:expr, $lit:literal $($arg:tt)*) => {
        { use ::std::fmt::Write as _; let _ = ::std::write!($buf, $lit $($arg)*); }
    };
}
use _format_to as format_to;
