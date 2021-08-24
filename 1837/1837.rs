// https://www.urionlinejudge.com.br/judge/pt/problems/view/1837
use std::io;

macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}

fn main() {
    read_vec!(v as f32);

    println!("{} {}", (v[0] / v[1]).floor(), (v[0] % v[1]));
}
