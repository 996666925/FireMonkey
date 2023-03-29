use std::fmt::format;

use crate::ast::*;

pub fn compile(ast: CompUnit) -> String {
    let mut result = String::new();

    for item in ast.items.iter() {
        result += &compileGlobalItems(item);
    }

    result
}

fn compileGlobalItems(item: &GlobalItem) -> String {
    let mut result = String::new();
    match item {
        GlobalItem::Decl(stmt) => {
            result += &compileStmt(&stmt);
        }
        GlobalItem::FuncDef(func) => {
            let params = func
                .params
                .iter()
                .map(|params| params.id.clone())
                .collect::<Vec<String>>()
                .join(",");

            result += &format!("function {}({})", func.ident, params);
            result += &compileBlock(&func.block);
        }
    }
    result
}

fn compileBlock(block: &Block) -> String {
    let mut result = String::new();
    result += "{";
    for stmt in block.items.iter() {
        result += &compileStmt(stmt);
    }
    result += "}";
    result
}

fn compileStmt(stmt: &Stmt) -> String {
    let mut result = String::new();
    match stmt {
        Stmt::Return(value) => {
            result += &format!("return {};", compileExp(value));
        }
        Stmt::Decl(Decl { btype, lval, exp }) => {
            let r#type = match btype {
                BType::Never => "",
                _ => "let ",
            };
            let exp = match exp {
                Some(ValValue) => {
                    format!("={}", &compileValValue(ValValue))
                }
                None => String::new(),
            };

            let name = &lval.id;

            result += &format!("{}{}{};", r#type, name, exp);
        }
        Stmt::Assign(Assign { rval, exp }) => {
            let arr = if (rval.indices.is_empty()) {
                String::new()
            } else {
                let str = rval
                    .indices
                    .iter()
                    .map(|exp| compileExp(exp))
                    .collect::<Vec<String>>()
                    .join(",");
                format!("[{}]", str)
            };
            result += &format!("{}{}={};", rval.id, arr, &compileValValue(exp));
        }
        Stmt::Block(block) => {
            result += &format!("{{{}}}", &compileBlock(block));
        }
        Stmt::If(value) => {
            result += &compileIf(value);
        }
        Stmt::While(value) => {
            result += &compileWhile(value);
        }
        Stmt::Continue => {
            result += "continue;";
        }
        Stmt::Break => {
            result += "break;";
        }
        Stmt::Exp(exp) => {
            if let Some(exp) = exp {
                result += &compileExp(exp);
            }
        }
        Stmt::Foreach(foreach) => {
            result += &compileForeach(foreach);
        }
    }
    result
}

fn compileExp(exp: &MulExp) -> String {
    let mut result = String::new();
    match exp {
        MulExp::Unary(unary) => {
            let value = compileUnary(unary);
            result += &value;
        }
        MulExp::MulUnary(mul, op, unary) => {
            let op = matchOp(op);
            let value = compileUnary(unary);
            result += &format!("{}{}{}", compileExp(mul), op, value);
        }
    }
    result
}

fn matchOp(op: &Op) -> &str {
    match op {
        Op::add => "+",
        Op::sub => "-",
        Op::mul => "*",
        Op::div => "/",
        Op::or => "||",
        Op::and => "&&",
        Op::gt => ">",
        Op::ge => ">=",
        Op::lt => "<",
        Op::le => "<=",
        Op::eq => "==",
        Op::neq => "!=",
    }
}

fn matchPrimary(primary: &PrimaryExp) -> String {
    match primary {
        PrimaryExp::Number(num) => num.to_string(),
        PrimaryExp::LVal(val) => val.id.clone(),
        PrimaryExp::RVal(val) => compileRVal(val),
    }
}

fn compileRVal(rval: &RVal) -> String {
    let mut result = String::new();
    result += &rval.id;
    if !rval.indices.is_empty() {
        let arr = rval
            .indices
            .iter()
            .map(|exp| compileExp(exp))
            .collect::<Vec<String>>()
            .join("][");
        result += &format!("[{}]", arr);
    }

    result
}

fn compileIf(value: &If) -> String {
    let mut result = String::new();
    result += &format!("if({})", &compileExp(&value.cond));
    if let Stmt::Block(block) = &value.then {
        result += &compileBlock(block);
    }

    if let Some(Stmt::Block(block)) = &value.else_then {
        result += &format!("else{}", &compileBlock(block));
    }
    if let Some(Stmt::If(r#if)) = &value.else_then {
        result += &format!("else {}", &compileIf(r#if));
    }

    result
}

fn compileWhile(value: &While) -> String {
    let mut result = String::new();
    if let Stmt::Block(block) = &value.block {
        result += &format!(
            "while({}){}",
            &compileExp(&value.cond),
            &compileBlock(&block)
        );
    }

    result
}

fn compileForeach(value: &Foreach) -> String {
    let mut result = String::new();
    if let Stmt::Block(block) = &value.block {
        let arr = compileExp(&value.array);
        let arr = &arr[0..arr.len() - 1];

        result += &format!("for(let {} of {}){}", value.id, arr, &compileBlock(&block));
    }

    result
}

fn compileUnary(value: &UnaryExp) -> String {
    let mut result = String::new();
    match value {
        UnaryExp::Primary(primary) => {
            result += &matchPrimary(primary);
        }
        UnaryExp::Call(func) => {
            let args = func
                .args
                .iter()
                .map(|exp| compileExp(exp))
                .collect::<Vec<String>>()
                .join(",");

            result += &format!("{}({});", func.id, args);
        }
    }

    result
}

fn compileValValue(list: &ValValue) -> String {
    let mut result = String::new();
    match list {
        ValValue::Exp(exp) => {
            result += &compileExp(exp);
        }
        ValValue::List(exp) => {
            let list = exp
                .iter()
                .map(|value| compileValValue(value))
                .collect::<Vec<String>>()
                .join(",");
            result += &format!("[{}]", list);
        }
    }
    result
}
