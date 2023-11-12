#![allow(unused)]

use std::fs;
use std::io;

use tree_sitter;
use tree_sitter::Node;
use tree_sitter_graph;
use tree_sitter_nix;

fn move_node_by_list<'a>(node: &'a Node, idxs: &[usize]) -> Node<'a> {
    return idxs
        .into_iter()
        .fold(*node, (|n, i| (n.child(*i).unwrap())));
}

fn main() {
    let lang = tree_sitter_nix::language();
    let mut s = tree_sitter::Parser::new();
    s.set_language(lang);
    let text = fs::read_to_string("/home/heinwol/nixos-config/home-modules/fish.nix").unwrap();
    let tree = s.parse(&text, None).unwrap();
    // tree_sitter_graph
    // let mut f = io::BufWriter
    let mut f = fs::File::create("/home/heinwol/temp/update-nix-fetchers/temp.dot").unwrap();
    tree.print_dot_graph(&f);
    let cursor = tree.walk();
    let source: String = "lala".into();
    let exp = move_node_by_list(&cursor.node(), &[0, 2, 1, 0, 2, 1, 0, 0, 0]).to_sexp();
    println!("{:#?}", exp);
}
