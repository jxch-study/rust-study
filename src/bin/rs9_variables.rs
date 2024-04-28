// 常量永远不可变(不可用mut)，必须指明数据类型；可以出现在任何作用域
const MAX: u32 = 90;

fn main() {
    // let 是不可变变量；添加 mut 后使值可变（不可赋值为其他数据类型）
    let mut x = 1;
    x = x + 1;
    println!("x: {}", x);

    // showing 隐藏之前的变量，后续代码使用同名新变量（与数据类型无关）
    let x = "jk";
    println!("x: {}", x);

    println!("const: {}", MAX);
}