use std::fmt;

/* Object that represents actual tokens */
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
}

impl Token {
    pub fn new(token_type : TokenType, token_value : String) -> Token {
        Token { token_type, token_value }
    }
}

/* Names for different types of Tokens */
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum TokenType {
    Null,   // Only used (currently) for initializing the Scanner, might be able to remove this in the future
    Eof,

    // Keywords
    // Begin,
    // End,

    // Operators
    OpPlus,
    OpMult,
    OpMinus,
    OpDivi,
    OpAssign,

    // Constants
    IntLit,
    RealLit,
    // StrLit,

    // Other
    Ident,
    LParen,
    RParen,
    Semi,
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
        CharGroup::PUNCT,  // 33   !
        CharGroup::PUNCT,  // 34   "
        CharGroup::PUNCT,  // 35   #
        CharGroup::PUNCT,  // 36   $
        CharGroup::PUNCT,  // 37   %
        CharGroup::PUNCT,  // 38   &
        CharGroup::PUNCT,  // 39   '
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
        CharGroup::PUNCT,  // 63   ?
        CharGroup::PUNCT,  // 64   @
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
        CharGroup::PUNCT,  // 92   \
        CharGroup::PUNCT,  // 93   ]
        CharGroup::PUNCT,  // 94   ^
        CharGroup::PUNCT,  // 95   _
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
        CharGroup::PUNCT,  // 124  |
        CharGroup::PUNCT,  // 125  }
        CharGroup::PUNCT,  // 126  ~
        CharGroup::INVLD,  // 127  DEL
    ];

    charmap[value as usize]
}