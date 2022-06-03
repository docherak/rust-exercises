use std::io;

fn main() {
    loop {
        println!("Select n to calculate n-th Fibonacci number: ");
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line!");
        let index: u32 = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let nth_fib = fibonacci(index);
        println!("{}", nth_fib);

        break;
    }
}

fn fibonacci(n: u32) -> u32 {
    let mut n1: u32 = 0;
    let mut n2: u32 = 1;

    if (n == n1) || (n == n2) {
        return n
    }

    for _i in 1..n {
        let next = n1 + n2;
        n1 = n2;
        n2 = next;
    }
    n2
}
