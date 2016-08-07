use types::Type;

/*
KEYWORDS:

and	break	do	else
elseif	end	false	for
function	if	in	local
nil	not	or	repeat
return	then	true	until
while
*/

#[derive(Debug)]
pub struct AST {
    tokens: Vec<Token>,
    source: String
}

impl AST {
    fn parse(source: String) -> Self {
        AST {
            source: source,
            tokens: Vec::new(),
        }
    }
}


#[derive(Debug)]
enum Token {
    Constant(Type),
    Identifier(String),
    Symbol(String), // ?
    Equal,
    Keyword,
    InfixOp,
}

#[derive(Debug)]
enum Variable {
}

#[derive(Debug)]
enum Constant {
}

#[derive(Debug)]
enum Expression {
}

#[derive(Debug)]
enum Statement {
}