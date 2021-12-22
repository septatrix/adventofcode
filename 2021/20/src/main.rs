fn get_number(img: &[Vec<u8>], x: usize, y: usize, fallback: u8) -> u16 {
    [
        img.get(y.wrapping_sub(1))
            .and_then(|row| row.get(x.wrapping_sub(1))),
        img.get(y.wrapping_sub(1)).and_then(|row| Some(&row[x])),
        img.get(y.wrapping_sub(1)).and_then(|row| row.get(x + 1)),
        img[y].get(x.wrapping_sub(1)),
        Some(&img[y][x]),
        img[y].get(x + 1),
        img.get(y + 1).and_then(|row| row.get(x.wrapping_sub(1))),
        img.get(y + 1).and_then(|row| Some(&row[x])),
        img.get(y + 1).and_then(|row| row.get(x + 1)),
    ]
    .iter()
    .map(|pixel| pixel.unwrap_or(&fallback))
    .map(|&n| n as u16)
    .reduce(|acc, pixel| (acc << 1) + pixel)
    .unwrap()
}

fn solve(iterations: u8) -> u16 {
    let (mapping_str, image_str) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let mapping = <[u8; 512]>::try_from(
        mapping_str
            .chars()
            .map(|c| (c == '#') as u8)
            .collect::<Vec<_>>(),
    )
    .unwrap();

    let mut image = image_str
        .lines()
        .map(|l| l.chars().map(|c| (c == '#') as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut filler = 0;

    for _ in 0..iterations {
        image.iter_mut().for_each(|row| row.insert(0, filler));
        image.iter_mut().for_each(|row| row.push(filler));
        image.insert(0, vec![filler; image[0].len()]);
        image.push(vec![filler; image[0].len()]);

        let mut new_image = image.iter().map(|row| row.clone()).collect::<Vec<_>>();
        for y in 0..image.len() {
            for x in 0..image[y].len() {
                let num = get_number(&image[..], x, y, filler);
                new_image[y][x] = mapping[num as usize];
            }
        }

        filler = mapping[std::iter::repeat(filler as usize)
            .take(9)
            .reduce(|acc, n| (acc << 1) + n)
            .unwrap()];
        image = new_image;
    }

    image
        .into_iter()
        .flat_map(|row| row)
        .map(|n| n as u16)
        .sum::<u16>()
}

fn main() {
    assert_eq!(dbg!(solve(2)), 5489);
    assert_eq!(dbg!(solve(50)), 19066);
}
