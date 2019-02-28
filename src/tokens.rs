pub enum TokenType {
    TK_BEGIN,
    TK_END,
    TK_PLUS,
    TK_LPAREN,
    TK_RPAREN,
    TK_IDENT,
}

enum ByteType {
    INVALID,
    WHITESPACE,
    PUNCTUATION,
    DIGIT,
    ALPHA,
}

pub ASCIIMapper = [
    ByteType::WHITESPACE,   // 0    NULL
    ByteType::WHITESPACE,   // 1    START OF HEADING
    ByteType::WHITESPACE,   // 2    START OF TEXT
    ByteType::WHITESPACE,   // 3    END OF TEXT
    ByteType::WHITESPACE,   // 4    END OF TRANSMISSION
    ByteType::WHITESPACE,   // 5    ENQUIRY
    ByteType::WHITESPACE,   // 6    ACKNOWLEDGE
    ByteType::WHITESPACE,   // 7    BELL
    ByteType::WHITESPACE,   // 8    BACKSPACE
    ByteType::WHITESPACE,   // 9    HORIZONTAL TAB
    ByteType::WHITESPACE,   // 10   * LINE FEED
    ByteType::WHITESPACE,   // 11   VERTICAL TAB
    ByteType::WHITESPACE,   // 12   FORM FEED
    ByteType::WHITESPACE,   // 13   * CARRIAGE RETURN
    ByteType::WHITESPACE,   // 14   SHIFT OUT
    ByteType::WHITESPACE,   // 15   SHIFT IN
    ByteType::WHITESPACE,   // 16   DATA LINK ESCAPE
    ByteType::WHITESPACE,   // 17   DEVICE CONTROL 1
    ByteType::WHITESPACE,   // 18   DEVICE CONTROL 2
    ByteType::WHITESPACE,   // 19   DEVICE CONTROL 3
    ByteType::WHITESPACE,   // 20   DEVICE CONTROL 4
    ByteType::WHITESPACE,   // 21   NEGATIVE ACKNOWLEDGE
    ByteType::WHITESPACE,   // 22   SYNCHRONOUS IDLE
    ByteType::WHITESPACE,   // 23   END OF TRANSMISSION BLOCK
    ByteType::WHITESPACE,   // 24   CANCEL
    ByteType::WHITESPACE,   // 25   END OF MEDIUM
    ByteType::WHITESPACE,   // 26   SUBSTITUTE
    ByteType::WHITESPACE,   // 27   ESCAPE
    ByteType::WHITESPACE,   // 28   FILE SEPARATOR
    ByteType::WHITESPACE,   // 29   GROUP SEPARATOR
    ByteType::WHITESPACE,   // 30   RECORD SEPARATOR
    ByteType::WHITESPACE,   // 31   UNIT SEPARATOR
    ByteType::WHITESPACE,   // 32   SPACE

    ByteType::PUNCTUATION,  // 33   !
    ByteType::PUNCTUATION,  // 34   "
    ByteType::PUNCTUATION,  // 35   #
    ByteType::PUNCTUATION,  // 36   $
    ByteType::PUNCTUATION,  // 37   %
    ByteType::PUNCTUATION,  // 38   &
    ByteType::PUNCTUATION,  // 39   '
    ByteType::PUNCTUATION,  // 40   (
    ByteType::PUNCTUATION,  // 41   )
    ByteType::PUNCTUATION,  // 42   *
    ByteType::PUNCTUATION,  // 43   +
    ByteType::PUNCTUATION,  // 44   ,
    ByteType::PUNCTUATION,  // 45   -
    ByteType::PUNCTUATION,  // 46   .
    ByteType::PUNCTUATION,  // 47   /

    ByteType::DIGIT,        // 48   0
    ByteType::DIGIT,        // 49   1
    ByteType::DIGIT,        // 50   2
    ByteType::DIGIT,        // 51   3
    ByteType::DIGIT,        // 52   4
    ByteType::DIGIT,        // 53   5
    ByteType::DIGIT,        // 54   6
    ByteType::DIGIT,        // 55   7
    ByteType::DIGIT,        // 56   8
    ByteType::DIGIT,        // 57   9

    ByteType::PUNCTUATION,  // 58   :
    ByteType::PUNCTUATION,  // 59   ;
    ByteType::PUNCTUATION,  // 60   <
    ByteType::PUNCTUATION,  // 61   =
    ByteType::PUNCTUATION,  // 62   >
    ByteType::PUNCTUATION,  // 63   ?
    ByteType::PUNCTUATION,  // 64   @

    ByteType::ALPHA,        // 65   A
    ByteType::ALPHA,        // 66   B
    ByteType::ALPHA,        // 67   C
    ByteType::ALPHA,        // 68   D
    ByteType::ALPHA,        // 69   E
    ByteType::ALPHA,        // 70   F
    ByteType::ALPHA,        // 71   G
    ByteType::ALPHA,        // 72   H
    ByteType::ALPHA,        // 73   I
    ByteType::ALPHA,        // 74   J
    ByteType::ALPHA,        // 75   K
    ByteType::ALPHA,        // 76   L
    ByteType::ALPHA,        // 77   M
    ByteType::ALPHA,        // 78   N
    ByteType::ALPHA,        // 79   O
    ByteType::ALPHA,        // 80   P
    ByteType::ALPHA,        // 81   Q
    ByteType::ALPHA,        // 82   R
    ByteType::ALPHA,        // 83   S
    ByteType::ALPHA,        // 84   T
    ByteType::ALPHA,        // 85   U
    ByteType::ALPHA,        // 86   V
    ByteType::ALPHA,        // 87   W
    ByteType::ALPHA,        // 88   X
    ByteType::ALPHA,        // 89   Y
    ByteType::ALPHA,        // 90   Z

    ByteType::PUNCTUATION,  // 91   [
    ByteType::PUNCTUATION,  // 92   \
    ByteType::PUNCTUATION,  // 93   ]
    ByteType::PUNCTUATION,  // 94   ^
    ByteType::PUNCTUATION,  // 95   _
    ByteType::PUNCTUATION,  // 96   `

    ByteType::ALPHA,        // 97   a
    ByteType::ALPHA,        // 98   b
    ByteType::ALPHA,        // 99   c
    ByteType::ALPHA,        // 100  d
    ByteType::ALPHA,        // 101  e
    ByteType::ALPHA,        // 102  f
    ByteType::ALPHA,        // 103  g
    ByteType::ALPHA,        // 104  h
    ByteType::ALPHA,        // 105  i
    ByteType::ALPHA,        // 106  j
    ByteType::ALPHA,        // 107  k
    ByteType::ALPHA,        // 108  l
    ByteType::ALPHA,        // 109  m
    ByteType::ALPHA,        // 110  n
    ByteType::ALPHA,        // 111  o
    ByteType::ALPHA,        // 112  p
    ByteType::ALPHA,        // 113  q
    ByteType::ALPHA,        // 114  r
    ByteType::ALPHA,        // 115  s
    ByteType::ALPHA,        // 116  t
    ByteType::ALPHA,        // 117  u
    ByteType::ALPHA,        // 118  v
    ByteType::ALPHA,        // 119  w
    ByteType::ALPHA,        // 120  x
    ByteType::ALPHA,        // 121  y
    ByteType::ALPHA,        // 122  z

    ByteType::PUNCTUATION,  // 123  {
    ByteType::PUNCTUATION,  // 124  |
    ByteType::PUNCTUATION,  // 125  }
    ByteType::PUNCTUATION,  // 126  ~
    ByteType::INVALID,      // 127  DEL
]
