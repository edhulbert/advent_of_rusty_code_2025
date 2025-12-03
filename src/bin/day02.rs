use std::{fs::File, io::{BufRead, BufReader}, time::Instant};

fn main() {
    // challenge_1();
    challenge_2();
}

fn challenge_2() {
    println!("Challenge 2!");
    let now: Instant = Instant::now();
    let mut total = 0;
    let mut amount_checked = 0;

    for split_result in parse().split(b',') {
        let range_string: Vec<u8> = split_result.unwrap();
        let (start, finish) = get_range_ends(range_string);
        for n in start..finish+1 {
            let num_bytes: Vec<usize> = n
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
        
            let size: usize = num_bytes.len(); 
    }
}

fn challenge_1() {
    println!("Challenge 1!");
    let now: Instant = Instant::now();
    let mut total = 0;
    let mut amount_checked = 0;

    for split_result in parse().split(b',') {
        let range_string: Vec<u8> = split_result.unwrap();
        let (start, finish) = get_range_ends(range_string);
        for n in start..finish+1 {
            let num_bytes: Vec<usize> = n
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(); // 1 2 1 2
        
            let size: usize = num_bytes.len(); 
            let mut is_ok = true;
            if size % 2 != 0 {
                is_ok = false;
            } else {
                let half_size = size / 2; 
                for number in 0..half_size { 
                    if num_bytes[number] != num_bytes[number+half_size] {
                        is_ok = false;
                        break;
                    }
                }
            };
            if is_ok {
                total += n;
            }
            amount_checked += 1;
        }
    }

    println!("Total: {}, amout checked: {}", total, amount_checked);
    let duration = now.elapsed();
    println!("Took {:.2?}", duration);

}

fn parse() -> BufReader<File> {
    let file = File::open("input/day02.txt").unwrap();
    BufReader::new(file)
}

fn get_range_ends(input: Vec<u8>) -> (usize, usize) {
        let string = String::from_utf8(input).unwrap();
        let dash_loc = string.find("-").unwrap();
        (string[..dash_loc].parse().unwrap(), string[dash_loc+1..].parse().unwrap())
}