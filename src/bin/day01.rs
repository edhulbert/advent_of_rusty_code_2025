use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> std::io::Result<()> {
    challenge_2()
}

fn challenge_2() -> std::io::Result<()> {
    println!("Challenge 2!");
    let now: Instant = Instant::now();

    let file = File::open("input/day01_test.txt")?;
    let buf_reader: BufReader<File> = BufReader::new(file);
    let mut pos: i16 = 50;
    let mut zeros: i16 = 0;

    for line_result in buf_reader.lines() {
        let line = line_result?;
        let (dir, amount) = line.split_at(1);
        let amount_no: i16 = amount.parse().unwrap();
        let (new_pos, num_zeros) = determine_new_pos_and_zeros(pos, amount_no, dir);
        pos = new_pos;
        zeros += num_zeros;
    }

    println!("password: {}", zeros);

    let elapsed = now.elapsed();
    println!("elapsed: {:.2?}", elapsed);
    Ok(())
}

fn determine_new_pos_and_zeros(pos: i16, a: i16, dir: &str) -> (i16, i16) {
    let (new_pos, crossings) = if dir == "L" {
        let total_pos = pos - a; // -1
        let crossings = if pos == 0 {
            (100 - total_pos) / 100 - 1
        } else {
            (100 - total_pos) / 100
        };
        (determine_negative_pos(total_pos), crossings)
    } else {
        let total_pos: i16 = pos + a;
        (determine_positive_pos(total_pos), total_pos / 100)
    };
    println!("new: {}, crossings: {}", new_pos, crossings);
    (new_pos, crossings)
}

fn determine_negative_pos(pos: i16) -> i16 {
    return if pos < 0 { 
        let num: i16 = 100 + (pos % 100);
        if num == 100 { 0 } else { num }
    } else {
        pos
    };
}

fn determine_positive_pos(pos: i16) -> i16 {
    pos.rem_euclid(100)
}
