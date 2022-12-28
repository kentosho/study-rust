pub fn fib_match(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_match(n - 2) + fib_match(n - 1),
    }
}

pub fn fib_one(n: i32) -> i32 {
    fn func(a: i32, b: i32, c: i32) -> i32 {
        if c < 2 {
            return a;
        }
        func(a + b, a, c - 1)
    }
    func(1, 0, n)
}

pub fn fib_dp_simple(n: i32) -> i32 {
    let mut f1 = 1;
    let mut f2 = 0;
    let mut tmp = 0;
    for _ in 0..n - 1 {
        tmp = f1 + f2;
        f2 = f1;
        f1 = tmp
    }
    tmp
}
