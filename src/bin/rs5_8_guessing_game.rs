use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数");

    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("神秘数字：{}", secret_num);

    loop {
        let mut guess = String::new();
        // io::Result => OK; Err
        io::stdin().read_line(&mut guess).expect("无法读取");

        // shadow 隐藏之前的变量名
        // parse 返回 Result
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字");
                continue;
            },
        };
        println!("你猜的是：{}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("你赢了");
                break;
            },
        }
    }


}