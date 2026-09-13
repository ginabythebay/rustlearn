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
    let mut prev = 1;
    let mut accum = 1;
    for _ in 2..n {
        let tmp = accum;
        accum += prev;
        prev = tmp;
    }
    return accum;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
    }
}
