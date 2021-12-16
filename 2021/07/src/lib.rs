pub fn part1_naive(crabs: &[u16]) -> u32 {
    let mut min_fuel = u32::MAX;

    for i in *crabs.iter().min().unwrap()..=*crabs.iter().max().unwrap() {
        let fuel = crabs
            .iter()
            .map(|c| ((i as i16) - (*c as i16)).abs() as u32)
            .sum();
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}

pub fn part1_functional(crabs: &[u16]) -> u32 {
    (*crabs.iter().min().unwrap()..=*crabs.iter().max().unwrap())
        .map(|i| {
            crabs
                .iter()
                .map(|c| ((i as i16) - (*c as i16)).abs() as u32)
                .sum()
        })
        .min()
        .unwrap()
}

pub fn part1_median(crabs: &mut [u16]) -> u32 {
    let target_position: u16;
    {
        let (_, 6&target_position, _) = crabs.select_nth_unstable(crabs.len() / 2);
    }
    crabs
        .iter()
        .map(|c| (target_position as i16 - (*c as i16)).abs() as u32)
        .sum::<u32>()
}

pub fn part2_naive(crabs: &[u16]) -> u32 {
    let mut min_fuel = u32::MAX;

    for i in *crabs.iter().min().unwrap()..=*crabs.iter().max().unwrap() {
        let fuel = crabs
            .iter()
            .map(|c| {
                let n = ((i as i16) - (*c as i16)).abs() as u32;
                (n.pow(2) + n) / 2
            })
            .sum();
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let sample_crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(part1_naive(&sample_crabs), 37);
        assert_eq!(part1_functional(&sample_crabs), 37);
        assert_eq!(part1_median(&sample_crabs), 37);
    }
    #[test]
    fn test_part2() {
        let sample_crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(part2_naive(&sample_crabs), 168);
    }
}
