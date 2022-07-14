use std::io;

macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parseble");
    };
}

fn main() {

    loop {
        read!(count as usize);
        if count == 0 {
            break;
        }

        if count == 1 {
            println!("  {}", count);
            println!();
            continue;
        }

        let mut j: usize = count - 1;
        let mut array = vec![vec![0; count]; count];
        let mut i = 0;

        while i <= j {
            let mut m = i;
            let mut value = i + 1;
            while m < j {
                array[i][m] = value;
                array[m][i] = value;
                array[j][m] = value;
                array[m][j] = value;
                m += 1;
            };
            array[j][j] = value;

            j -= 1;
            i += 1;
        }

        for x in 0..count  {
            for y in 0..count {
              if y == 0 || array[x][y] / 10 != 0 {
                print!("  {}", array[x][y]);
                continue;
              }



              print!("   {}", array[x][y]);

            }
            println!();
        }
        println!();
    }
}