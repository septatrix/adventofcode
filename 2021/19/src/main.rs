use std::collections::{HashMap, HashSet};

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
                    (a, b, c)
                })
                .collect::<Vec<_>>()
        })
        .enumerate()
        .collect::<Vec<_>>();

    // set first scanner as the coordinate origin
    let mut scanner_pos = Vec::new();
    scanner_pos.push(Point(0, 0, 0));

    let mut beacons = HashSet::new();
    beacons.extend(scanners.pop().unwrap().1);

    'outer: while !scanners.is_empty() {
        for i in 0..scanners.len() {
            for orientation in 0..24u8 {
                let transformed =
                    Vec::from_iter(scanners[i].1.iter().map(|&p| rotate(p, orientation)));
                let mut diffs = HashMap::new();
                for (xb, yb, zb) in &transformed {
                    for (xa, ya, za) in &beacons {
                        // increase counter for this offset
                        *diffs.entry((xa - xb, ya - yb, za - zb)).or_insert(0) += 1
                    }
                }
                if let Some(((dx, dy, dz), _)) = diffs.into_iter().filter(|&(_, n)| n >= 12).next()
                {
                    for (x, y, z) in transformed {
                        beacons.insert((x + dx, y + dy, z + dz));
                    }
                    scanner_pos.push(Point(dx, dy, dz));
                    scanners.swap_remove(i);
                    continue 'outer;
                }
            }
        }
    }
    dbg!(beacons.len());

    let mut max_dist = 0;
    for i in 0..scanner_pos.len() {
        for j in i..scanner_pos.len() {
            let dist = Point::distance(&scanner_pos[i], &scanner_pos[j]);
            max_dist = std::cmp::max(max_dist, dist)
        }
    }

    println!("Covered distance: {}", max_dist);
}

fn rotate((x, y, z): (i16, i16, i16), variant: u8) -> (i16, i16, i16) {
    match variant {
        0 => (x, y, z),
        1 => (x, -z, y),
        2 => (x, -y, -z),
        3 => (x, z, -y),
        4 => (y, z, x),
        5 => (y, -x, z),
        6 => (y, -z, -x),
        7 => (y, x, -z),
        8 => (z, -y, x),
        9 => (z, -x, -y),
        10 => (z, y, -x),
        11 => (z, x, y),
        12 => (-x, -y, z),
        13 => (-x, -z, -y),
        14 => (-x, y, -z),
        15 => (-x, z, y),
        16 => (-y, -z, x),
        17 => (-y, -x, -z),
        18 => (-y, z, -x),
        19 => (-y, x, z),
        20 => (-z, y, x),
        21 => (-z, -x, y),
        22 => (-z, -y, -x),
        23 => (-z, x, -y),
        v => panic!("Invalid variant {}", v),
    }
}
