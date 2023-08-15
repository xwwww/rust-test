use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数字!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("请输入数字！");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("输入失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("你输入的数字：{guess}");

        println!("随机数是：{secret_number}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("你赢了！");
                break
            }
        }
    }
}
