use std::ops::Range;
use core::num::ParseIntError;

use logos::{SpannedIter, Logos};

pub use self::TokenKind::*;
#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}

impl std::fmt::Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexicalError::InvalidInteger(err) => write!(f, "Invalid integer: {}", err),
            LexicalError::InvalidToken => write!(f, "Invalid token"),
        }
    }
}



#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind<'a> {
    // Skip
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,

    #[regex(r"--[^\n\f]*", logos::skip)]
    Comment,

    #[regex(r"/\*[^\+]([^\*]|(\*[^/]))*\*/", logos::skip)]
    CommentBlock,

    #[regex(r#"[_a-zA-Z][_$a-zA-Z0-9]*"#, |lex| lex.slice())]
    Ident(&'a str),

    #[regex(r#"`[^`]*`"#, |lex| lex.slice().trim_matches('`'))]
    #[regex(r#""([^"\\]|\\.|"")*""#, |lex| lex.slice().trim_matches('"'))]
    #[regex(r#"'([^'\\]|\\.|'')*'"#, |lex| lex.slice().trim_matches('\'')) ]
    QuotedString(&'a str),

    #[regex(r"\-?[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    LiteralInteger(i64),

    #[regex(r"[0-9]+[eE][+-]?[0-9]+", |lex| lex.slice())]
    #[regex(r"([0-9]*\.[0-9]+([eE][+-]?[0-9]+)?)|([0-9]+\.[0-9]*([eE][+-]?[0-9]+)?)", |lex| lex.slice())]
    LiteralFloat(&'a str),

    #[token("=")]
    Eq,

    #[token("<>")]
    #[token("!=")]
    NotEq,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,

    #[token("<=")]
    LtEq,

    #[token(">=")]
    GtEq,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("//")]
    IntDiv,

    #[token("%")]
    Modulo,

    #[token("||")]
    StringConcat,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token(";")]
    SemiColon,

    // Keywords
    #[token("AND", ignore(ascii_case))]
    AND,

    #[token("AS", ignore(ascii_case))]
    AS,

    #[token("ASC", ignore(ascii_case))]
    ASC,

    #[token("BOOLEAN", ignore(ascii_case))]
    BOOLEAN,

    #[token("BY", ignore(ascii_case))]
    BY,

    #[token("CREATE", ignore(ascii_case))]
    CREATE,

    #[token("CROSS", ignore(ascii_case))]
    CROSS,

    #[token("DELETE", ignore(ascii_case))]
    DELETE,

    #[token("DESC", ignore(ascii_case))]
    DESC,

    #[token("DISTINCT", ignore(ascii_case))]
    DISTINCT,

    #[token("DROP", ignore(ascii_case))]
    DROP,

    #[token("EXCEPT", ignore(ascii_case))]
    EXCEPT,

    #[token("EXCLUDE", ignore(ascii_case))]
    EXCLUDE,

    #[token("EXISTS", ignore(ascii_case))]
    EXISTS,

    #[token("FALSE", ignore(ascii_case))]
    FALSE,

    #[token("FLOAT", ignore(ascii_case))]
    FLOAT,

    #[token("FROM", ignore(ascii_case))]
    FROM,

    #[token("FULL", ignore(ascii_case))]
    FULL,

    #[token("GROUP", ignore(ascii_case))]
    GROUP,

    #[token("HAVING", ignore(ascii_case))]
    HAVING,

    #[token("IN", ignore(ascii_case))]
    IN,

    #[token("INNER", ignore(ascii_case))]
    INNER,

    #[token("INSERT", ignore(ascii_case))]
    INSERT,

    #[token("INT", ignore(ascii_case))]
    INT,

    #[token("INTO", ignore(ascii_case))]
    INTO,

    #[token("IS", ignore(ascii_case))]
    IS,

    #[token("JOIN", ignore(ascii_case))]
    JOIN,

    #[token("LEFT", ignore(ascii_case))]
    LEFT,

    #[token("LIKE", ignore(ascii_case))]
    LIKE,

    #[token("LIMIT", ignore(ascii_case))]
    LIMIT,

    #[token("NOT", ignore(ascii_case))]
    NOT,

    #[token("NULL", ignore(ascii_case))]
    NULL,

    #[token("OFFSET", ignore(ascii_case))]
    OFFSET,

    #[token("ON", ignore(ascii_case))]
    ON,

    #[token("OR", ignore(ascii_case))]
    OR,

    #[token("ORDER", ignore(ascii_case))]
    ORDER,

    #[token("OUTER", ignore(ascii_case))]
    OUTER,

    #[token("OVER", ignore(ascii_case))]
    OVER,

    #[token("PARTITION", ignore(ascii_case))]
    PARTITION,

    #[token("RECURSIVE", ignore(ascii_case))]
    RECURSIVE,

    #[token("RIGHT", ignore(ascii_case))]
    RIGHT,

    #[token("SELECT", ignore(ascii_case))]
    SELECT,

    #[token("SET", ignore(ascii_case))]
    SET,

    #[token("TABLE", ignore(ascii_case))]
    TABLE,

    #[token("TEXT", ignore(ascii_case))]
    TEXT,

    #[token("TRUE", ignore(ascii_case))]
    TRUE,

    #[token("UPDATE", ignore(ascii_case))]
    UPDATE,

    #[token("VALUES", ignore(ascii_case))]
    VALUES,

    #[token("WHERE", ignore(ascii_case))]
    WHERE,

    #[token("WINDOW", ignore(ascii_case))]
    WINDOW,

    #[token("WITH", ignore(ascii_case))]
    WITH,

    #[token("FIRST", ignore(ascii_case))]
    FIRST,

    #[token("LAST", ignore(ascii_case))]
    LAST,
}
impl<'a> TokenKind<'a> {
    pub fn is_literal(&self) -> bool {
        matches!(self, LiteralInteger(..) | LiteralFloat(..) | QuotedString(..))
    }

    pub fn is_keyword(&self) -> bool {
        !matches!(
            self,
            Ident(..)
                | QuotedString(..)
                | LiteralInteger(..)
                | LiteralFloat(..)
                | Eq
                | NotEq
                | Lt
                | Gt
                | LtEq
                | GtEq
                | Plus
                | Minus
                | Multiply
                | Divide
                | IntDiv
                | Modulo
                | StringConcat
                | LParen
                | RParen
                | Comma
                | Dot
        )
    }
}

impl<'a> std::fmt::Display for TokenKind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Whitespace => write!(f, "Whitespace"),
            BOOLEAN => write!(f,"BOOLEAN"),
            CREATE => write!(f,"CREATE"),
            Comment => write!(f, "Comment"),
            CommentBlock => write!(f, "CommentBlock"),
            DELETE => write!(f,"DELETE"),
            DROP => write!(f,"DROP"),
            FALSE => write!(f,"FALSE"),
            Ident(s) => write!(f, "Ident({})",s),
            QuotedString(s) => write!(f, "QuotedString({})",s),
            LiteralInteger(n) => write!(f, "LiteralInteger({})",n),
            LiteralFloat(float_n) => write!(f, "LiteralFloat({})",float_n),
            Eq => write!(f, "Eq"),
            NotEq => write!(f, "NotEq"),
            Lt => write!(f, "Lt"),
            Gt => write!(f, "Gt"),
            LtEq => write!(f, "LtEq"),
            GtEq => write!(f, "GtEq"),
            Plus => write!(f, "Plus"),
            Minus => write!(f, "Minus"),
            Multiply => write!(f, "Multiply"),
            Divide => write!(f, "Divide"),
            IntDiv => write!(f, "IntDiv"),
            Modulo => write!(f, "Modulo"),
            StringConcat => write!(f, "StringConcat"),
            LParen => write!(f, "LParen"),
            RParen => write!(f, "RParen"),
            Comma => write!(f, "Comma"),
            Dot => write!(f, "Dot"),
            SemiColon => write!(f, "SemiColon"),
            AND => write!(f, "AND"),
            AS => write!(f, "AS"),
            ASC => write!(f, "ASC"),
            BY => write!(f, "BY"),
            CROSS => write!(f, "CROSS"),
            DESC => write!(f, "DESC"),
            DISTINCT => write!(f, "DISTINCT"),
            EXCEPT => write!(f, "EXCEPT"),
            EXCLUDE => write!(f, "EXCLUDE"),
            EXISTS => write!(f, "EXISTS"),
            FROM => write!(f, "FROM"),
            FULL => write!(f, "FULL"),
            GROUP => write!(f, "GROUP"),
            HAVING => write!(f, "HAVING"),
            IN => write!(f, "IN"),
            INNER => write!(f, "INNER"),
            INTO => write!(f, "INTO"),
            IS => write!(f, "IS"),
            JOIN => write!(f, "JOIN"),
            LEFT => write!(f, "LEFT"),
            LIKE => write!(f, "LIKE"),
            LIMIT => write!(f, "LIMIT"),
            NOT => write!(f, "NOT"),
            NULL => write!(f, "NULL"),
            OFFSET => write!(f, "OFFSET"),
            ON => write!(f, "ON"),
            OR => write!(f, "OR"),
            ORDER => write!(f, "ORDER"),
            OUTER => write!(f, "OUTER"),
            OVER => write!(f, "OVER"),
            PARTITION => write!(f, "PARTITION"),
            RECURSIVE => write!(f, "RECURSIVE"),
            RIGHT => write!(f, "RIGHT"),
            SELECT => write!(f, "SELECT"),
            TABLE => write!(f, "TABLE"),
            WHERE => write!(f, "WHERE"),
            WINDOW => write!(f, "WINDOW"),
            WITH => write!(f, "WITH"),
            FIRST => write!(f, "FIRST"),
            LAST => write!(f, "LAST"),
            _ => todo!(),
        }
    }
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
  // instead of an iterator over characters, we have a token iterator
  token_stream: SpannedIter<'input, TokenKind<'input>>,
}
impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        // the Token::lexer() method is provided by the Logos trait
        Self { token_stream: TokenKind::lexer(input).spanned() }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<TokenKind<'input>, usize, LexicalError>;
  
    fn next(&mut self) -> Option<Self::Item> {
      self.token_stream
        .next()
        .map(|(token, span)| Ok((span.start, token.unwrap(), span.end)))
    }
}
  


#[cfg(test)]
mod tests {
    #[test]
    pub fn test_tokenizer() {
        use super::*;
        let sql = "Select a, t1.b, count(c) FROM t1 where a > 1 order by b limit 10, 20";
        let tokenizer = TokenKind::lexer(sql);

        let tokens: Vec<_> = tokenizer.map(|token| token.unwrap()).collect();

        let tokens: Vec<_> = tokens.into_iter().collect();
        #[rustfmt::skip]
        let expected = vec![
            TokenKind::SELECT,
            TokenKind::Ident("a"),
            TokenKind::Comma, 
            TokenKind::Ident("t1"), 
            TokenKind::Dot, 
            TokenKind::Ident("b"), 
            TokenKind::Comma, 
            TokenKind::Ident("count"), 
            TokenKind::LParen, 
            TokenKind::Ident("c"), 
            TokenKind::RParen, 
            TokenKind::FROM, 
            TokenKind::Ident("t1"), 
            TokenKind::WHERE, 
            TokenKind::Ident("a"), 
            TokenKind::Gt, 
            TokenKind::LiteralInteger(1), 
            TokenKind::ORDER, 
            TokenKind::BY, 
            TokenKind::Ident("b"), 
            TokenKind::LIMIT, 
            TokenKind::LiteralInteger(10), 
            TokenKind::Comma, 
            TokenKind::LiteralInteger(20), 
        ];

        assert_eq!(tokens, expected);
    }
}