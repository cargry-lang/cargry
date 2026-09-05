use lewekk_head::LexerManager;
use lexer::lexer::CargryLexer;

#[test]
fn scope_and_paren_test() {
    let mut lex = CargryLexer::new();
    let r = lex.run("{ ( {} ) }");
    let result = lex
        .get_tokens()
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();
    assert!(r.is_ok());
    assert_eq!(result, vec!["{", "(", "{", "}", ")", "}"]);
}
