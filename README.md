# A Mini Virtual Machine

[![Build Status][travis-badge]][travis-project] [![codecov][codecov-badge]][codecov-project]

A minimalistic language with virtual machine written in [Rust][rust-lang]. Based on [how to from Terrence Parr][parr-how-to] to write a virtual machine.

The purpose of this project is to get in touch with [Rust][rust-lang].

The crate doc is [here][crate-doc].

## General Building Blocks

In general a general purpose programming language (whether or not it is vm based) is made of three major building blocks:

```text
+-------\      +----------+     +--------------+     +---------+
|        |     |          |     |              |     |         |
| source | --> | frontend | --> | intermediate | --> | backend |
|        |     |          |     |              |     |         |
+--------+     +----------+     +--------------+     +---------+
```

The frontend parses the source code and transforms it into an intermediate form (abstract syntax tree). This tree is transformed into byte code which will be executed by the backend.

## Frontend

The frontend is built of a lexer for lexical analysis (token generation) and a parser to create the abstract syntax tree. It's purpose is to transform the givne source code into an intermediate model (abstract syntax tree).

### Source Code Syntax

The source code syntax is defined by two grammars: one for the lexer and one for the parser. Example source:

```text
var s = "Hello, World!"
println(s)

var x = 1
var y = 2
z = x + y
println(z)
```

#### Lexer Grammar (Regular)

The lexer grammar defines the recognised tokens:

```text
(*
  Everything ending with _OP is an operator.
  Everything ending with _KW is a keyword.
*)

ANY             (* Any character. *)
EOF             (* End of file.   *)
EOL = "\n" .    (* End of line.   *)

WS          = " " | "\n" | "\r" | "\t" .
CHARACTER   = "a" .. "z" | "A" .. "Z" .
DIGIT       = "0" .. "9" .
DIGITS      = DIGIT { DIGIT } .
SIGN        = "+" | "-" .
IDENTIFIER  =  ( CHARACTER | "_" ) { CHARACTER DIGIT } .

(* Types: *)
TRUE                = "true" .
FALSE               = "false" .
BOOLEAN             = TRUE  | FALSE .
INTEGER             = { SIGN } DIGITS .
REAL                = { SIGN } DIGITS "." { DIGITS } { EXPONENT_PART }
                    | { SIGN } DIGITS { EXPONENT_PART } .
EXPONENT_PART       = EXPONENT_INDICATOR INTEGER .
EXPONENT_INDICATOR  = "e" | "E" .
STRING              = '"' ( ANY | '\"' ) { ( ANY | '\"')  } '"' .
CHAR                = "'" ANY "'"

(* Delimiters: *)
LEFT_PAREN      = "(" .
RIGHT_PAREN     = ")" .
LEFT_BRACKET    = "[" .
RIGHT_BRACKET   = "]" .
LEFT_BRACE      = "{" .
RIGHT_BRACE     = "}" .
COMMA           = "," .

(* Logical operators as keywords: *)
AND_KW      = "and" .
OR_KW       = "or" .
NOT_KW      = "not" .
IF_KW       = "if" .
ELSE_KW     = "else" .
WHILE_KW    = "while" .
CONST_KW    = "const" .
VAR_KW      = "var" .

ASSIGN_OP   = "=" .

(* Compare operators: *)
EQUAL_OP                = "==" .
NOT_EQUAL_OP            = "!=" .
EQ_OPS                  = EQUAL_OP | NOT_EQUAL_OP .

LESS_THAN_OP            = "<" .
LESS_THAN_EQUAL_OP      = "<=" .
GREATER_THAN_OP         =  ">" .
GREATER_THAN_EQUAL_OP   = ">=" .
REL_OPS                 = LESS_THAN_OP | LESS_THAN_EQUAL_OP
                        | GREATER_THAN_OP | GREATER_THAN_EQUAL_OP .

(* Math operators: *)
PLUS_OP     = "+" .
MINUS_OP    = "-" .
STAR_OP     = "*" .
SLASH_OP    = "/" .
MOD_OP      = "%" .
ADD_OPS     = PLUS_OP | MINUS_OP.
MUL_OPS     = STAR_OP | SLASH_OP | MOD_OP.
```

#### Parser Grammar (Context Free)

The parser grammar defines the semantic structure of the recognized tokens and how to build a abstract syntax tree from it.

Some definitions:

1. Expressions always return a value.
1. Statements does not return a value.
1. Statements/expressions are terminated by new line.

```text
(* A statement is one line of source. *)
program                 = statement EOL { statement EOL } EOF .
statement               = assignment
                        | constant_declaration
                        | variable_declaration
                        | or_expression .
assignment              = IDENTIFIER ASSIGN_OP or_expression .
constant_declaration    = CONST_KW assignment .
variable_declaration    = VAR_KW ( IDENTIFIER | assignment ) .
or_expression           = and_expression { OR_KW and_expression } .
and_expression          = equal_expression { AND_KW equal_expression } .
equal_expression        = relation_expression { EQ_OPS relation_expression } .
relation_expression     = concat_expression { REL_OPS simple_expression } .
simple_expression       = term { ADD_OPS term } .
term                    = factor { MUL_OPS factor } .
factor                  = var_or_const_value
                        | literal_value
                        | LEFT_PAREN simple_expression RIGHT_PAREN
                        | NOT_KW factor
                        | or_expression
                        | function_call .
var_or_const_value      = IDENTIFIER  .
literal_value           = BOOLEAN | INTEGER | REAL | STRING | CHAR .
function_call           = IDENTIFIER LEFT_PAREN [ function_params ] RIGHT_PAREN .
function_params         = equal_expression { "," or_expression } .
```

## Intermediate

The intermediate part defines the building blocks of the abstract syntax tree (AST). It also provides visitors to walk the AST.

### Some Examples

#### Simple Math Expression

The given source code:

```text
1 + 2 * 3
```

produces the tokens:

```text
<INTEGER(1) '1' [1, 1]>
<OPERATOR(PLUS) '+' [1, 3]>
<INTEGER(2) '2' [1, 5]>
<OPERATOR(STAR) '*' [1, 7]>
<INTEGER(3) '3' [1, 9]>
<EOL '\n' [1, 10]>
<EOF '' [2, 1]>
```

and the AST:

```text
   (+)
   / \
 (1) (*)
     / \
   (2) (3)
```

#### Variable Declaration with Initialization

The given source code:

```text
var x = 1 + 2
```

produces the tokens:

```text
<KEYWORD(VAR) 'var' [1, 1]>
<IDENTIFIER("x") 'x' [1, 5]>
<OPERATOR(ASSIGN) '=' [1, 7]>
<INTEGER(1) '1' [1, 9]>
<OPERATOR(PLUS) '+' [1, 11]>
<INTEGER(2) '2' [1, 13]>
<EOL '\n' [1, 14]>
<EOF '' [2, 1]>
```

and the AST:

```text
      (=)
      / \
(var x) (+)
        / \
      (1) (2)
```

## Backend

The backend is responsible for executing the program.

### Virtual Machine

A virtual machine is an idealized abstraction of a general CPU. Schematic overview of a virtual machine:

```text
+----------+     +----------------------------------------+
|          |     |  /--------\    registers->  +-------+  |
|          |     |  |        |                 |   sp  |  |
|   data   |     |  |  +-----v----+            +-------+  |
|  memory  <----->  |  |   fetch  |            |   fp  |  |
|          |     |  |  +-----+----+            +-------+  |
|          |     |  |        |                 |   ip  |  |
+----------+     |  |  +-----v----+            +-------+  |
+----------+     |  |  |  decode  |                       |
|          |     |  |  +-----+----+                ^      |
|          |     |  |        |                     |      |
|   code   |     |  |  +-----v----+            +-------+  |
|  memory  <----->  |  | execute  |            +-------+  |
|          |     |  |  +-----+----+            +-------+  |
|          |     |  |        |                 +-------+  |
|          |     |  \--------/         stack-> +-------+  |
+----------+     +----------------------------------------+

sp: stack pointer
fp: function pointer
ip: instruction pointer
```

Example of an integer addition operation (iadd instruction):

1. fetch: `opcode = code[ip]`
1. decode: `switch (opcode) { ... }`
1. execute: `stack[++sp] = stack[sp--] + stack[sp--]`

## TODO

- Consider using a parser generator
    - [lalrpop](http://lalrpop.github.io/lalrpop/README.html)

[travis-project]:   https://travis-ci.org/Weltraumschaf/minivm
[travis-badge]:     https://travis-ci.org/Weltraumschaf/minivm.svg?branch=master
[codecov-project]:  https://codecov.io/gh/Weltraumschaf/minivm
[codecov-badge]:    https://codecov.io/gh/Weltraumschaf/minivm/branch/master/graph/badge.svg
[rust-lang]:        https://www.rust-lang.org/
[parr-how-to]:      https://www.youtube.com/watch?feature=youtu.be&v=OjaAToVkoTw
[crate-doc]:        https://weltraumschaf.github.io/minivm/minivm/index.html
