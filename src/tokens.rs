use std::fmt;

/* Object that represents actual tokens */
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
    pub src_info: (usize, usize)
}

impl Token {
    pub fn new(token_type: TokenType, token_value: String, src_info: (usize, usize)) -> Token {
        Token { token_type, token_value, src_info }
    }

    pub fn to_op(&self) -> &str {
        match self.token_type {
            TokenType::Null => "NOP",
            TokenType::Eof => "OP_EXIT",
            TokenType::OpPlus => "OP_ADD",
            TokenType::OpMinus => "OP_SUB",
            TokenType::OpMult => "OP_MULT",
            TokenType::OpDivi => "OP_DIVI",
            _ => "NOP"
        }
    }
}

/* Names for different types of Tokens */
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Null,   // Only used (currently) for initializing the Scanner, might be able to remove this in the future
    Eof,

    //** Keywords
    Begin,
    End,

    //** Operators
    OpPlus,
    OpMult,
    OpMinus,
    OpDivi,
    OpAssign,

    //** Constants
    IntLit,
    RealLit,
    // StrLit,

    //** Other
    Ident,
    LParen,
    RParen,
    Semi,
}

impl TokenType {
    pub fn to_str(&self) -> &str {
        match *self {
            TokenType::Null => "TK_NULL",
            TokenType::Eof => "TK_EOF",
            TokenType::Begin => "TK_BEGIN",
            TokenType::End => "TK_END",
            TokenType::OpPlus => "TK_PLUS",
            TokenType::OpMinus => "TK_MINUS",
            TokenType::OpMult => "TK_MULT",
            TokenType::OpDivi => "TK_DIVI",
            TokenType::IntLit => "TK_INTLIT",
            TokenType::RealLit => "TK_REALLIT",
            TokenType::Ident => "TK_IDENT",
            TokenType::LParen => "TK_LPAREN",
            TokenType::RParen => "TK_RPAREN",
            TokenType::Semi => "TK_SEMICOL",
            _ => "OTHER"
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenType::IntLit => write!(f, "TK_INTLIT"),
            TokenType::OpPlus => write!(f, "TK_OPPLUS"),
            _ => write!(f, "OTHER")
        }
    }
}

/* Names for different categories of character values the scanner might encounter */
#[derive(Debug, Copy, Clone)]
pub enum CharGroup {
    INVLD,
    WHITE,
    PUNCT,
    DIGIT,
    ALPHA,
    QUOTE,
}

impl fmt::Display for CharGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CharGroup::INVLD => write!(f, "INVLD"),
            CharGroup::WHITE => write!(f, "WHITE"),
            CharGroup::PUNCT => write!(f, "PUNCT"),
            CharGroup::DIGIT => write!(f, "DIGIT"),
            CharGroup::ALPHA => write!(f, "ALPHA"),
            CharGroup::QUOTE => write!(f, "QUOTE"),
        }
    }
}


pub fn get_char_group(value: u8) -> CharGroup {
    // Should automatically return CharGroup::INVLD if value > 127

    // Complete ASCII table mapped to CharGroup values
    // Function performs a table lookup
    let charmap = &[
        CharGroup::WHITE,  // 0    NULL
        CharGroup::WHITE,  // 1    START OF HEADING
        CharGroup::WHITE,  // 2    START OF TEXT
        CharGroup::WHITE,  // 3    END OF TEXT
        CharGroup::WHITE,  // 4    END OF TRANSMISSION
        CharGroup::WHITE,  // 5    ENQUIRY
        CharGroup::WHITE,  // 6    ACKNOWLEDGE
        CharGroup::WHITE,  // 7    BELL
        CharGroup::WHITE,  // 8    BACKSPACE
        CharGroup::WHITE,  // 9    HORIZONTAL TAB
        CharGroup::WHITE,  // 10   * LINE FEED
        CharGroup::WHITE,  // 11   VERTICAL TAB
        CharGroup::WHITE,  // 12   FORM FEED
        CharGroup::WHITE,  // 13   * CARRIAGE RETURN
        CharGroup::WHITE,  // 14   SHIFT OUT
        CharGroup::WHITE,  // 15   SHIFT IN
        CharGroup::WHITE,  // 16   DATA LINK ESCAPE
        CharGroup::WHITE,  // 17   DEVICE CONTROL 1
        CharGroup::WHITE,  // 18   DEVICE CONTROL 2
        CharGroup::WHITE,  // 19   DEVICE CONTROL 3
        CharGroup::WHITE,  // 20   DEVICE CONTROL 4
        CharGroup::WHITE,  // 21   NEGATIVE ACKNOWLEDGE
        CharGroup::WHITE,  // 22   SYNCHRONOUS IDLE
        CharGroup::WHITE,  // 23   END OF TRANSMISSION BLOCK
        CharGroup::WHITE,  // 24   CANCEL
        CharGroup::WHITE,  // 25   END OF MEDIUM
        CharGroup::WHITE,  // 26   SUBSTITUTE
        CharGroup::WHITE,  // 27   ESCAPE
        CharGroup::WHITE,  // 28   FILE SEPARATOR
        CharGroup::WHITE,  // 29   GROUP SEPARATOR
        CharGroup::WHITE,  // 30   RECORD SEPARATOR
        CharGroup::WHITE,  // 31   UNIT SEPARATOR
        CharGroup::WHITE,  // 32   SPACE
        CharGroup::INVLD,  // 33   !
        CharGroup::INVLD,  // 34   "
        CharGroup::INVLD,  // 35   #
        CharGroup::INVLD,  // 36   $
        CharGroup::INVLD,  // 37   %
        CharGroup::INVLD,  // 38   &
        CharGroup::INVLD,  // 39   '
        CharGroup::PUNCT,  // 40   (
        CharGroup::PUNCT,  // 41   )
        CharGroup::PUNCT,  // 42   *
        CharGroup::PUNCT,  // 43   +
        CharGroup::PUNCT,  // 44   ,
        CharGroup::PUNCT,  // 45   -
        CharGroup::PUNCT,  // 46   .
        CharGroup::PUNCT,  // 47   /
        CharGroup::DIGIT,  // 48   0
        CharGroup::DIGIT,  // 49   1
        CharGroup::DIGIT,  // 50   2
        CharGroup::DIGIT,  // 51   3
        CharGroup::DIGIT,  // 52   4
        CharGroup::DIGIT,  // 53   5
        CharGroup::DIGIT,  // 54   6
        CharGroup::DIGIT,  // 55   7
        CharGroup::DIGIT,  // 56   8
        CharGroup::DIGIT,  // 57   9
        CharGroup::PUNCT,  // 58   :
        CharGroup::PUNCT,  // 59   ;
        CharGroup::PUNCT,  // 60   <
        CharGroup::PUNCT,  // 61   =
        CharGroup::PUNCT,  // 62   >
        CharGroup::INVLD,  // 63   ?
        CharGroup::INVLD,  // 64   @
        CharGroup::ALPHA,  // 65   A
        CharGroup::ALPHA,  // 66   B
        CharGroup::ALPHA,  // 67   C
        CharGroup::ALPHA,  // 68   D
        CharGroup::ALPHA,  // 69   E
        CharGroup::ALPHA,  // 70   F
        CharGroup::ALPHA,  // 71   G
        CharGroup::ALPHA,  // 72   H
        CharGroup::ALPHA,  // 73   I
        CharGroup::ALPHA,  // 74   J
        CharGroup::ALPHA,  // 75   K
        CharGroup::ALPHA,  // 76   L
        CharGroup::ALPHA,  // 77   M
        CharGroup::ALPHA,  // 78   N
        CharGroup::ALPHA,  // 79   O
        CharGroup::ALPHA,  // 80   P
        CharGroup::ALPHA,  // 81   Q
        CharGroup::ALPHA,  // 82   R
        CharGroup::ALPHA,  // 83   S
        CharGroup::ALPHA,  // 84   T
        CharGroup::ALPHA,  // 85   U
        CharGroup::ALPHA,  // 86   V
        CharGroup::ALPHA,  // 87   W
        CharGroup::ALPHA,  // 88   X
        CharGroup::ALPHA,  // 89   Y
        CharGroup::ALPHA,  // 90   Z
        CharGroup::PUNCT,  // 91   [
        CharGroup::INVLD,  // 92   \
        CharGroup::PUNCT,  // 93   ]
        CharGroup::INVLD,  // 94   ^
        CharGroup::INVLD,  // 95   _
        CharGroup::PUNCT,  // 96   `
        CharGroup::ALPHA,  // 97   a
        CharGroup::ALPHA,  // 98   b
        CharGroup::ALPHA,  // 99   c
        CharGroup::ALPHA,  // 100  d
        CharGroup::ALPHA,  // 101  e
        CharGroup::ALPHA,  // 102  f
        CharGroup::ALPHA,  // 103  g
        CharGroup::ALPHA,  // 104  h
        CharGroup::ALPHA,  // 105  i
        CharGroup::ALPHA,  // 106  j
        CharGroup::ALPHA,  // 107  k
        CharGroup::ALPHA,  // 108  l
        CharGroup::ALPHA,  // 109  m
        CharGroup::ALPHA,  // 110  n
        CharGroup::ALPHA,  // 111  o
        CharGroup::ALPHA,  // 112  p
        CharGroup::ALPHA,  // 113  q
        CharGroup::ALPHA,  // 114  r
        CharGroup::ALPHA,  // 115  s
        CharGroup::ALPHA,  // 116  t
        CharGroup::ALPHA,  // 117  u
        CharGroup::ALPHA,  // 118  v
        CharGroup::ALPHA,  // 119  w
        CharGroup::ALPHA,  // 120  x
        CharGroup::ALPHA,  // 121  y
        CharGroup::ALPHA,  // 122  z
        CharGroup::PUNCT,  // 123  {
        CharGroup::INVLD,  // 124  |
        CharGroup::PUNCT,  // 125  }
        CharGroup::INVLD,  // 126  ~
        CharGroup::INVLD,  // 127  DEL
    ];

    charmap[value as usize]
}
