use std::{fs::File, io::{BufRead, BufReader}, time::Instant};

fn main() {
    println!("Challenge 1!");
    let now: Instant = Instant::now();

    let file = File::open("input/day02.txt").unwrap();
    let buf_reader: BufReader<File> = BufReader::new(file);

    let mut total = 0;

    for split_result in buf_reader.split(b',') {
        let range = split_result.unwrap();
        let string = String::from_utf8(range).unwrap();
        let dash_loc = string.find("-").unwrap();
        let start: usize = string[..dash_loc].parse().unwrap();
        let finish: usize = string[dash_loc+1..].parse().unwrap();

        for n in start..finish+1 {
            let num_bytes: Vec<usize> = n
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(); // 1 2 1 2
        
            let size: usize = num_bytes.len(); 
            let hit: Option<()> = if size % 2 != 0 {
                None
            } else {
                let half_size = size / 2; 
                let mut is_match = true; 
                for number in 0..half_size { 
                    if num_bytes[number] != num_bytes[number+half_size] {
                        is_match = false;
                        break;
                    }
                }

                if is_match {
                    Some(())
                } else {
                    None
                }
            };
            match hit {
                Some(_) => {
                    println!("hit! {}", n);
                    total += n;
                },
                _ => println!("no hit {}", n)
            }
        }
    }

    println!("Total: {}", total);
    let duration = now.elapsed();
    println!("Took {:.2?}", duration);

}