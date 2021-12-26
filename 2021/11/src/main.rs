fn main() {
    let mut octopi = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| n.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total_flashes = 0;

    for step in 0.. {
        let mut flashing_octopi = 0;
        for row in octopi.iter_mut() {
            for energy in row.iter_mut() {
                if *energy > 9 {
                    *energy = 0;
                    flashing_octopi += 1;
                };
                *energy += 1;
            }
        }

        if flashing_octopi == octopi.len() * octopi[0].len() {
            println!("Synchronized after {} steps", step);
            break;
        }

        if step == 100 {
            println!("{} flashes after {} steps", total_flashes, step);
        }

        loop {
            let prev_flashes = total_flashes;
            for y in 0..octopi.len() {
                for x in 0..octopi[0].len() {
                    if octopi[y][x] > 9 && octopi[y][x] < 20 {
                        // println!("({}, {}) flashed {}", x, y, octopi[y][x]);
                        octopi[y][x] = 20;
                        total_flashes += 1;
                        for j in y.saturating_sub(1)..(y + 2).min(octopi.len()) {
                            for i in x.saturating_sub(1)..(x + 2).min(octopi[0].len()) {
                                octopi[j][i] += 1;
                            }
                        }
                    }
                }
            }
            if total_flashes == prev_flashes {
                break;
            }
        }
    }
}
