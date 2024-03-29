use std::io;

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parseble");
    };
}

macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
}

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

macro_rules! read_vec_with_filter {
  ($out:ident as $type:ty, $m:expr, $n:expr) => {
      let mut inner = String::new();
      io::stdin().read_line(&mut inner).unwrap();
      let mut $out = inner
          .trim()
          .split_whitespace()
          .map(|s| s.parse::<$type>().unwrap())
          .filter(|s| s <= $m || s <= $n)
          .collect::<Vec<$type>>();
  };
}


fn main() {
    read!(x as u32);
    read!(y as f64);
    read!(z as char);
    println!("{} {} {}", x, y, z);

    read_vec!(v as u32); // Reads space separated integers and stops when newline is encountered.

    read_boards!(boards as u32, x, x);
    println!("{:?}", v);
}
