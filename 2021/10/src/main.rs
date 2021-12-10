use std::collections::HashMap;

fn main() {
    let code: Vec<Vec<_>> = include_str!("../input.txt")
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect();

    let score_map = HashMap::<_, _>::from_iter([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let mut points = 0;
    let mut autocomplete_scores = Vec::new();

    'lines: for line in code {
        let mut stack = Vec::new();
        for c in line {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c)
            } else {
                let last_char = stack.pop().unwrap();
                if c as u8 == last_char as u8 + 1 || c as u8 == last_char as u8 + 2 {
                    continue;
                }
                points += match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!("Unexpected char {}", c),
                };
                continue 'lines;
            }
        }
        autocomplete_scores.push(
            stack
                .iter()
                .rfold(0u64, |score, c| score * 5 + score_map.get(&c).unwrap()),
        );
    }

    let middle_index = autocomplete_scores.len() / 2;
    println!("Points: {}", points);
    println!(
        "Autocomplete {} at index {}",
        autocomplete_scores.select_nth_unstable(middle_index).1,
        middle_index
    )
}
