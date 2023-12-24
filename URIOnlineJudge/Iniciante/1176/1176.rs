use std::io;

macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parseble");
    };
}

fn main() {
    read!(cases as u64);
    let mut count = 0;

    while count < cases {
        read!(fibonnaci_term as u64);
        println!("Fib({}) = {}", fibonnaci_term, fibonacci(fibonnaci_term));
        count += 1;
    }
}

fn fibonacci(sequence: u64) -> u64 {
    let mut fib: u64 = 1;
    let mut before_fib: u64;
    let mut aux: u64 = fib;
    if sequence == 0 {
        return 0;
    }

    if sequence == 1 {
        return 1;
    }

    for _count in 2..sequence {
        before_fib = fib;
        fib = fib + aux;
        aux = before_fib;
    }

    return fib;
}
