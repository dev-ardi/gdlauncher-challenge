use std::collections::btree_map::Entry;
use std::time::Instant;

fn main() {
    let input = include_str!("../challenge_input.txt");
    let parsed = input
        .lines()
        .map(|num| num.parse::<i128>().unwrap())
        .collect::<Vec<_>>();

    println!("================");
    println!("bruteforce_n2");

    let t0 = Instant::now();
    let collapse = bruteforce(&parsed);

    println!("total time: {}us", t0.elapsed().as_micros());
    println!(
        "time per line: {}",
        t0.elapsed().as_nanos() as usize / input.len()
    );
    println!("Tunnel would collapse {collapse} times");

    println!("================");
    println!("always_sort");
    let t0 = Instant::now();
    let collapse = always_sort(&parsed);

    println!("total time: {}us", t0.elapsed().as_micros());
    println!(
        "time per line: {}",
        t0.elapsed().as_nanos() as usize / input.len()
    );
    println!("Tunnel would collapse {collapse} times");

    println!("================");
    println!("optimized");
    let t0 = Instant::now();
    let collapse = optimized(&parsed);

    println!("total time: {}us", t0.elapsed().as_micros());
    println!(
        "time per line: {}",
        t0.elapsed().as_nanos() as usize / input.len()
    );
    println!("Tunnel would collapse {collapse} times");
}

// There's an optimization in the sort function where mostly sorted inputs are very cheap,
// For inputs that are off by one it's O(n).
fn always_sort(input: &Vec<i128>) -> usize {
    let mut running = input[0..100].to_owned();
    let mut collapse = 0;
    for (count, &num) in input[100..].iter().enumerate() {
        running.sort_unstable();
        if !exists_naive(&running, num) {
            collapse += 1;
        }
        let ix = running.binary_search(&input[count]).unwrap();

        // The count is exactly 100 less than the current index, exactly the one that we need to remove
        // We replace it with the new element we want to look. We will sort it afterwards. This
        // could be optimized,
        running[ix] = num;
    }
    return collapse;

    fn exists_naive(input: &Vec<i128>, num: i128) -> bool {
        // the input must be sorted. O(n)
        let mut p1 = 0;
        let mut p2 = 99;

        loop {
            let sum = input[p1] + input[p2];
            if sum == num {
                return true;
            } else if sum > num {
                p2 -= 1
            } else if sum < num {
                p1 += 1
            }
            if p1 == p2 {
                return false;
            }
        }
    }

    // Doesn't really work because there exists duplicates. I don't have the time to fix this
    fn exists_optimized(input: &Vec<i128>, num: i128) -> bool {
        for &x in input {
            let y = num - x;

            if y != x && input.binary_search(&y).is_ok() {
                return true;
            }
        }
        false
    }
}

fn bruteforce(input: &Vec<i128>) -> usize {
    let mut running = input[0..100].to_owned();
    let mut count = 0;
    for &num in input[100..].iter() {
        if !bruteforce_check(&running, num) {
            count += 1;
        }
        running.remove(0);
        running.push(num);
    }
    return count;

    fn bruteforce_check(input: &Vec<i128>, num: i128) -> bool {
        for (i, &x) in input.iter().enumerate() {
            for &y in &input[i + 1..] {
                if x + y == num {
                    return true;
                }
            }
        }
        false
    }
}

use std::collections::BTreeMap;

fn optimized(input: &Vec<i128>) -> usize {
    let mut running: BTreeMap<i128, usize> = BTreeMap::new();
    let mut count = 0;

    for &i in input[0..100].iter() {
        running.entry(i).and_modify(|x| *x += 1).or_insert(1);
    }
    for (index, &num) in input[100..].iter().enumerate() {
        if !optimized_check(&running, num) {
            count += 1;
        }
        match running.entry(input[index]) {
            Entry::Vacant(_) => {
                dbg!(running);
                panic!("entry for {} was empty, at {index}", input[index]);
            }
            Entry::Occupied(mut x) => {
                let val = x.get_mut();
                if *val == 1 {
                    x.remove();
                } else {
                    *val -= 1;
                }
            }
        }

        running.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }

    return count;

    fn optimized_check(input: &BTreeMap<i128, usize>, num: i128) -> bool {
        for (&x, &count) in input {
            let y = num - x;

            if y != x && input.contains_key(&y) {
                return true;
            } else if y == x && count > 1 {
                return true;
            }
        }
        false
    }
}
