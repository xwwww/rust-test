pub(crate) use firestorm::{
    profile_fn,
    bench
};

// use firestorm::{bench, profile_fn};

fn fib(n:i32) -> i32 {
    profile_fn!(fib);

    if n == 0 {
        return 0;
    }

    let mut num1 = 0;
    let mut num2 = 0;

    for _ in 1..n {
        let tmp = num1 + num2;
        num1 = num2;
        num2 = tmp;
    }

    return num2;
}


fn test() {
    fib(20);
}

fn main() {
    bench("./", test).unwrap();
}
