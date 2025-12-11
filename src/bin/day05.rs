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
        if line_result.is_empty() {
            break;
        } 
        let dash_index = line_result.find("-").unwrap();
        let start: usize = line_result[0..dash_index].parse().unwrap();
        let end: usize = line_result[dash_index+1..].parse().unwrap();

        let mut idx = match ranges.binary_search_by_key(&start, |range| range.start) {
            Ok(i) => i,
            Err(i) => i
        };

        println!("found idx: {}", idx);

        if idx > 0 && ranges[idx - 1].overlaps_or_adjacent(start, end) {
            idx -= 1;
        }
        println!("changed idx: {}", idx);


        let mut merged = Range{start: start, end: end};

        while idx < ranges.len() && ranges[idx].overlaps_or_adjacent(merged.start, merged.end) {
            merged = merged.merge(&ranges[idx]);
            println!("changed merged {:?}", merged);
            ranges.remove(idx);
        }

        ranges.insert(idx, merged);
        println!("ranges: {:?}", ranges);
    }

    let mut total = 0;
    for range in &ranges {
        total += range.end - range.start + 1;
    }


    println!("Took {:.2?}", now.elapsed());
    println!("total: {}", total);

}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize
}

impl Range {
    fn overlaps_or_adjacent(&self, start: usize, end: usize) -> bool {
        self.start <= end + 1 || self.end >= start - 1
    }

    fn merge(&self, other: &Range) -> Range {
        Range {
            start: self.start.min(other.start),
            end: self.end.min(other.end)
        }
    }
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
