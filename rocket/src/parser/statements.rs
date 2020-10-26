
pub enum Statement {
    Let(String, Expr),
    Return(Expr),
    Eval(Expr),
}

pub enum Expr {
    Null,
    Undefined,
    Nan,
    Boolean(bool),
    Number(f64),
    String(String),
    Unop {
        value: Expr,
        oper: Oper,
    },
    Biop {
        a: Expr,
        b: Expr,
        oper: Oper,
    },
}

pub enum Oper {
    Plus,
    Minus,
    Div,
    Mult,
}
