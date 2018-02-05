# A Mini Virtual Machine

[![Build Status](https://travis-ci.org/Weltraumschaf/minivm.svg?branch=master)](https://travis-ci.org/Weltraumschaf/minivm)

A minimalistic language with virtual machine written in [Rust][rust-lang].

The purpose of this project is to get in touch with [Rust][rust-lang].
## Source Code Syntax

### Scanner Grammar (Regular)

```
(*
    Everything ending with _OP is an operator.
    Everything ending with _KW is a keyword.
*)

ANY             (* Any character. *)
EOF             (* End of file. *)
EOL = "\n" .    (* End of line. *)

WS          = " " | "\n" | "\r" | "\t" .
CHARACTER   = "a" .. "z" | "A" .. "Z" .
DIGIT       = "0" .. "9" .
DIGITS      = DIGIT { DIGIT } .
SIGN        = "+" | "-" .
IDENTIFIER      =  ( CHARACTER | "_" ) { CHARACTER DIGIT } .

(* Types *)
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
    
(* logical operators as keywrods *)
AND_KW      = "and" .
OR_KW       = "or" .
NOT_KW      = "not" .
IF_KW       = "if" .
ELSE_KW     = "else" .
WHILE_KW    = "while" .
CONST_KW    = "const" .
VAR_KW      = "var" .

ASSIGN_OP   = "=" .

(* compare operators *)
EQUAL_OP                = "==" .
NOT_EQUAL_OP            = "!=" .
EQ_OPS                  = EQUAL_OP | NOT_EQUAL_OP . 

LESS_THAN_OP            = "<" .
LESS_THAN_EQUAL_OP      = "<=" .
GREATER_THAN_OP         =  ">" .
GREATER_THAN_EQUAL_OP   = ">=" .
REL_OPS                 = LESS_THAN_OP | LESS_THAN_EQUAL_OP
                        | GREATER_THAN_OP | GREATER_THAN_EQUAL_OP .

(* math operators *)
PLUS_OP     = "+" .
MINUS_OP    = "-" .
STAR_OP     = "*" .
SLASH_OP    = "/" .
MOD_OP      = "%" .
ADD_OPS     = PLUS_OP | MINUS_OP.
MUL_OPS     = STAR_OP | SLASH_OP | MOD_OP.
```

### Parser Grammar (Context Free)

Some definitions:
1. Expressions always return a value.
1. Statements does not return a value.
1. Statements/expressions are terminated by new line.

```
(* A statement is one line of source. *)
programm                = statement EOL { statement EOL } EOF . 
statement               = assignment
                        | or_expression .
assignment              = variable ASSIGN_OP or_expression .
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
literal_value           = BOOLEAN | INTEGER | REAL | STRING .
function_call           = IDENTIFIER LEFT_PAREN [ function_params ] RIGHT_PAREN .
function_params         = equal_expression { "," or_expression } .
```

[rust-lang]:    https://www.rust-lang.org/