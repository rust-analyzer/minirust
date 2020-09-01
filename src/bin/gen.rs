use ungrammar::Grammar;

fn main() {
    let g = minirust_grammar();
    eprintln!("{:#?}", g);
}

fn minirust_grammar() -> Grammar {
    let src = include_str!("../minirust.ungram");
    src.parse().unwrap()
}
