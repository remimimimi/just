use crate::common::*;

#[derive(Debug, PartialEq, Clone, Copy, Ord, PartialOrd, Eq)]
pub(crate) enum TokenKind {
    Asterisk,
    At,
    Backtick,
    BangEquals,
    BraceL,
    BraceR,
    BracketL,
    BracketR,
    Colon,
    ColonEquals,
    Comma,
    Comment,
    Dedent,
    Dollar,
    Eof,
    Eol,
    Equals,
    EqualsEquals,
    Identifier,
    Indent,
    InterpolationEnd,
    InterpolationStart,
    ParenL,
    ParenR,
    Plus,
    StringToken,
    Text,
    Unspecified,
    Whitespace,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        use TokenKind::*;
        write!(
            f,
            "{}",
            match *self {
                Asterisk => "'*'",
                At => "'@'",
                Backtick => "backtick",
                BangEquals => "'!='",
                BraceL => "'{'",
                BraceR => "'}'",
                BracketL => "'['",
                BracketR => "']'",
                Colon => "':'",
                ColonEquals => "':='",
                Comma => "','",
                Comment => "comment",
                Dedent => "dedent",
                Dollar => "'$'",
                Eof => "end of file",
                Eol => "end of line",
                Equals => "'='",
                EqualsEquals => "'=='",
                Identifier => "identifier",
                Indent => "indent",
                InterpolationEnd => "'}}'",
                InterpolationStart => "'{{'",
                ParenL => "'('",
                ParenR => "')'",
                Plus => "'+'",
                StringToken => "string",
                Text => "command text",
                Unspecified => "unspecified",
                Whitespace => "whitespace",
            }
        )
    }
}
