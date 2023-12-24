use std::io;

macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).unwrap();
        let mut $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}

macro_rules! read_boards {
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

macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parseble");
    };
}

struct BoardAmount {
  size: u32,
  amount: u32,
}

impl BoardAmount {
  fn new(size: u32) -> BoardAmount {
    BoardAmount {size: size, amount: 1}
  }

  fn add(&self) -> Self {
    Self {
      size: self.size,
      amount: self.amount + 1,
    }
  }
}


fn main() {
    loop {
        read!(m as u32);
        read!(n as u32);

        if m == 0 && n == 0 {
            break;
        }

        read!(width as u32);
        read!(board_amount as usize);

        let mut boards_amount: Vec<BoardAmount> = Vec::with_capacity(board_amount);

        for _ in 0..board_amount {
          read!(size as u32);

          println!("{}", size);
        }

        read_boards!(boards as u32, &m, &n);

        boards.sort_unstable_by(|a, b| b.cmp(a));

        println!("{:?}", boards);
    }
}


// validação - se o a largura da tabua (width) não é compativel com uma das dimensões devemos ir para outra. Exemplo: Dimensões 6x4 e comprimento 120, nessa validação não da para por as tabuas na dimensão de 4 (4x100) pois vai sobrar 40cm (400 % 120 = 40). Não podemos mostrar impossivel pois ainda é possivel fazer pela dimensão 6 (600 % 120 = 0)