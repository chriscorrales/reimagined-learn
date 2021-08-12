use std::io;

/*
INPUT
Dia 5
08 : 12 : 23
Dia 9
06 : 13 : 23

OUTPUT
3 dia(s)
22 hora(s)
1 minuto(s)
0 segundo(s)

*/

macro_rules! read_day {
    ($out:ident) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A String");
        let $out = &inner[4..].trim().parse::<u32>().expect("Parseble");
    };
}

macro_rules! read_vec {
    ($out:ident) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).unwrap();
        let hours = inner[0..2].parse::<u32>().expect("Parseble");
        let minutes = inner[5..7].parse::<u32>().expect("Parseble");
        let seconds = inner[10..12].parse::<u32>().expect("Parseble");

        let $out: [u32; 3] = [hours, minutes, seconds];
    };
}

fn main() {
    read_day!(day_start);
    read_vec!(time_start);

    read_day!(day_end);
    read_vec!(time_end);

    println!("Dia {}", day_start);

    println!(
        "{:?} : {:?} : {:?}",
        time_start[0], time_start[1], time_start[2]
    );

    println!("Dia {}", day_end);

    println!("{:?} : {:?} : {:?}", time_end[0], time_end[1], time_end[2]);
}
