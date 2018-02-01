# A Mini Virtual Machine

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
SYMBOL      =  ( CHARACTER | "_" ) { CHARACTER DIGIT | | "_" | "-" } .

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

(* // Delimiters: *)
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
LESS_THAN_OP            = "<" .
LESS_THAN_EQUAL_OP      = "<=" .
GREATER_THAN_OP         =  ">" .
GREATER_THAN_EQUAL_OP   = ">=" .

(* math operators *)
PLUS_OP     = "+" .
MINUS_OP    = "-" .
STAR_OP     = "*" .
SLASH_OP    = "/" .
MOD_OP      = "%" .
ADD_OPS     = PLUS_OP | MINUS_OP.
MUL_OPS     = STAR_OP | SLASH_OP | MOD_OP.
```