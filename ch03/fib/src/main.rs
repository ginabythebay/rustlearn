fn main() {
    let value = fib(6);
    println!("fib(6)={value}");
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    let mut prev = 0;
    let mut accum = 1;
    for _ in 2..n {
        let tmp = accum;
        accum += prev;
        prev = tmp;
    }
    return accum;
}
