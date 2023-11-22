#![allow(unused)]

use std::{fs, io};

use tree_sitter::{self, Node, QueryMatches};
use tree_sitter_graph;
use tree_sitter_nix;

fn move_node_by_list<'a>(node: &Node<'a>, idxs: &[usize]) -> Node<'a> {
    return idxs
        .into_iter()
        .fold(*node, (|n, i| (n.child(*i).unwrap())));
}

fn main() {
    let lang = tree_sitter_nix::language();
    let mut s = tree_sitter::Parser::new();
    s.set_language(lang);
    let text =
        fs::read_to_string("/home/heinwol/Documents/work/projects/update-nix-fetchers/test.nix")
            .unwrap();
    let text_str = text.as_bytes();
    let tree = s.parse(&text, None).unwrap();
    // tree_sitter_graph
    // let mut f = io::BufWriter
    let mut f =
        fs::File::create("/home/heinwol/Documents/work/projects/update-nix-fetchers/temp.dot")
            .unwrap();
    tree.print_dot_graph(&f);
    let cursor = tree.walk();
    // let exp = move_node_by_list(&cursor.node(), &[0, 2, 1, 0, 2, 1, 0, 0, 0]);
    let top = &cursor.node();

    // that's literal "source_plugin"
    let exp = move_node_by_list(top, &[0]);
    // let it = exp.field_name_for_child(0).unwrap();

    let q = tree_sitter::Query::new(lang, "((identifier) @start (#eq? @start \"x\"))").unwrap();
    let mut qc = tree_sitter::QueryCursor::new();
    // qc.
    let mtch = qc.matches(&q, exp, text_str);
    let mtchvec: Vec<tree_sitter::QueryMatch> = mtch.collect();
    // println!("{:#?}", top);
    println!("{:#?}", mtchvec);
}
