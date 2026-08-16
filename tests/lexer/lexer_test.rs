use lewekk_head::LexerManager;
use lexer::CargryLexer;

#[test]
fn scope_and_paren_test() {
    let mut lex = CargryLexer::new();
    lex.run("{ ( {} ) }");
    let result = lex
        .get_tokens()
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();
    assert_eq!(result, vec!["{", "(", "{", "}", ")", "}"]);
}
