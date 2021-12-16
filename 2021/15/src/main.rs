use std::cmp::Ordering;
use std::collections::BinaryHeap;

const MAP_SIZE: usize = 100;

trait Map {
    fn get(&self, position: &(u16, u16)) -> u8;
}

impl<const MAP_WIDTH: usize, const MAP_HEIGHT: usize> Map for &[[u8; MAP_WIDTH]; MAP_HEIGHT] {
    fn get(&self, &(x, y): &(u16, u16)) -> u8 {
        self[y as usize][x as usize]
    }
}

struct TiledMap<const TILE_WIDTH: usize, const TILE_HEIGHT: usize> {
    base_tile: [[u8; TILE_WIDTH]; TILE_HEIGHT],
}

impl Map for TiledMap<MAP_SIZE, MAP_SIZE> {
    fn get(&self, &(x, y): &(u16, u16)) -> u8 {
        (self.base_tile[y as usize % MAP_SIZE][x as usize % MAP_SIZE]
            + (x / MAP_SIZE as u16 + y / MAP_SIZE as u16) as u8
            - 1)
            % 9
            + 1
    }
}

fn neighbours<const MAP_WIDTH: usize, const MAP_HEIGHT: usize>(
    &(x, y): &(u16, u16),
) -> Vec<(u16, u16)> {
    let mut neighbours = Vec::new();
    if x < (MAP_WIDTH - 1) as u16 {
        neighbours.push((x + 1, y));
    }
    if y < (MAP_HEIGHT - 1) as u16 {
        neighbours.push((x, y + 1));
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    return neighbours;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct State {
    risk: u16,
    position: (u16, u16),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then(self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_lowest_risk<M: Map, const MAP_WIDTH: usize, const MAP_HEIGHT: usize>(cavern: M) -> u16 {
    let start = (0, 0);
    // let end = Pos(2, 2);
    let end = (MAP_WIDTH as u16 - 1, MAP_HEIGHT as u16 - 1);

    let mut total_risk = [[u16::MAX; MAP_WIDTH]; MAP_HEIGHT];
    let mut visited = BinaryHeap::<State>::new();

    total_risk[start.1 as usize][start.0 as usize] = 0;
    visited.push(State {
        risk: 0,
        position: start,
    });

    let lowest_risk = loop {
        let State { risk, position } = visited.pop().unwrap();
        if position == end {
            break risk;
        }
        if risk > total_risk[position.1 as usize][position.0 as usize] {
            continue;
        }

        for (x, y) in neighbours::<MAP_WIDTH, MAP_HEIGHT>(&position) {
            let next = State {
                risk: risk + cavern.get(&(x, y)) as u16,
                position: (x, y),
            };
            if next.risk < total_risk[y as usize][x as usize] {
                visited.push(next);
                // Relaxation, we have now found a better way
                total_risk[y as usize][x as usize] = next.risk;
            }
        }
    };

    return lowest_risk;
}

fn main() {
    let cavern = <[[u8; MAP_SIZE]; MAP_SIZE]>::try_from(
        include_str!("../input.txt")
            .strip_suffix('\n')
            .unwrap()
            .split('\n')
            .map(|s| {
                <[u8; MAP_SIZE]>::try_from(
                    s.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect::<Vec<_>>(),
                )
                .unwrap()
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let lowest_risk = get_lowest_risk::<_, MAP_SIZE, MAP_SIZE>(&cavern);
    println!("Lowest risk: {}", lowest_risk);
    let lowest_risk_tiled =
        get_lowest_risk::<_, { MAP_SIZE * 5 }, { MAP_SIZE * 5 }>(TiledMap::<MAP_SIZE, MAP_SIZE> {
            base_tile: cavern,
        });
    println!("Lowest risk (tiled): {}", lowest_risk_tiled);
}
