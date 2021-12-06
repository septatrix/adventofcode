const WIDTH: usize = 12;

fn main() {
    let nums = include_str!("../input.txt")
        .lines()
        .map(|s| i16::from_str_radix(s, 2).unwrap());

    let mut bit_counts = [0i16; 12];

    for mut num in nums {
        for i in 0..WIDTH {
            bit_counts[i] += (num & 1) * 2 - 1;
            num >>= 1;
        }
    }

    let gamma_rate: u64 = bit_counts
        .iter()
        .enumerate()
        .map(|(i, &n)| ((n > 0) as u64) << i)
        .sum();

    println!("Bitcounts {:?}", bit_counts);
    println!("Gamma rate {:?}", gamma_rate);
    println!(
        "Answer: {:?}",
        gamma_rate * (gamma_rate ^ ((1 << WIDTH) - 1))
    );
}

/*

    let nums = include_str!("../input.txt").lines();

    let mut bit_counts = [0i16; 12];

    for num in nums {
        for (i, digit) in num.chars().enumerate() {
            bit_counts[WIDTH - 1 - i] += ((digit == '0') as i16) * 2 - 1;
        }
    }

    let gamma_rate: u64 = bit_counts
        .iter()
        .enumerate()
        .map(|(i, n)| ((n < &0) as u64) << i)
        .sum();

*/
