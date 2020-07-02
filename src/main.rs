fn main() {
    let num = 10;
    println!("Fibonacci number {} is {}", num, fib_n(num))
}

fn fib_n(num: u32) -> u32 {
    // zeroth and first are fixed by definition
    if num == 0 || num == 1 {
        return num;
    }

    let mut fn2 = 0; // previous previous
    let mut fn1 = 1; // previous

    let mut fn0 = 0; // current
    for _ in 1..num {
        fn0 = fn1 + fn2; // current is sum of previous 2
        println!("{}", fn0);

        // update previous values
        fn2 = fn1;
        fn1 = fn0;
    }
    fn0
}
