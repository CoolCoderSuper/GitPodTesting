#[derive(Debug)]
pub struct SyntaxToken{
    pub text: String,
    pub position: usize,
    pub kind: SyntaxKind
}

#[derive(PartialEq, Debug)]
pub enum SyntaxKind {
    OpenBracketToken,
    CloseBracketToken,
    OpenBraceToken,
    CloseBraceToken,
    CommaToken,
    ColonToken,
    IdentifierToken,
    StringLiteralToken,
    NumberLiteralToken,
    TrueLiteralToken,
    FalseLiteralToken,
    NullLiteralToken,
    WhiteSpaceToken,
    CommentToken,
    BlockCommentToken,
    EndOfFileToken,
    BadToken
}