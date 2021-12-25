fn main() {
    let input = include_str!("../input.txt");
    let starting_pos = input
        .lines()
        .map(|l| l.rsplit_once(' ').unwrap().1.parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(dbg!(part1(&starting_pos)), 551901);
    assert_eq!(dbg!(part2(&starting_pos)), 272847859601291);
}

fn part1(starting_pos: &Vec<usize>) -> u32 {
    let mut tracks = [(1..=10).cycle(), (1..=10).cycle()];
    // advance the players to their starting field
    tracks[0].by_ref().nth(starting_pos[0] - 1).unwrap();
    tracks[1].by_ref().nth(starting_pos[1] - 1).unwrap();

    let mut dice = (1..=100).cycle().enumerate();
    let mut scores = [0u16; 2];
    for i in 0.. {
        let curr_player = i % 2;
        let advance_by = dice.by_ref().take(3).map(|(_, n)| n).sum::<usize>();
        let landing_field = tracks[curr_player].by_ref().nth(advance_by - 1).unwrap();
        scores[curr_player] += landing_field;
        if scores[curr_player] >= 1000 {
            break;
        }
    }
    let dice_rolls = dice.next().unwrap().0 as u32;
    let losing_score = scores.into_iter().min().unwrap() as u32;
    println!("Dice rolls: {}", dice_rolls);
    println!("Losing score: {}", losing_score);
    dice_rolls * losing_score
}

fn part2(starting_pos: &Vec<usize>) -> u64 {
    fn part2_recursive(
        curr_player_pos: u8,
        other_player_pos: u8,
        curr_player_score: u8,
        other_player_score: u8,
    ) -> (u64, u64) {
        if other_player_score >= 21 {
            return (0, 1);
        }
        let mut total_wins = (0, 0);
        {
            // 3 - 1 - 111
            let curr_player_next_pos = (curr_player_pos + 2) % 10 + 1;
            let wins = part2_recursive(
                other_player_pos,
                curr_player_next_pos,
                other_player_score,
                curr_player_score + curr_player_next_pos,
            );
            total_wins.0 += wins.1;
            total_wins.1 += wins.0;
        }
        {
            // 4 - 3 - 112 121 211
            let curr_player_next_pos = (curr_player_pos + 3) % 10 + 1;
            let wins = part2_recursive(
                other_player_pos,
                curr_player_next_pos,
                other_player_score,
                curr_player_score + curr_player_next_pos,
            );
            total_wins.0 += 3 * wins.1;
            total_wins.1 += 3 * wins.0;
        }
        {
            // 5 - 6 - 113 131 311 122 212 221
            let curr_player_next_pos = (curr_player_pos + 4) % 10 + 1;
            let wins = part2_recursive(
                other_player_pos,
                curr_player_next_pos,
                other_player_score,
                curr_player_score + curr_player_next_pos,
            );
            total_wins.0 += 6 * wins.1;
            total_wins.1 += 6 * wins.0;
        }
        {
            // 6 - 7 - 123 132 312 321 213 222 231
            let curr_player_next_pos = (curr_player_pos + 5) % 10 + 1;
            let wins = part2_recursive(
                other_player_pos,
                curr_player_next_pos,
                other_player_score,
                curr_player_score + curr_player_next_pos,
            );
            total_wins.0 += 7 * wins.1;
            total_wins.1 += 7 * wins.0;
        }
        {
            // 7 - 6 - 331 313 133 322 232 223
            let curr_player_next_pos = (curr_player_pos + 6) % 10 + 1;
            let wins = part2_recursive(
                other_player_pos,
                curr_player_next_pos,
                other_player_score,
                curr_player_score + curr_player_next_pos,
            );
            total_wins.0 += 6 * wins.1;
            total_wins.1 += 6 * wins.0;
        }
        {
            // 8 - 3 - 332 323 233
            let curr_player_next_pos = (curr_player_pos + 7) % 10 + 1;
            let wins = part2_recursive(
                other_player_pos,
                curr_player_next_pos,
                other_player_score,
                curr_player_score + curr_player_next_pos,
            );
            total_wins.0 += 3 * wins.1;
            total_wins.1 += 3 * wins.0;
        }
        {
            // 9 - 1 - 333
            let curr_player_next_pos = (curr_player_pos + 8) % 10 + 1;
            let wins = part2_recursive(
                other_player_pos,
                curr_player_next_pos,
                other_player_score,
                curr_player_score + curr_player_next_pos,
            );
            total_wins.0 += wins.1;
            total_wins.1 += wins.0;
        }
        total_wins
    }

    let wins = part2_recursive(starting_pos[0] as u8, starting_pos[1] as u8, 0, 0);

    std::cmp::max(wins.0, wins.1)
}
