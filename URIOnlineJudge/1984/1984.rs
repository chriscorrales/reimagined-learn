use std::io;

macro_rules! read {
    ($out:ident) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().chars().rev().collect::<String>();
    };
}

fn main() {
    read!(invet_string);
    println!("{}", invet_string);
}
