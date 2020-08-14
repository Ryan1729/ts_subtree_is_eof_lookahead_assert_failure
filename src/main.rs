use tree_sitter::*;

extern "C" { fn tree_sitter_rust() -> Language; }

fn main() {
    let rust_lang = unsafe { tree_sitter_rust() };

    let mut parser = Parser::new();

    parser.set_language(rust_lang).unwrap();

    let to_parse = "";

    let mut tree = None;

    tree = parser.parse(to_parse, tree.as_ref());

    parser.parse(to_parse, tree.as_ref());
}
