use std::io;

fn main() {
    println!("请输入温度（以C或者F结尾）:");

    let mut s = String::new();

    match io::stdin().read_line(&mut s) {
        Ok(_) => s = s.trim().to_uppercase().to_string(),
        Err(e) => println!("error:{}", e),
    }

    let mut chars = s.chars();
    chars.next_back();

    let temperature: f64 = match chars.as_str().parse() {
        Ok(num) => num,
        _ => {
            println!("请输入数字");
            return;
        }
    };

    match s.chars().last() {
        Some('C') => {
            println!("您输入的是摄氏度：{}", temperature);
            println!("转换为华氏度是：{}", temperature * 1.8 + 32.0);
        },
        Some('F') => {
            println!("您输入的华摄氏度：{}", temperature);
            println!("转换为摄氏度是：{}", (temperature - 32.0)/1.8);
        },
        _ => println!("请以C或者F结尾"),
    }
}
