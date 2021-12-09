use std::collections::{BinaryHeap, HashMap};

fn get_basin(heights: &Vec<Vec<u8>>, mut x: usize, mut y: usize) -> Option<((usize, usize), u8)> {
    let mut cell = heights[y][x];

    if cell == 9 {
        return None;
    }

    let mut i = 0;
    loop {
        cell = heights[y][x];

        let above = heights
            .get(y.wrapping_sub(1))
            .and_then(|row| Some(&row[x]))
            .unwrap_or(&u8::MAX);
        let below = heights
            .get(y + 1)
            .and_then(|row| Some(&row[x]))
            .unwrap_or(&u8::MAX);

        let left = heights[y].get(x.wrapping_sub(1)).unwrap_or(&u8::MAX);
        let right = heights[y].get(x + 1).unwrap_or(&u8::MAX);

        if *above < cell {
            y -= 1;
        } else if *below < cell {
            y += 1;
        } else if *left < cell {
            x -= 1;
        } else if *right < cell {
            x += 1;
        } else {
            return Some(((x, y), i));
        }

        i += 1;
    }
}

fn main() {
    let heights: Vec<Vec<u8>> = include_str!("../input.txt")
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let mut risk_level = 0;
    let mut basins = HashMap::<(usize, usize), _>::new();

    for (y, row) in heights.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let above = heights
                .get(y.wrapping_sub(1))
                .and_then(|row| row.get(x))
                .unwrap_or(&u8::MAX);
            // let above = heights[(y+WIDTH-1) % WIDTH][x]
            let below = heights
                .get(y + 1)
                .and_then(|row| row.get(x))
                .unwrap_or(&u8::MAX);
            let left = row.get(x.wrapping_sub(1)).unwrap_or(&u8::MAX);
            let right = row.get(x + 1).unwrap_or(&u8::MAX);

            if cell < above && cell < below && cell < left && cell < right {
                print!("\x1b[1m{}", cell);
                risk_level += *cell as u32 + 1;
                *basins.entry((x, y)).or_insert(0) += 1;
            } else {
                if let Some((pos, i)) = get_basin(&heights, x, y) {
                    *basins.entry(pos).or_insert(0) += 1;
                    print!("\x1b[48;2;{};{};{}m{}", 255, u8::MAX - u8::MAX / i, 0, cell);
                } else {
                    print!("\x1b[48;2;{};{};{}m{}", 0, 0, 0, cell);
                }
            }
            print!("\x1b[0m");
        }
        println!("");
    }

    println!("Risk level: {}", risk_level);
    let mut heap = basins.into_values().collect::<BinaryHeap<_>>();

    let top_three = [
        heap.pop().unwrap(),
        heap.pop().unwrap(),
        heap.pop().unwrap(),
    ];
    println!(
        "Top 3: {:?}, Product: {}",
        top_three,
        top_three.into_iter().product::<u32>()
    );
}
