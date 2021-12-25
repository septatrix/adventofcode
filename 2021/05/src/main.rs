struct LineIter {
    curr_point: (u16, u16),
    end: (u16, u16),
    finished: bool,
}

impl Iterator for LineIter {
    type Item = (u16, u16);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_point == self.end {
            if self.finished {
                return None;
            }
            self.finished = true;
            return Some(self.end);
        }
        let prev = self.curr_point;
        self.curr_point = (
            (prev.0 as i16 + (self.end.0 as i16 - prev.0 as i16).signum()) as u16,
            (prev.1 as i16 + (self.end.1 as i16 - prev.1 as i16).signum()) as u16,
        );
        Some(prev)
    }
}

const MAP_SIZE: usize = 1000;

fn main() {
    let lines = include_str!("../input.txt").lines().map(|l| {
        let [x1, y1, x2, y2] = <[u16; 4]>::try_from(
            l.split(" -> ")
                .flat_map(|p| p.split(",").map(|n| n.parse().unwrap()))
                .collect::<Vec<_>>(),
        )
        .unwrap();
        LineIter {
            curr_point: (x1, y1),
            end: (x2, y2),
            finished: false,
        }
    });

    let mut map = [[0u8; MAP_SIZE]; MAP_SIZE];

    for line in lines {
        for (x, y) in line {
            map[y as usize][x as usize] += 1;
        }
    }

    let mut overlaps = 0;

    println!("P2\n{} {}\n2", map.len(), map[0].len());

    for row in &map {
        for cell in row {
            print!(
                "{} ",
                match cell {
                    0 => 0,
                    1 => 1,
                    _ => 2,
                }
            );
            if *cell >= 2 {
                overlaps += 1;
            }
        }
        println!();
    }

    eprintln!("Overlapping lines: {}", overlaps);
}
