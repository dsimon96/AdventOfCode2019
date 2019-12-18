use std::io::{self, BufRead};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

const fn idx(layer: usize, row: usize, col: usize) -> usize {
    layer * WIDTH * HEIGHT + row * WIDTH + col
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().expect("No input").unwrap();
    let img: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10).expect("not a digit"))
        .collect();
    let num_layers = img.len() / (WIDTH * HEIGHT);

    // Part 1
    let layer_digit_freq: Vec<Vec<usize>> = (0..num_layers)
        .map(|i| {
            (0..10)
                .map(|d| {
                    img[idx(i, 0, 0)..idx(i + 1, 0, 0)]
                        .into_iter()
                        .filter(|&v| d == *v)
                        .count()
                })
                .collect()
        })
        .collect();

    let min_zeroes_layer = layer_digit_freq
        .iter()
        .min_by_key(|freqs| freqs[0])
        .unwrap();

    println!("{}", min_zeroes_layer[1] * min_zeroes_layer[2]);

    // Part 2
    let flattened_image: Vec<u32> = (0..WIDTH * HEIGHT)
        .map(|off| {
            (0..num_layers)
                .map(|l| img[idx(l, 0, off)])
                .filter(|&v| v != 2)
                .next()
                .unwrap()
        })
        .collect();

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            if flatten_image[idx(0, row, col)] == 0 {
                print!("\u{2588}");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
