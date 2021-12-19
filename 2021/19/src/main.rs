use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Point(i16, i16, i16);

impl Point {
    fn distance(&Point(x1, y1, z1): &Self, &Point(x2, y2, z2): &Self) -> u16 {
        [x1 - x2, y1 - y2, z1 - z2]
            .iter()
            .map(|n| n.abs() as u16)
            .sum()
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut scanners = input
        .split_terminator("\n\n")
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|p| {
                    let [a, b, c] = <[i16; 3]>::try_from(
                        p.split(',').map(|n| n.parse().unwrap()).collect::<Vec<_>>(),
                    )
                    .unwrap();
                    Point(a, b, c)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // set first scanner as the coordinate origin
    let mut scanner_pos = Vec::new();
    scanner_pos.push(Point(0, 0, 0));

    let mut beacons = HashSet::new();
    beacons.extend(scanners.pop().unwrap());

    while !scanners.is_empty() {
        for (i, scanner) in scanners.clone().iter().enumerate().rev() {
            for orientation in 0..24u8 {
                let transformed = Vec::from_iter(scanner.iter().map(|p| rotate(p, orientation)));
                let mut diffs = HashMap::new();
                for Point(xb, yb, zb) in &transformed {
                    for Point(xa, ya, za) in &beacons {
                        // increase counter for this offset
                        *diffs.entry((xa - xb, ya - yb, za - zb)).or_insert(0) += 1
                    }
                }
                if let Some(((dx, dy, dz), _)) = diffs.into_iter().filter(|&(_, n)| n >= 12).next()
                {
                    for Point(x, y, z) in transformed {
                        beacons.insert(Point(x + dx, y + dy, z + dz));
                    }
                    scanner_pos.push(Point(dx, dy, dz));
                    scanners.swap_remove(i);
                    break;
                }
            }
        }
    }
    dbg!(beacons.len());

    let mut max_dist = 0;
    for i in 0..scanner_pos.len() {
        for j in i..scanner_pos.len() {
            let dist = Point::distance(&scanner_pos[i], &scanner_pos[j]);
            max_dist = max_dist.max(dist)
        }
    }

    println!("Covered distance: {}", max_dist);
}

fn rotate(&Point(x, y, z): &Point, variant: u8) -> Point {
    match variant {
        0 => Point(x, y, z),
        1 => Point(x, -z, y),
        2 => Point(x, -y, -z),
        3 => Point(x, z, -y),
        4 => Point(y, z, x),
        5 => Point(y, -x, z),
        6 => Point(y, -z, -x),
        7 => Point(y, x, -z),
        8 => Point(z, -y, x),
        9 => Point(z, -x, -y),
        10 => Point(z, y, -x),
        11 => Point(z, x, y),
        12 => Point(-x, -y, z),
        13 => Point(-x, -z, -y),
        14 => Point(-x, y, -z),
        15 => Point(-x, z, y),
        16 => Point(-y, -z, x),
        17 => Point(-y, -x, -z),
        18 => Point(-y, z, -x),
        19 => Point(-y, x, z),
        20 => Point(-z, y, x),
        21 => Point(-z, -x, y),
        22 => Point(-z, -y, -x),
        23 => Point(-z, x, -y),
        v => panic!("Invalid variant {}", v),
    }
}
