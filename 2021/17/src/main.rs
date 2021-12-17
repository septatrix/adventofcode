use std::cmp::max;

fn main() {
    let (x_spec, y_spec) = include_str!("../input.txt")
        .strip_prefix("target area: ")
        .and_then(|s| s.strip_suffix('\n'))
        .and_then(|s| s.split_once(", "))
        .unwrap();
    let (x_min, x_max): (i32, i32) = x_spec
        .strip_prefix("x=")
        .and_then(|s| s.split_once(".."))
        .and_then(|(min, max)| Some((min.parse().unwrap(), max.parse().unwrap())))
        .unwrap();
    let (y_min, y_max): (i32, i32) = y_spec
        .strip_prefix("y=")
        .and_then(|s| s.split_once(".."))
        .and_then(|(min, max)| Some((min.parse().unwrap(), max.parse().unwrap())))
        .unwrap();

    println!(
        "target area: x={}..{}, y={}..{}",
        x_min, x_max, y_min, y_max
    );

    let mut highest_y = 0;
    let mut hitting_velocities = 0;

    for x_vel in 0..=x_max {
        for y_vel in y_min..=-y_min {
            let mut x = 0i32;
            let mut y = 0i32;
            let mut curr_x_vel = x_vel;
            let mut curr_y_vel = y_vel;
            while x <= x_max && y >= y_min {
                x += curr_x_vel;
                y += curr_y_vel;
                curr_x_vel -= (curr_x_vel > 0) as i32;
                curr_y_vel -= 1;
                if (x_min..=x_max).contains(&x) && (y_min..=y_max).contains(&y) {
                    hitting_velocities += 1;
                    highest_y = max(highest_y, (y_vel * (y_vel + 1)) / 2);
                    break;
                }
            }
        }
    }

    println!("Highest y: {}", highest_y);
    println!("Hitting velocities: {}", hitting_velocities);
}
