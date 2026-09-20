use lewekk_head::LexerManager;
use lexer::lexer::{
    CargryLexer, CargryTokens, token_lparen, token_lscope, token_rparen, token_rscope,
};

#[test]
fn scope_and_paren_test() {
    let mut lex = CargryLexer::new();
    let r = lex.run("{ ( {} ) }");
    let result = lex
        .get_rules()
        .iter()
        .map(|(x, _)| x.clone())
        .collect::<Vec<CargryTokens>>();
    assert!(r.is_ok());
    assert_eq!(
        result,
        vec![
            token_lscope(),
            token_lparen(),
            token_lscope(),
            token_rscope(),
            token_rparen(),
            token_rscope()
        ]
    );
}
