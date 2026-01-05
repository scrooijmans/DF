# Enum Token Copy item path

<a href="https://docs.rs/sqlparser/0.58.0/x86_64-unknown-linux-gnu/src/sqlparser/tokenizer.rs.html#55" class="src">Source</a>

``` rust
pub enum Token {
Show 103 variants    EOF,
    Word(Word),
    Number(String, bool),
    Char(char),
    SingleQuotedString(String),
    DoubleQuotedString(String),
    TripleSingleQuotedString(String),
    TripleDoubleQuotedString(String),
    DollarQuotedString(DollarQuotedString),
    SingleQuotedByteStringLiteral(String),
    DoubleQuotedByteStringLiteral(String),
    TripleSingleQuotedByteStringLiteral(String),
    TripleDoubleQuotedByteStringLiteral(String),
    SingleQuotedRawStringLiteral(String),
    DoubleQuotedRawStringLiteral(String),
    TripleSingleQuotedRawStringLiteral(String),
    TripleDoubleQuotedRawStringLiteral(String),
    NationalStringLiteral(String),
    EscapedStringLiteral(String),
    UnicodeStringLiteral(String),
    HexStringLiteral(String),
    Comma,
    Whitespace(Whitespace),
    DoubleEq,
    Eq,
    Neq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Spaceship,
    Plus,
    Minus,
    Mul,
    Div,
    DuckIntDiv,
    Mod,
    StringConcat,
    LParen,
    RParen,
    Period,
    Colon,
    DoubleColon,
    Assignment,
    SemiColon,
    Backslash,
    LBracket,
    RBracket,
    Ampersand,
    Pipe,
    Caret,
    LBrace,
    RBrace,
    RArrow,
    Sharp,
    DoubleSharp,
    Tilde,
    TildeAsterisk,
    ExclamationMarkTilde,
    ExclamationMarkTildeAsterisk,
    DoubleTilde,
    DoubleTildeAsterisk,
    ExclamationMarkDoubleTilde,
    ExclamationMarkDoubleTildeAsterisk,
    ShiftLeft,
    ShiftRight,
    Overlap,
    ExclamationMark,
    DoubleExclamationMark,
    AtSign,
    CaretAt,
    PGSquareRoot,
    PGCubeRoot,
    Placeholder(String),
    Arrow,
    LongArrow,
    HashArrow,
    AtDashAt,
    QuestionMarkDash,
    AmpersandLeftAngleBracket,
    AmpersandRightAngleBracket,
    AmpersandLeftAngleBracketVerticalBar,
    VerticalBarAmpersandRightAngleBracket,
    TwoWayArrow,
    LeftAngleBracketCaret,
    RightAngleBracketCaret,
    QuestionMarkSharp,
    QuestionMarkDashVerticalBar,
    QuestionMarkDoubleVerticalBar,
    TildeEqual,
    ShiftLeftVerticalBar,
    VerticalBarShiftRight,
    VerticalBarRightAngleBracket,
    HashLongArrow,
    AtArrow,
    ArrowAt,
    HashMinus,
    AtQuestion,
    AtAt,
    Question,
    QuestionAnd,
    QuestionPipe,
    CustomBinaryOperator(String),
}
```

Expand description

SQL Token enumeration

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.EOF" class="anchor">§</a>

### EOF

An end-of-file marker, not a real token

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Word" class="anchor">§</a>

### Word(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Word.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::Word">Word</a>)

A keyword (like SELECT) or an optionally quoted SQL identifier

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Number" class="anchor">§</a>

### Number(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>)

An unsigned numeric literal

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Char" class="anchor">§</a>

### Char(<a href="https://doc.rust-lang.org/nightly/std/primitive.char.html" class="primitive">char</a>)

A character that could not be tokenized

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.SingleQuotedString" class="anchor">§</a>

### SingleQuotedString(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Single quoted string: i.e: ‘string’

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.DoubleQuotedString" class="anchor">§</a>

### DoubleQuotedString(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Double quoted string: i.e: “string”

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.TripleSingleQuotedString" class="anchor">§</a>

### TripleSingleQuotedString(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Triple single quoted strings: Example ‘’‘abc’‘’ [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.TripleDoubleQuotedString" class="anchor">§</a>

### TripleDoubleQuotedString(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Triple double quoted strings: Example “”“abc”“” [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.DollarQuotedString" class="anchor">§</a>

### DollarQuotedString(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.DollarQuotedString.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::DollarQuotedString">DollarQuotedString</a>)

Dollar quoted string: i.e: \$\$string\$\$ or \$tag_name\$string\$tag_name\$

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.SingleQuotedByteStringLiteral" class="anchor">§</a>

### SingleQuotedByteStringLiteral(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Byte string literal: i.e: b’string’ or B’string’ (note that some backends, such as PostgreSQL, may treat this syntax as a bit string literal instead, i.e: b’10010101’)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.DoubleQuotedByteStringLiteral" class="anchor">§</a>

### DoubleQuotedByteStringLiteral(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Byte string literal: i.e: b“string“ or B“string“

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.TripleSingleQuotedByteStringLiteral" class="anchor">§</a>

### TripleSingleQuotedByteStringLiteral(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Triple single quoted literal with byte string prefix. Example `B'''abc'''` [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.TripleDoubleQuotedByteStringLiteral" class="anchor">§</a>

### TripleDoubleQuotedByteStringLiteral(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Triple double quoted literal with byte string prefix. Example `B"""abc"""` [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.SingleQuotedRawStringLiteral" class="anchor">§</a>

### SingleQuotedRawStringLiteral(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Single quoted literal with raw string prefix. Example `R'abc'` [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.DoubleQuotedRawStringLiteral" class="anchor">§</a>

### DoubleQuotedRawStringLiteral(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Double quoted literal with raw string prefix. Example `R"abc"` [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.TripleSingleQuotedRawStringLiteral" class="anchor">§</a>

### TripleSingleQuotedRawStringLiteral(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Triple single quoted literal with raw string prefix. Example `R'''abc'''` [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.TripleDoubleQuotedRawStringLiteral" class="anchor">§</a>

### TripleDoubleQuotedRawStringLiteral(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Triple double quoted literal with raw string prefix. Example `R"""abc"""` [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical#quoted_literals)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.NationalStringLiteral" class="anchor">§</a>

### NationalStringLiteral(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

“National” string literal: i.e: N’string’

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.EscapedStringLiteral" class="anchor">§</a>

### EscapedStringLiteral(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

“escaped” string literal, which are an extension to the SQL standard: i.e: e’first \n second’ or E ‘first \n second’

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.UnicodeStringLiteral" class="anchor">§</a>

### UnicodeStringLiteral(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Unicode string literal: i.e: U&‘first \000A second’

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.HexStringLiteral" class="anchor">§</a>

### HexStringLiteral(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Hexadecimal string literal: i.e.: X’deadbeef’

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Comma" class="anchor">§</a>

### Comma

Comma

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Whitespace" class="anchor">§</a>

### Whitespace(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Whitespace.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Whitespace">Whitespace</a>)

Whitespace (space, tab, etc)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.DoubleEq" class="anchor">§</a>

### DoubleEq

Double equals sign `==`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Eq" class="anchor">§</a>

### Eq

Equality operator `=`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Neq" class="anchor">§</a>

### Neq

Not Equals operator `<>` (or `!=` in some dialects)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Lt" class="anchor">§</a>

### Lt

Less Than operator `<`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Gt" class="anchor">§</a>

### Gt

Greater Than operator `>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.LtEq" class="anchor">§</a>

### LtEq

Less Than Or Equals operator `<=`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.GtEq" class="anchor">§</a>

### GtEq

Greater Than Or Equals operator `>=`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Spaceship" class="anchor">§</a>

### Spaceship

Spaceship operator \<=\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Plus" class="anchor">§</a>

### Plus

Plus operator `+`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Minus" class="anchor">§</a>

### Minus

Minus operator `-`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Mul" class="anchor">§</a>

### Mul

Multiplication operator `*`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Div" class="anchor">§</a>

### Div

Division operator `/`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.DuckIntDiv" class="anchor">§</a>

### DuckIntDiv

Integer division operator `//` in DuckDB

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Mod" class="anchor">§</a>

### Mod

Modulo Operator `%`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.StringConcat" class="anchor">§</a>

### StringConcat

String concatenation `||`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.LParen" class="anchor">§</a>

### LParen

Left parenthesis `(`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.RParen" class="anchor">§</a>

### RParen

Right parenthesis `)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Period" class="anchor">§</a>

### Period

Period (used for compound identifiers or projections into nested types)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Colon" class="anchor">§</a>

### Colon

Colon `:`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.DoubleColon" class="anchor">§</a>

### DoubleColon

DoubleColon `::` (used for casting in PostgreSQL)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Assignment" class="anchor">§</a>

### Assignment

Assignment `:=` (used for keyword argument in DuckDB macros and some functions, and for variable declarations in DuckDB and Snowflake)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.SemiColon" class="anchor">§</a>

### SemiColon

SemiColon `;` used as separator for COPY and payload

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Backslash" class="anchor">§</a>

### Backslash

Backslash `\` used in terminating the COPY payload with `\.`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.LBracket" class="anchor">§</a>

### LBracket

Left bracket `[`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.RBracket" class="anchor">§</a>

### RBracket

Right bracket `]`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Ampersand" class="anchor">§</a>

### Ampersand

Ampersand `&`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Pipe" class="anchor">§</a>

### Pipe

Pipe `|`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Caret" class="anchor">§</a>

### Caret

Caret `^`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.LBrace" class="anchor">§</a>

### LBrace

Left brace `{`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.RBrace" class="anchor">§</a>

### RBrace

Right brace `}`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.RArrow" class="anchor">§</a>

### RArrow

Right Arrow `=>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Sharp" class="anchor">§</a>

### Sharp

Sharp `#` used for PostgreSQL Bitwise XOR operator, also PostgreSQL/Redshift geometrical unary/binary operator (Number of points in path or polygon/Intersection)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.DoubleSharp" class="anchor">§</a>

### DoubleSharp

`##` PostgreSQL/Redshift geometrical binary operator (Point of closest proximity)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Tilde" class="anchor">§</a>

### Tilde

Tilde `~` used for PostgreSQL Bitwise NOT operator or case sensitive match regular expression operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.TildeAsterisk" class="anchor">§</a>

### TildeAsterisk

`~*` , a case insensitive match regular expression operator in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.ExclamationMarkTilde" class="anchor">§</a>

### ExclamationMarkTilde

`!~` , a case sensitive not match regular expression operator in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.ExclamationMarkTildeAsterisk" class="anchor">§</a>

### ExclamationMarkTildeAsterisk

`!~*` , a case insensitive not match regular expression operator in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.DoubleTilde" class="anchor">§</a>

### DoubleTilde

`~~`, a case sensitive match pattern operator in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.DoubleTildeAsterisk" class="anchor">§</a>

### DoubleTildeAsterisk

`~~*`, a case insensitive match pattern operator in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.ExclamationMarkDoubleTilde" class="anchor">§</a>

### ExclamationMarkDoubleTilde

`!~~`, a case sensitive not match pattern operator in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.ExclamationMarkDoubleTildeAsterisk" class="anchor">§</a>

### ExclamationMarkDoubleTildeAsterisk

`!~~*`, a case insensitive not match pattern operator in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.ShiftLeft" class="anchor">§</a>

### ShiftLeft

`<<`, a bitwise shift left operator in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.ShiftRight" class="anchor">§</a>

### ShiftRight

`>>`, a bitwise shift right operator in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Overlap" class="anchor">§</a>

### Overlap

`&&`, an overlap operator in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.ExclamationMark" class="anchor">§</a>

### ExclamationMark

Exclamation Mark `!` used for PostgreSQL factorial operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.DoubleExclamationMark" class="anchor">§</a>

### DoubleExclamationMark

Double Exclamation Mark `!!` used for PostgreSQL prefix factorial operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.AtSign" class="anchor">§</a>

### AtSign

AtSign `@` used for PostgreSQL abs operator, also PostgreSQL/Redshift geometrical unary/binary operator (Center, Contained or on)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.CaretAt" class="anchor">§</a>

### CaretAt

`^@`, a “starts with” string operator in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.PGSquareRoot" class="anchor">§</a>

### PGSquareRoot

`|/`, a square root math operator in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.PGCubeRoot" class="anchor">§</a>

### PGCubeRoot

`||/`, a cube root math operator in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Placeholder" class="anchor">§</a>

### Placeholder(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

`?` or `$` , a prepared statement arg placeholder

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Arrow" class="anchor">§</a>

### Arrow

`->`, used as a operator to extract json field in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.LongArrow" class="anchor">§</a>

### LongArrow

`->>`, used as a operator to extract json field as text in PostgreSQL

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.HashArrow" class="anchor">§</a>

### HashArrow

`#>`, extracts JSON sub-object at the specified path

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.AtDashAt" class="anchor">§</a>

### AtDashAt

`@-@` PostgreSQL/Redshift geometrical unary operator (Length or circumference)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.QuestionMarkDash" class="anchor">§</a>

### QuestionMarkDash

`?-` PostgreSQL/Redshift geometrical unary/binary operator (Is horizontal?/Are horizontally aligned?)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.AmpersandLeftAngleBracket" class="anchor">§</a>

### AmpersandLeftAngleBracket

`&<` PostgreSQL/Redshift geometrical binary operator (Overlaps to left?)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.AmpersandRightAngleBracket" class="anchor">§</a>

### AmpersandRightAngleBracket

`&>` PostgreSQL/Redshift geometrical binary operator (Overlaps to right?)\`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.AmpersandLeftAngleBracketVerticalBar" class="anchor">§</a>

### AmpersandLeftAngleBracketVerticalBar

`&<|` PostgreSQL/Redshift geometrical binary operator (Does not extend above?)\`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.VerticalBarAmpersandRightAngleBracket" class="anchor">§</a>

### VerticalBarAmpersandRightAngleBracket

`|&>` PostgreSQL/Redshift geometrical binary operator (Does not extend below?)\`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.TwoWayArrow" class="anchor">§</a>

### TwoWayArrow

`<->` PostgreSQL/Redshift geometrical binary operator (Distance between)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.LeftAngleBracketCaret" class="anchor">§</a>

### LeftAngleBracketCaret

`<^` PostgreSQL/Redshift geometrical binary operator (Is below?)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.RightAngleBracketCaret" class="anchor">§</a>

### RightAngleBracketCaret

`>^` PostgreSQL/Redshift geometrical binary operator (Is above?)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.QuestionMarkSharp" class="anchor">§</a>

### QuestionMarkSharp

`?#` PostgreSQL/Redshift geometrical binary operator (Intersects or overlaps)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.QuestionMarkDashVerticalBar" class="anchor">§</a>

### QuestionMarkDashVerticalBar

`?-|` PostgreSQL/Redshift geometrical binary operator (Is perpendicular?)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.QuestionMarkDoubleVerticalBar" class="anchor">§</a>

### QuestionMarkDoubleVerticalBar

`?||` PostgreSQL/Redshift geometrical binary operator (Are parallel?)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.TildeEqual" class="anchor">§</a>

### TildeEqual

`~=` PostgreSQL/Redshift geometrical binary operator (Same as)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.ShiftLeftVerticalBar" class="anchor">§</a>

### ShiftLeftVerticalBar

\`\<\<\| PostgreSQL/Redshift geometrical binary operator (Is strictly below?)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.VerticalBarShiftRight" class="anchor">§</a>

### VerticalBarShiftRight

\`\|\>\> PostgreSQL/Redshift geometrical binary operator (Is strictly above?)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.VerticalBarRightAngleBracket" class="anchor">§</a>

### VerticalBarRightAngleBracket

\`\|\> BigQuery pipe operator

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.HashLongArrow" class="anchor">§</a>

### HashLongArrow

`#>>`, extracts JSON sub-object at the specified path as text

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.AtArrow" class="anchor">§</a>

### AtArrow

jsonb @\> jsonb -\> boolean: Test whether left json contains the right json

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.ArrowAt" class="anchor">§</a>

### ArrowAt

jsonb \<@ jsonb -\> boolean: Test whether right json contains the left json

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.HashMinus" class="anchor">§</a>

### HashMinus

jsonb \#- text\[\] -\> jsonb: Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.AtQuestion" class="anchor">§</a>

### AtQuestion

jsonb @? jsonpath -\> boolean: Does JSON path return any item for the specified JSON value?

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.AtAt" class="anchor">§</a>

### AtAt

jsonb @@ jsonpath → boolean: Returns the result of a JSON path predicate check for the specified JSON value. Only the first item of the result is taken into account. If the result is not Boolean, then NULL is returned.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.Question" class="anchor">§</a>

### Question

jsonb ? text -\> boolean: Checks whether the string exists as a top-level key within the jsonb object

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.QuestionAnd" class="anchor">§</a>

### QuestionAnd

jsonb ?& text\[\] -\> boolean: Check whether all members of the text array exist as top-level keys within the jsonb object

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.QuestionPipe" class="anchor">§</a>

### QuestionPipe

jsonb ?\| text\[\] -\> boolean: Check whether any member of the text array exists as top-level keys within the jsonb object

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#variant.CustomBinaryOperator" class="anchor">§</a>

### CustomBinaryOperator(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>)

Custom binary operator This is used to represent any custom binary operator that is not part of the SQL standard. PostgreSQL allows defining custom binary operators using CREATE OPERATOR.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-Token" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.make_keyword" class="fn">make_keyword</a>(keyword: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.make_word" class="fn">make_word</a>(word: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, quote_style: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.char.html" class="primitive">char</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-Clone-for-Token" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-Debug-for-Token" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-Display-for-Token" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-Hash-for-Token" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-Ord-for-Token" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-PartialEq%3CToken%3E-for-TokenWithSpan" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.TokenWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::TokenWithSpan">TokenWithSpan</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.eq-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.ne-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-PartialEq%3CTokenWithSpan%3E-for-Token" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.TokenWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::TokenWithSpan">TokenWithSpan</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.eq-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.TokenWithSpan.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::TokenWithSpan">TokenWithSpan</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.ne-2" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-PartialEq-for-Token" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-PartialOrd-for-Token" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-Visit-for-Token" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visit">Visit</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::Visitor::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-VisitMut-for-Token" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitMut">VisitMut</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#method.visit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html#tymethod.visit" class="fn">visit</a>\<V\>(&mut self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::VisitorMut::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-Eq-for-Token" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#impl-StructuralPartialEq-for-Token" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html" class="enum" title="enum datafusion::logical_expr::sqlparser::tokenizer::Token">Token</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/enum.Token.html#blanket-implementations" class="anchor">§</a>
