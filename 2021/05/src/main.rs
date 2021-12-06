const MAP_SIZE: usize = 10;

fn main() {
    let lines = include_str!("../test.txt").lines();

    let mut map = vec![[0u8; MAP_SIZE]; MAP_SIZE];

    for line in lines {
        let nums: Vec<u16> = line
            .split(" -> ")
            .flat_map(|p| {
                p.split(",")
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<u16>>()
            })
            .collect();
        let [x1, y1, x2, y2] = <[u16; 4]>::try_from(nums).unwrap();

        // if x1 != x2 && y1 != y2 {
        //     continue;
        // }

        let y_iter: Vec<u16> = if y1 <= y2 {
            (y1..=y2).collect()
        } else {
            (y2..=y1).rev().collect()
        };

        let x_iter: Vec<u16> = if x1 <= x2 {
            (x1..=x2).collect()
        } else {
            (x2..=x1).rev().collect()
        };
        for (y, x) in y_iter.into_iter().zip(x_iter) {
            map[y as usize][x as usize] += 1;
        }
    }

    let mut overlaps = 0;

    for row in &map {
        for cell in row {
            if cell >= &2 {
                overlaps += 1;
            }
        }
    }

    for row in map {
        println!("Row: {:?}", row);
    }
    println!("Overlapping lines: {}", overlaps);
}
