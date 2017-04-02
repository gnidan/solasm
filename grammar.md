Block = '{' Item* '}'
Item =
    Identifier |
    Block |
    FunctionalExpression |
    LocalDefinition |
    FunctionalAssignment |
    Assignment |
    LabelDefinition |
    Switch |
    FunctionDefinition |
    For |
    'break' | 'continue' |
    Sub | 'dataSize' '(' Identifier ')' |
    LinkerSymbol |
    'errorLabel' | 'bytecodeSize' |
    NumberLiteral | StringLiteral | HexLiteral
Identifier = [a-zA-Z_$] [a-zA-Z_0-9]*
FunctionalExpression = Identifier '(' ( Item ( ',' Item )* )? ')'
LocalDefinition = 'let' IdentifierOrList ':=' FunctionalExpression
FunctionalAssignment = IdentifierOrList ':=' FunctionalExpression
IdentifierOrList = Identifier | '(' IdentifierList ')'
IdentifierList = Identifier ( ',' Identifier)*
Assignment = '=:' Identifier
LabelDefinition = Identifier ':'
Switch = 'switch' FunctionalExpression Case*
    ( 'default' ':' Block )?
Case = 'case' FunctionalExpression ':' Block
FunctionDefinition = 'function' Identifier '(' IdentifierList? ')'
    ( '->' '(' IdentifierList ')' )? Block
For = 'for' ( Block | FunctionalExpression)
    FunctionalExpression ( Block | FunctionalExpression) Block
Sub = 'assembly' Identifier Block
LinkerSymbol = 'linkerSymbol' '(' StringLiteral ')'
NumberLiteral = HexNumber | DecimalNumber
HexLiteral = 'hex' ('"' ([0-9a-fA-F]{2})* '"' | '\'' ([0-9a-fA-F]{2})* '\'')
StringLiteral = '"' ([^"\r\n\\] | '\\' .)* '"'
HexNumber = '0x' [0-9a-fA-F]+
DecimalNumber = [0-9]+
