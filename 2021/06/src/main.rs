use std::collections::HashMap;

// TODO add deque solution
// TODO add matrix solution (eigenvalues etc.)

fn main() {
    let fishes: Vec<u8> = include_str!("../input.txt")
        .split(",")
        .map(|n| n.replace("\n", "").parse().unwrap())
        .collect();

    println!("#Fishes: {}", array(256, fishes.clone()));
    println!("#Fishes: {}", naive(80, fishes.clone()));
    println!("#Fishes: {}", hash_map(256, fishes));
}

pub fn naive(days: usize, mut fishes: Vec<u8>) -> usize {
    for _ in 0..days {
        fishes = fishes
            .iter()
            .flat_map(|&f| if f == 0 { vec![6, 8] } else { vec![f - 1] })
            .collect();
    }

    return fishes.len();
}

pub fn hash_map(days: usize, fishes: Vec<u8>) -> usize {
    let mut fish_map: HashMap<u8, usize> = HashMap::new();
    for fish in fishes {
        *fish_map.entry(fish).or_insert(0) += 1;
    }

    for _ in 0..days {
        fish_map = fish_map.iter().fold(HashMap::new(), |mut map, (&age, &n)| {
            if age == 0 {
                *map.entry(6).or_insert(0) += n;
                *map.entry(8).or_insert(0) += n;
            } else {
                *map.entry(age - 1).or_insert(0) += n;
            }
            map
        });
    }

    return fish_map.values().sum::<usize>();
}

pub fn array(days: usize, fishes: Vec<u8>) -> usize {
    let mut fish_cycle = [0usize; 9];
    for fish in fishes {
        fish_cycle[fish as usize] += 1;
    }
    for i in 0..days {
        fish_cycle[(i + fish_cycle.len() - 2) % fish_cycle.len()] +=
            fish_cycle[i % fish_cycle.len()];
    }
    return fish_cycle.iter().sum::<usize>();
}
