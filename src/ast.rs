pub struct CompUnit {
    pub items: Vec<GlobalItem>,
}

pub enum GlobalItem {
    Decl(Stmt),
    FuncDef(FuncDef),
}

pub struct FuncDef {
    pub func_type: BType,
    pub ident: String,
    pub params: Vec<FuncParam>,
    pub block: Block,
}

pub struct FuncParam {
    pub btype: BType,
    pub id: String,
}

pub struct FuncCall {
    pub id: String,
    pub args: Vec<MulExp>,
}

pub struct Block {
    pub items: Vec<Stmt>,
}

pub enum Stmt {
    Return(MulExp),
    Decl(Decl),
    Assign(Assign),
    Block(Block),
    If(Box<If>),
    While(Box<While>),
    Continue,
    Break,
    Exp(Option<MulExp>),
    Foreach(Box<Foreach>),
}

pub enum PrimaryExp {
    Number(i32),
    LVal(LVal),
    RVal(RVal),
}

pub struct Decl {
    pub btype: BType,
    pub lval: LVal,
    pub exp: Option<ValValue>,
}

pub struct Assign {
    pub rval: RVal,
    pub exp: ValValue,
}

pub struct LVal {
    pub id: String,
}
pub struct RVal {
    pub id: String,
    pub indices: Vec<MulExp>,
}

pub enum ValValue {
    Exp(MulExp),
    List(Vec<ValValue>),
}
pub enum UnaryExp {
    Primary(PrimaryExp),
    Call(FuncCall),
}

pub enum MulExp {
    Unary(UnaryExp),
    MulUnary(Box<MulExp>, Op, UnaryExp),
}

pub enum Op {
    add,
    sub,
    mul,
    div,
    or,
    and,
    gt,
    ge,
    lt,
    le,
    eq,
    neq,
}

pub enum BType {
    Int,
    Float,
    Char,
    Double,
    Bool,
    Void,
    Never,
}

pub struct If {
    pub cond: MulExp,
    pub then: Stmt,
    pub else_then: Option<Stmt>,
}

pub struct While {
    pub cond: MulExp,
    pub block: Stmt,
}

pub struct Foreach {
    pub btype: BType,
    pub id: String,
    pub array: MulExp,
    pub block: Stmt,
}
