use std::time::Instant;

fn main() {
    let input = include_str!("../challenge_input.txt");
    let parsed = input
        .lines()
        .map(|num| num.parse::<u128>().unwrap())
        .collect::<Vec<_>>();

    let t0 = Instant::now();
    let collapse = always_sort(&parsed);
    println!("total time: {}us", t0.elapsed().as_micros());
    println!(
        "time per line: {}ns",
        t0.elapsed().as_nanos() / input.len() as u128
    );
    if collapse.len() == 0 {
        println!("Tunnel is safe");
    } else {
        println!("Tunnel would collapse {} times", collapse.len());
    }
}

// There's an optimization in the sort function where mostly sorted inputs are very cheap,
// For inputs that are off by one it's O(n).
fn always_sort(input: &Vec<u128>) -> Vec<usize> {
    let mut running = input[0..100].to_owned();
    let mut collapse = Vec::new();
    for (count, &num) in input[100..].iter().enumerate() {
        running.sort_unstable();
        if !exists_naive(&running, num) {
            collapse.push(count + 100);
        }
        let ix = running.binary_search(&input[count]).unwrap();

        // The count is exactly 100 less than the current index, exactly the one that we need to remove
        // We replace it with the new element we want to look. We will sort it afterwards. This
        // could be optimized,
        running[ix] = num;
    }
    return collapse;
}

fn exists_naive(input: &Vec<u128>, num: u128) -> bool {
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
