#[derive(Debug)]
struct RebootStep {
    value: bool,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    zmin: i32,
    zmax: i32,
}

fn main() {
    let reboot_steps = include_str!("../input.txt").lines().take(20).map(|l| {
        let (value, cuboid) = l.split_once(' ').unwrap();
        let mut ranges = cuboid.split(',');
        let (xmin, xmax) = ranges
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .split_once("..")
            .unwrap();
        let (ymin, ymax) = ranges
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .split_once("..")
            .unwrap();
        let (zmin, zmax) = ranges
            .next()
            .unwrap()
            .strip_prefix("z=")
            .unwrap()
            .split_once("..")
            .unwrap();
        RebootStep {
            value: value == "on",
            xmin: xmin.parse().unwrap(),
            xmax: xmax.parse().unwrap(),
            ymin: ymin.parse().unwrap(),
            ymax: ymax.parse().unwrap(),
            zmin: zmin.parse().unwrap(),
            zmax: zmax.parse().unwrap(),
        }
    });

    let mut reactor = [[[false; 101]; 101]; 101];

    for step in reboot_steps {
        println!("{:?}", step);
        let mut count = 0;
        for z in step.zmin..=step.zmax {
            for y in step.ymin..=step.ymax {
                for x in step.xmin..=step.xmax {
                    if let Some(value) = reactor
                        .get_mut((50 + z) as usize)
                        .and_then(|plane| plane.get_mut((50 + y) as usize))
                        .and_then(|row| row.get_mut((50 + x) as usize))
                    {
                        if *value != step.value {
                            *value = step.value;
                            count += 1;
                        }
                    }
                }
            }
        }
        println!("Toggled {} cubes", count);
    }

    let mut count = 0;
    for z in -50..=50 {
        for y in -50..=50 {
            for x in -50..=50 {
                let val = reactor[(50 + z) as usize][(50 + y) as usize][(50 + x) as usize];
                count += if val { 1 } else { 0 };
            }
        }
    }

    println!("Count: {}", count);
}
