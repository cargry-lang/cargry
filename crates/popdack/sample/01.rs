use lewekk::Lexer;
use lewekk_head::{LexRule, LexerManager};
use lewekk_utils::{lstring, lign};
use lewekk_macro::{lexer, tokens};

#[lexer(Token)]
struct A;
impl LexRule<Token> for A {
    fn lparse(&self, input: &String) -> Result<(String, Vec<String>), String> {
        let f = lstring("a", lign());
        f(input)
    }
}

#[tokens(A)]
enum Token {}

struct MyLexer {
    lexer: Lexer<Token>,
}
impl LexerManager<Token> for MyLexer {
    fn new() -> Self {
        let mut lex = Lexer::new(None);
        lex.add_rule(A);
        Self { lexer: lex }
    }

    fn run(&mut self, input: &str) -> Result<(), String> {
        self.lexer.run(input)
    }

    fn get_tokens(&self) -> &Vec<String> {
        self.lexer.get_tokens()
    }

    fn get_rules(&self) -> &Vec<(Token, usize)> {
        self.lexer.get_rules()
    }
}

enum Node {
    A(Option<RefN>),
}

fn tokenize()

fn parse(tokens: &Vec<Tokens>)
