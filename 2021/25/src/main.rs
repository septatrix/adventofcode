fn main() {
    let input = include_str!("../input.txt");
    let mut seafloor = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let seafloor_height = seafloor.len();
    let seafloor_width = seafloor[0].len();

    for i in 0.. {
        let mut fixpoint = true;
        // move east
        for y in 0..seafloor_height {
            for x in 0..seafloor_width {
                if seafloor[y][x] == '>' && seafloor[y][(x + 1) % seafloor_width] == '.' {
                    seafloor[y][x] = 'E';
                    fixpoint = false;
                }
            }
        }
        for y in 0..seafloor_height {
            for x in 0..seafloor_width {
                if seafloor[y][x] == 'E' {
                    seafloor[y][x] = '.';
                    seafloor[y][(x + 1) % seafloor_width] = '>';
                }
            }
        }
        // move south
        for x in 0..seafloor_width {
            for y in 0..seafloor_height {
                if seafloor[y][x] == 'v' && seafloor[(y + 1) % seafloor_height][x] == '.' {
                    seafloor[y][x] = 'S';
                    fixpoint = false;
                }
            }
        }
        for x in 0..seafloor_width {
            for y in 0..seafloor_height {
                if seafloor[y][x] == 'S' {
                    seafloor[y][x] = '.';
                    seafloor[(y + 1) % seafloor_height][x] = 'v';
                }
            }
        }

        println!("\nAfter {} steps:", i + 1);
        for y in 0..seafloor_height {
            for x in 0..seafloor_width {
                print!("{}", seafloor[y][x]);
            }
            println!();
        }
        if fixpoint {
            return;
        }
    }
}
