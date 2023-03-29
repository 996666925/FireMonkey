use lalrpop_util::lalrpop_mod;
use quickjspp::{Context, JsValue};
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

use crate::compiler::compile;

mod ast;
mod compiler;

lalrpop_mod!(firemonkey);

fn main() -> Result<()> {
    // 解析命令行参数
    let mut args = args();
    args.next();
    let input = args.next().unwrap_or("hello.c".to_string());
    // args.next();
    // let output = args.next().unwrap();

    // 读取输入文件
    let input = read_to_string(input)?;

    // 调用 lalrpop 生成的 parser 解析输入文件
    let ast = firemonkey::CompUnitParser::new().parse(&input).unwrap();

    let js = compile(ast);

    #[cfg(debug_assertions)]
    println!("{:#?}", js);

    let context = Context::new().unwrap();
    context
        .add_callback("printlnNative", |msg: String| {
            println!("{}", &msg);
            JsValue::Undefined
        })
        .unwrap();
    context
        .eval(
            r#"
    function range(a, b, c) {
        let size = Math.abs(a - b);
        let min = a < b ? a : b;
        let step = c ? c : 1;
        return Array.from({ length: size},function(item,index) { return min + index * step } 
        )
    }
    Array.prototype.toString = function () {
        return "{" + this.join(",") + "}"
    }
    "#,
        )
        .unwrap();
    context
        .eval("function println(msg){printlnNative(msg.toString())}")
        .unwrap();

    context.eval(&js).unwrap();
    context
        .call_function("main", Vec::<JsValue>::new())
        .unwrap();

    Ok(())
}
