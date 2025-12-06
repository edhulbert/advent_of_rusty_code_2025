use std::{fs::File, io::BufRead, io::BufReader, time::Instant, collections::BTreeSet, collections::BTreeMap};
use std::collections::btree_map::Entry;

fn main() {
    // task_1();
    task_2();
}

fn task_2() {
    println!("hello from task 2!");
    let now = Instant::now();

    let buf_reader: BufReader<File> = parse();

    let mut ranges: Vec<Range> = vec!();

    for line in buf_reader.lines() {
        let line_result = line.unwrap();
        println!("{}", line_result);
        if line_result.is_empty() {
            break;
        } 
        let dash_index = line_result.find("-").unwrap();
        let start: usize = line_result[0..dash_index].parse().unwrap();
        let end: usize = line_result[dash_index+1..].parse().unwrap();

        // find first value in upper bounds that is higher than the upper bound
        let next_highest_range = ranges
            .iter()
            .find(|range: &&Range| range.upper >= end);

        
    }

    // 3, 10, 16, 12
    // 5, 14, 20, 18

    // 3
    // 5

// 10, 14
// find first set in upper bounds that is higher than upper bound
// 

    // 3, 10
    // 5, 14

    // 3, 10, 16
    // 5, 14, 20
    
    // new set is (12, 20)
    // find first set in "upper bounds" that is higher than the upper bound
    // if that lower bound is lower that our new one, we need to merge

    // do the same for upper boud

    // 3, 10
    // 5, 20

    // 3, 5
    // 3, 5, 10, 14
    // 3, 5, 10, 14, 16, 20
    // 3, 5, 10, 20

    let total = fresh_ids.len();

    println!("Took {:.2?}", now.elapsed());
    println!("total: {}", total);

}

struct Range {
    lower: usize,
    upper: usize
}

fn task_1() {

    println!("hello from task 1!");
    let now = Instant::now();
    let mut total = 0;

    let buf_reader = parse();

    let mut ids: Vec<usize> = vec!();
    let mut range_map: BTreeMap<usize, usize> = BTreeMap::new();

    // split up structures
    for line in buf_reader.lines() {
        let line_result = line.unwrap();
        if line_result.contains("-") {
            let dash_index = line_result.find("-").unwrap();
            let start: usize = line_result[0..dash_index].parse().unwrap();
            let end: usize = line_result[dash_index+1..].parse().unwrap();
            match range_map.entry(start) {
                Entry::Vacant(entry) => {
                    entry.insert(end);
                }
                Entry::Occupied(mut occupied) => {
                    if end > *occupied.get() {
                        occupied.insert(end);
                    }
                }
            }
        } else if line_result.is_empty() {
            continue;
        } else {
            ids.push(line_result.parse::<usize>().unwrap());
        }
    }

    println!("{:?}", ids);

    for id in &ids {
        for (&start, &end) in &range_map {
            if start <= *id && end >= *id {
                total += 1;
                break;
            }
        }
    }

    println!("Took {:.2?}", now.elapsed());
    println!("total: {}", total);

}

fn parse() -> BufReader<File> {
    let file = File::open("input/day05.txt").unwrap();
    BufReader::new(file)
}
