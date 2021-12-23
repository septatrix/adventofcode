fn get_number(
    buffer: &[u8],
    buffer_width: usize,
    x: usize,
    y: usize,
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
    fallback: u8,
) -> u16 {
    [
        (y - 1, x - 1),
        (y - 1, x),
        (y - 1, x + 1),
        (y, x - 1),
        (y, x),
        (y, x + 1),
        (y + 1, x - 1),
        (y + 1, x),
        (y + 1, x + 1),
    ]
    .iter()
    .map(|(y, x)| if xmin <= *x && *x < xmax && ymin <= *y && *y < ymax { buffer[*y * buffer_width + *x] } else {fallback} as u16)
    .reduce(|acc, pixel| (acc << 1) + pixel)
    .unwrap()
}

fn solve(iterations: usize, mapping: &[u8; 512], image: &Vec<Vec<u8>>) -> u16 {
    let padding = iterations + 1;
    let buffer_width = image.len() + 2 * padding;
    let buffer_height = image[0].len() + 2 * padding;

    let mut buffer = [
        vec![0u8; buffer_width * buffer_height],
        vec![0u8; buffer_width * buffer_height],
    ];

    for y in 0..image.len() {
        for x in 0..image[y].len() {
            buffer[0][(padding + y) * buffer_width + padding + x] = image[y][x];
        }
    }

    let mut filler = 0;

    for i in 0..iterations {
        for y in (padding - i - 1)..(padding + image.len() + i + 1) {
            for x in (padding - i - 1)..(padding + image[0].len() + i + 1) {
                let old_value = get_number(
                    &buffer[i % 2][..],
                    buffer_width,
                    x,
                    y,
                    padding - i,
                    padding + image[0].len() + i,
                    padding - i,
                    padding + image.len() + i,
                    filler,
                );
                let new_value = mapping[old_value as usize];
                buffer[(i + 1) % 2][y * buffer_width + x] = new_value;
            }
        }

        filler = mapping[std::iter::repeat(filler as usize)
            .take(9)
            .reduce(|acc, n| (acc << 1) + n)
            .unwrap()];
    }

    buffer[iterations % 2]
        .iter()
        .map(|&n| n as u16)
        .sum::<u16>()
}

fn main() {
    let (mapping_str, image_str) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let mapping = <[u8; 512]>::try_from(
        mapping_str
            .chars()
            .map(|c| (c == '#') as u8)
            .collect::<Vec<_>>(),
    )
    .unwrap();

    let image = image_str
        .lines()
        .map(|l| l.chars().map(|c| (c == '#') as u8).collect())
        .collect();

    // assert_eq!(dbg!(solve(2, &mapping, &image)), 35);
    // assert_eq!(dbg!(solve(50, &mapping, &image)), 3351);
    assert_eq!(dbg!(solve(2, &mapping, &image)), 5489);
    assert_eq!(dbg!(solve(50, &mapping, &image)), 19066);
}
