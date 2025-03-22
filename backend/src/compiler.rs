use crate::parser::ast::*;
use crate::parser::Directive;

use lazy_static::lazy_static;
use regex::Regex;

use serde::Serialize;

lazy_static! {
    static ref SmallPrint: Regex = Regex::new(r"\{([^{}]*)\}").unwrap();
    static ref Vowel: Regex = Regex::new(r"([aeiouAEIOU])").unwrap();
    static ref Y: Regex = Regex::new(r"([yY])").unwrap();
    static ref Intone: Regex = Regex::new(r"\(([^()]*)\)").unwrap();
    static ref Flex: Regex = Regex::new(r"\^([^^]*)\^").unwrap();
    static ref Mediant: Regex = Regex::new(r"\~([^~]*)\~").unwrap();
    static ref Accent: Regex = Regex::new(r"\`([^`]*)\`").unwrap();
}

#[derive(Serialize, PartialEq)]
pub enum Element {
    Box(Vec<Element>),
    Text(String),
    Heading(String, u8),
    Instruction(String),
    RawGabc(String),
    Title(String),
    Error(String),
    Empty,
}

pub fn compile_ast(tree: ASTree<Directive>) -> Vec<Element> {
    let mut res = Vec::new();

    for child in tree.children() {
        match child {
            ASTNode::Node(dir) => res.push(compile_node(dir)),
            ASTNode::Tree(tree) => {
                let tree = match tree.root {
                    Some(_) => compile_tree(tree),
                    None => compile_ast(tree),
                };

                res.extend(clear_empty(tree));
            }
        }
    }

    res
}

fn clear_empty(vec: Vec<Element>) -> Vec<Element> {
    vec.into_iter().filter(|e| e != &Element::Empty).collect()
}

fn compile_dispatch(node: ASTNode<Directive>) -> Vec<Element> {
    match node {
        ASTNode::Node(directive) => vec![compile_node(directive)],
        ASTNode::Tree(tree) => clear_empty(compile_tree(tree)),
    }
}

fn compile_tree(tree: ASTree<Directive>) -> Vec<Element> {
    match tree.root {
        Some(Directive::Box) => {
            let mut cont = Vec::new();
            for node in tree.children() {
                cont.extend(clear_empty(compile_dispatch(node)));
            }

            vec![Element::Box(clear_empty(cont))]
        }

        None => {
            let mut cont = Vec::new();
            for node in tree.children() {
                cont.extend(clear_empty(compile_dispatch(node)));
            }

            clear_empty(cont)
        }

        _ => vec![compile_node(Directive::Error(format!(
            "Unsupported tree root directive {:?}",
            tree.root
        )))],
    }
}

fn compile_node(node: Directive) -> Element {
    match node {
        Directive::Text(text) => {
            // let text = text.replace('*', "<span class='symbol'>*</span><br/>&nbsp;&nbsp;&nbsp;&nbsp;")
            // .replace("+++", "<span class='symbol'>✠</span>")
            // .replace('+', "<span class='symbol'>+</span><br/>");

            // let text = SmallPrint.replace_all(&text, "<span class='instr'>$1</span>");

            // let text = Intone.replace_all(&text, |caps: &regex::Captures| {
            // 	style_first_vowel(&caps[1], "\u{030A}", "span")
            // });

            // let text = Flex.replace_all(&text, |caps: &regex::Captures| {
            // 	style_first_vowel(&caps[1], "\u{0302}", "i")
            // });

            // let text = Mediant.replace_all(&text, |caps: &regex::Captures| {
            // 	style_first_vowel(&caps[1], "\u{0303}", "u")
            // });

            // let text = Accent.replace_all(&text, |caps: &regex::Captures| {
            // 	style_first_vowel(&caps[1], "\u{0301}", "b")
            // });

            Element::Text(text)
        }

        Directive::Heading(text, level) => Element::Heading(text, level),

        Directive::Hymn(hymn) => {
            let mut buffer = format!(
                "initial-style: 1;\nannotation: Hymn.;\ncentering-scheme: english;\n%%\n({})",
                hymn.clef
            );
            for stanza_idx in 0..(hymn.verses.len() / hymn.melody.len()) {
                for verse_idx in 0..hymn.melody.len() {
                    let verse = &hymn.verses[hymn.verse_idx(stanza_idx, verse_idx)];

                    if verse_idx == 0 && stanza_idx > 0 {
                        buffer = format!("{} (::) {}. ", buffer, stanza_idx + 1);
                    }

                    for (idx, syllable) in verse.into_iter().enumerate() {
                        let continuous = syllable.ends_with('-');
                        let syllable = if continuous {
                            syllable[0..syllable.len() - 1].to_string()
                        } else {
                            format!("{} ", syllable)
                        };
                        buffer = format!("{}{}({})", buffer, syllable, hymn.melody[verse_idx][idx]);
                    }

                    if verse_idx == hymn.melody.len() - 1 {
                        continue;
                    }

                    buffer = format!(
                        "{} {}({})",
                        buffer,
                        if stanza_idx == 0 && verse_idx == 0 {
                            "<sp>*</sp>"
                        } else {
                            ""
                        },
                        if verse_idx % 2 == 0 { "," } else { ";" }
                    );
                }
            }

            buffer = format!("{} (::) A({})men.({})", buffer, hymn.amen.0, hymn.amen.1);

            compile_node(Directive::RawGabc(buffer))
        }

        Directive::Instruction(text) => Element::Instruction(text),

        Directive::RawGabc(text) => Element::RawGabc(text),

        Directive::Title(text) => Element::Title(text),

        Directive::Error(why) => Element::Error(why),

        Directive::Empty => Element::Empty,

        _ => compile_node(Directive::Error(format!("Unsupported node {:?}", node))),
    }
}
