use plex::lexer;
use crate::token::Token;


lexer! {
    fn next_token(text: 'a) -> Token;

    r#"[ \t\r\n]+"# => Token::Whitespace,
    // "C-style" comments (/* .. */) - can't contain "*/"
    r#"/[*](~(.*[*]/.*))[*]/"# => Token::Comment,
    // "C++-style" comments (// ...)
    r#"//[^\n]*"# => Token::Comment,

    r#"print"# => Token::Print,
    r#"typeof"# => Token::Typeof,

    r#"[0-9]+"# => {
        if let Ok(i) = text.parse() {
            Token::Integer(i)
        } else {
            panic!("integer {} is out of range", text)
        }
    }

    r#"\["# => Token::LeftBracket,
    r#"\]"# => Token::RightBracket,

    r#"\{"# => Token::LeftBrace,
    r#"\}"# => Token::RightBrace,

    r#"true"# => Token::True,
    r#"false"# => Token::False,

    r#","# => Token::Comma,
    
    r#"\".*\""# => 
        Token::String(text[1..(text.len() - 1)].to_owned()),
    r#"env"# => Token::Env,

    r#"[a-zA-Z_][a-zA-Z0-9_]*"# => Token::Ident(text.to_owned()),

    r#"="# => Token::Equals,
    r#"\+"# => Token::Plus,
    r#"-"# => Token::Minus,
    r#"\*"# => Token::Star,
    r#"/"# => Token::Slash,
    r#"\("# => Token::LeftParen,
    r#"\)"# => Token::RightParen,
    r#";"# => Token::Semi,
    r#"!"# => Token::Bang,
    
    r#">"# => Token::Gt,
    r#"<"# => Token::Lt,
    
    r#">="# => Token::Ge,
    r#"<="# => Token::Le,
    
    r#"=="# => Token::Eq,
    r#"!="# => Token::Ne,

    r#"\."# => Token::Dot,

    r#"."# => panic!("unexpected character: {}", text),
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer<'a> {
        Lexer {
            original: s,
            remaining: s,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span(pub usize, pub usize);

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = next_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, Span(lo, hi))
            } else {
                return None;
            };
            match tok {
                Token::Whitespace | Token::Comment => {
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}