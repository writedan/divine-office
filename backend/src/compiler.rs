use crate::parser::Expr;
use crate::preprocessor::Preprocessor;

#[derive(serde::Serialize, Debug, PartialEq)]
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

pub fn compile_exprs(exprs: Vec<Expr>) -> Vec<Element> {
    let preprocessor = Preprocessor::new();
    let preprocessed = preprocessor.preprocess(exprs);

    let mut out = Vec::new();
    for item in preprocessed {
        match item {
            Ok(expr) => out.push(compile_expr(expr)),
            Err(why) => out.push(Element::Error(why)),
        }
    }
    out
}

fn compile_expr(expr: Expr) -> Element {
    match expr {
        Expr::String(s) => Element::Text(s),
        Expr::List(vals) => compile_list(vals),
        Expr::Symbol(s) => Element::Error(format!("Unexpected symbol: {:?}", s)),
        _ => Element::Error(format!("Unexpected expression: {:?}", expr)),
    }
}

fn compile_list(exprs: Vec<Expr>) -> Element {
    if exprs.is_empty() {
        return Element::Empty;
    }

    let head = &exprs[0];

    match head {
        Expr::Symbol(sym) => match sym.as_str() {
            "instr"    => compile_instr(&exprs[1..]),
            "box"      => compile_box(&exprs[1..]),
            "heading"  => compile_heading(&exprs[1..]),
            "title"    => compile_title(&exprs[1..]),
            "raw-gabc" => compile_raw_gabc(&exprs[1..]),
            _ => Element::Error(format!("Unknown function: {}", sym)),
        },

        other => Element::Error(format!(
            "List must begin with a symbol, got {:?}",
            other
        )),
    }
}

fn compile_instr(args: &[Expr]) -> Element { if args.len() != 1 { return Element::Error(format!("instr expects 1 argument, got {:?}", args)); } match &args[0] { Expr::String(s) => Element::Instruction(s.clone()), other => Element::Error(format!("instr: expected string, got {:?}", other)), } }

fn compile_box(args: &[Expr]) -> Element {
    let mut items = Vec::new();
    for a in args {
        items.push(compile_expr(a.clone()));
    }
    Element::Box(items)
}

fn compile_heading(args: &[Expr]) -> Element {
    // TODO: implement fully
    if args.len() != 2 {
        return Element::Error("heading expects (text level)".into());
    }
    let text = match &args[0] {
        Expr::String(s) => s.clone(),
        other => return Element::Error(format!("heading: expected text, got {:?}", other)),
    };
    let level = match &args[1] {
        Expr::Number(n) => *n as u8,
        other => return Element::Error(format!("heading: expected integer, got {:?}", other)),
    };
    Element::Heading(text, level)
}

fn compile_title(args: &[Expr]) -> Element {
    if args.len() != 1 {
        return Element::Error("title expects 1 argument".into());
    }
    match &args[0] {
        Expr::String(s) => Element::Title(s.clone()),
        other => Element::Error(format!("title: expected string, got {:?}", other)),
    }
}

fn compile_raw_gabc(args: &[Expr]) -> Element {
    if args.len() != 1 {
        return Element::Error("raw-gabc expects 1 argument".into());
    }
    match &args[0] {
        Expr::String(s) => Element::RawGabc(s.clone()),
        other => Element::Error(format!("raw-gabc: expected string, got {:?}", other)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn run(text: &str) -> Vec<Element> {
        let mut lexer = Lexer::from_str(text);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let exprs = parser.parse().unwrap();
        compile_exprs(exprs)
    }

    #[test]
    fn test_compile_instr_string_literal() {
        let result = run("(instr \"hello\")");
        assert_eq!(result, vec![Element::Instruction("hello".to_string())]);
    }

    #[test]
    fn test_compile_instr_symbol_resolves_via_preprocessor() {
        let result = run("(instr test-var)");
        assert_eq!(
            result,
            vec![Element::Instruction("This is a test variable.".into())]
        );
    }

    #[test]
    fn test_compile_instr_wrong_arg_count() {
        let result = run("(instr \"a\" \"b\")");
        assert!(matches!(result[0], Element::Error(_)));
    }

    #[test]
    fn test_compile_title() {
        let result = run("(title \"My Title\")");
        assert_eq!(result, vec![Element::Title("My Title".into())]);
    }

    #[test]
    fn test_compile_title_error_non_string() {
        let result = run("(title 123)");
        assert!(matches!(result[0], Element::Error(_)));
    }

    #[test]
    fn test_compile_heading() {
        let result = run("(heading \"Chapter 1\" 2)");
        assert_eq!(result, vec![Element::Heading("Chapter 1".into(), 2)]);
    }

    #[test]
    fn test_compile_heading_wrong_types() {
        let result = run("(heading 123 2)");
        assert!(matches!(result[0], Element::Error(_)));
    }

    #[test]
    fn test_compile_box() {
        let result = run("(box \"a\" \"b\" \"c\" (instr \"test\"))");
        assert_eq!(
            result,
            vec![Element::Box(vec![
                Element::Text("a".into()),
                Element::Text("b".into()),
                Element::Text("c".into()),
                Element::Instruction("test".into())
            ])]
        );
    }

    #[test]
    fn test_compile_box_nested() {
        let result = run("(box (title \"X\") (instr \"y\"))");
        assert_eq!(
            result,
            vec![Element::Box(vec![
                Element::Title("X".into()),
                Element::Instruction("y".into())
            ])]
        );
    }

    #[test]
    fn test_compile_raw_gabc() {
        let result = run(r#"(raw-gabc "c4 d e f")"#);
        assert_eq!(result, vec![Element::RawGabc("c4 d e f".into())]);
    }

    #[test]
    fn test_unknown_function() {
        let result = run("(florp \"x\")");
        assert!(matches!(result[0], Element::Error(_)));
    }

    #[test]
    fn test_preprocessor_expands_inside_box() {
        let result = run("(box test-var)");
        assert_eq!(
            result,
            vec![Element::Box(vec![Element::Text(
                "This is a test variable.".into()
            )])]
        );
    }
}
