use std::io;

// https://www.urionlinejudge.com.br/judge/pt/problems/view/1061

macro_rules! read_day {
    ($out:ident) => {
        let mut inner = String::new();
        io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner[4..].trim().parse::<u32>().expect("Parseble");
    };
}

macro_rules! read_time {
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
    read_time!(time_start);

    read_day!(day_end);
    read_time!(time_end);

    let seconds_start = convert_to_seconds(&time_start, day_start);
    let seconds_end = convert_to_seconds(&time_end, day_end);

    let mut diff_sec = seconds_end - seconds_start;

    println!("{} dia(s)", convert_time(&mut diff_sec, 86400));
    println!("{} hora(s)", convert_time(&mut diff_sec, 3600));
    println!("{} minuto(s)", convert_time(&mut diff_sec, 60));
    println!("{} segundo(s)", diff_sec);
}

fn convert_time(diff_sec: &mut u32, convert_number: u32) -> u32 {
    let converted = *diff_sec / convert_number;
    *diff_sec = *diff_sec - (converted * convert_number);
    return converted;
}

fn convert_to_seconds(time: &[u32; 3], day: u32) -> u32 {
    return (time[1] * 60) + (time[0] * 3600) + (day * 86400) + time[2];
}
