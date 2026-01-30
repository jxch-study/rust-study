use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("猜数游戏");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取");

    let guess: i32 = guess.trim().parse().expect("无法转换");

    match guess.cmp(&secret_number) {
        Ordering::Equal => { println!("you win") }
        Ordering::Less => { println!("Too small") }
        Ordering::Greater => { println!("Too big") }
    }
}
