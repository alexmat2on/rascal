use std::fmt;

/* Object that represents actual tokens */
#[derive(Debug)]
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
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Null,   // Only used (currently) for initializing the Scanner, might be able to remove this in the future
    Eof,
    // Sep,

    // Keywords
    // Begin,
    // End,

    // Operators
    OpPlus,
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

/* Names for different character values the scanner might encounter */
#[derive(Debug)]
pub enum ByteType {
    INVLD,
    WHITE,
    PUNCT,
    DIGIT,
    ALPHA,
}

impl fmt::Display for ByteType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ByteType::INVLD => write!(f, "INVLD"),
            ByteType::WHITE => write!(f, "WHITE"),
            ByteType::PUNCT => write!(f, "PUNCT"),
            ByteType::DIGIT => write!(f, "DIGIT"),
            ByteType::ALPHA => write!(f, "ALPHA"),
        }
    }
}


pub fn get_byte_type(value: u8) -> &'static ByteType {
    // Should automatically return ByteType::INVLD if value > 127

    // Complete ASCII table mapped to ByteType values
    // Function performs a table lookup
    let charmap = &[
        ByteType::WHITE,  // 0    NULL
        ByteType::WHITE,  // 1    START OF HEADING
        ByteType::WHITE,  // 2    START OF TEXT
        ByteType::WHITE,  // 3    END OF TEXT
        ByteType::WHITE,  // 4    END OF TRANSMISSION
        ByteType::WHITE,  // 5    ENQUIRY
        ByteType::WHITE,  // 6    ACKNOWLEDGE
        ByteType::WHITE,  // 7    BELL
        ByteType::WHITE,  // 8    BACKSPACE
        ByteType::WHITE,  // 9    HORIZONTAL TAB
        ByteType::WHITE,  // 10   * LINE FEED
        ByteType::WHITE,  // 11   VERTICAL TAB
        ByteType::WHITE,  // 12   FORM FEED
        ByteType::WHITE,  // 13   * CARRIAGE RETURN
        ByteType::WHITE,  // 14   SHIFT OUT
        ByteType::WHITE,  // 15   SHIFT IN
        ByteType::WHITE,  // 16   DATA LINK ESCAPE
        ByteType::WHITE,  // 17   DEVICE CONTROL 1
        ByteType::WHITE,  // 18   DEVICE CONTROL 2
        ByteType::WHITE,  // 19   DEVICE CONTROL 3
        ByteType::WHITE,  // 20   DEVICE CONTROL 4
        ByteType::WHITE,  // 21   NEGATIVE ACKNOWLEDGE
        ByteType::WHITE,  // 22   SYNCHRONOUS IDLE
        ByteType::WHITE,  // 23   END OF TRANSMISSION BLOCK
        ByteType::WHITE,  // 24   CANCEL
        ByteType::WHITE,  // 25   END OF MEDIUM
        ByteType::WHITE,  // 26   SUBSTITUTE
        ByteType::WHITE,  // 27   ESCAPE
        ByteType::WHITE,  // 28   FILE SEPARATOR
        ByteType::WHITE,  // 29   GROUP SEPARATOR
        ByteType::WHITE,  // 30   RECORD SEPARATOR
        ByteType::WHITE,  // 31   UNIT SEPARATOR
        ByteType::WHITE,  // 32   SPACE
        ByteType::PUNCT,  // 33   !
        ByteType::PUNCT,  // 34   "
        ByteType::PUNCT,  // 35   #
        ByteType::PUNCT,  // 36   $
        ByteType::PUNCT,  // 37   %
        ByteType::PUNCT,  // 38   &
        ByteType::PUNCT,  // 39   '
        ByteType::PUNCT,  // 40   (
        ByteType::PUNCT,  // 41   )
        ByteType::PUNCT,  // 42   *
        ByteType::PUNCT,  // 43   +
        ByteType::PUNCT,  // 44   ,
        ByteType::PUNCT,  // 45   -
        ByteType::PUNCT,  // 46   .
        ByteType::PUNCT,  // 47   /
        ByteType::DIGIT,  // 48   0
        ByteType::DIGIT,  // 49   1
        ByteType::DIGIT,  // 50   2
        ByteType::DIGIT,  // 51   3
        ByteType::DIGIT,  // 52   4
        ByteType::DIGIT,  // 53   5
        ByteType::DIGIT,  // 54   6
        ByteType::DIGIT,  // 55   7
        ByteType::DIGIT,  // 56   8
        ByteType::DIGIT,  // 57   9
        ByteType::PUNCT,  // 58   :
        ByteType::PUNCT,  // 59   ;
        ByteType::PUNCT,  // 60   <
        ByteType::PUNCT,  // 61   =
        ByteType::PUNCT,  // 62   >
        ByteType::PUNCT,  // 63   ?
        ByteType::PUNCT,  // 64   @
        ByteType::ALPHA,  // 65   A
        ByteType::ALPHA,  // 66   B
        ByteType::ALPHA,  // 67   C
        ByteType::ALPHA,  // 68   D
        ByteType::ALPHA,  // 69   E
        ByteType::ALPHA,  // 70   F
        ByteType::ALPHA,  // 71   G
        ByteType::ALPHA,  // 72   H
        ByteType::ALPHA,  // 73   I
        ByteType::ALPHA,  // 74   J
        ByteType::ALPHA,  // 75   K
        ByteType::ALPHA,  // 76   L
        ByteType::ALPHA,  // 77   M
        ByteType::ALPHA,  // 78   N
        ByteType::ALPHA,  // 79   O
        ByteType::ALPHA,  // 80   P
        ByteType::ALPHA,  // 81   Q
        ByteType::ALPHA,  // 82   R
        ByteType::ALPHA,  // 83   S
        ByteType::ALPHA,  // 84   T
        ByteType::ALPHA,  // 85   U
        ByteType::ALPHA,  // 86   V
        ByteType::ALPHA,  // 87   W
        ByteType::ALPHA,  // 88   X
        ByteType::ALPHA,  // 89   Y
        ByteType::ALPHA,  // 90   Z
        ByteType::PUNCT,  // 91   [
        ByteType::PUNCT,  // 92   \
        ByteType::PUNCT,  // 93   ]
        ByteType::PUNCT,  // 94   ^
        ByteType::PUNCT,  // 95   _
        ByteType::PUNCT,  // 96   `
        ByteType::ALPHA,  // 97   a
        ByteType::ALPHA,  // 98   b
        ByteType::ALPHA,  // 99   c
        ByteType::ALPHA,  // 100  d
        ByteType::ALPHA,  // 101  e
        ByteType::ALPHA,  // 102  f
        ByteType::ALPHA,  // 103  g
        ByteType::ALPHA,  // 104  h
        ByteType::ALPHA,  // 105  i
        ByteType::ALPHA,  // 106  j
        ByteType::ALPHA,  // 107  k
        ByteType::ALPHA,  // 108  l
        ByteType::ALPHA,  // 109  m
        ByteType::ALPHA,  // 110  n
        ByteType::ALPHA,  // 111  o
        ByteType::ALPHA,  // 112  p
        ByteType::ALPHA,  // 113  q
        ByteType::ALPHA,  // 114  r
        ByteType::ALPHA,  // 115  s
        ByteType::ALPHA,  // 116  t
        ByteType::ALPHA,  // 117  u
        ByteType::ALPHA,  // 118  v
        ByteType::ALPHA,  // 119  w
        ByteType::ALPHA,  // 120  x
        ByteType::ALPHA,  // 121  y
        ByteType::ALPHA,  // 122  z
        ByteType::PUNCT,  // 123  {
        ByteType::PUNCT,  // 124  |
        ByteType::PUNCT,  // 125  }
        ByteType::PUNCT,  // 126  ~
        ByteType::INVLD,  // 127  DEL
    ];

    &charmap[value as usize]
}
