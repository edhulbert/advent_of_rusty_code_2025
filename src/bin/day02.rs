use std::{fs::File, io::{BufRead, BufReader}, time::Instant};

fn main() {
    challenge_1();
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
        
            if has_repeating_pattern(&num_bytes) {
                total += n;
            }
            amount_checked += 1;
            
            // get factors of the size 
            // e.g.
            // size=9 -> 1,3
            // size=15 -> 1,3,5

            // starting at i = 0
            // for each multiple, number of potentials = num_size/multiple 
            // for size=15
            // check 15/5 = 3 points to check
            // starting 0, 5, 10
            // finishing 4, 9, 14


        }
    }
    println!("Took {:.2?}", now.elapsed());
    println!("Total: {}, amout checked: {}", total, amount_checked);
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

    println!("Took {:.2?}", now.elapsed());
    println!("Total: {}, amout checked: {}", total, amount_checked);

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

fn has_repeating_pattern(number: &Vec<usize>) -> bool {
    let len = number.len();
    for pattern_len in 1..len { 
        if len % pattern_len == 0 {
            let is_repeating = (pattern_len..len)
                .all(|i| number[i] == number[i % pattern_len]);
            
            if is_repeating {
                return true; 
            }
        }
    }
    false
}
