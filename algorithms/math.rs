// Loop Fibonacci
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

// Recursive Fibonacci
fn fibonacci(sequence: u64) -> u64 {
    if sequence == 0 {
        return 0;
    } else if sequence == 1 {
        return 1;
    } else {
        return fibonacci(sequence - 1) + fibonacci(sequence - 2);
    }
}
