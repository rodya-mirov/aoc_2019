const DAY_8: &str = include_str!("resources/8a.txt");

fn to_image(data: &str, num_rows: usize, num_cols: usize) -> Image {
    let chars = data
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();

    let layer_size = num_rows * num_cols;
    let num_chars = chars.len();
    if num_chars % layer_size != 0 {
        panic!(
            "Received {} characters, but layer size is {}*{}={}, which is not a divisor",
            chars.len(),
            num_cols,
            num_rows,
            layer_size
        );
    }

    let num_layers = num_chars / layer_size;
    let row_size = num_cols;

    let mut current_row = Vec::with_capacity(num_cols);
    let mut current_layer = Vec::with_capacity(num_rows);
    let mut layers = Vec::with_capacity(num_layers);

    for c in chars {
        current_row.push(c);
        if current_row.len() == row_size {
            current_layer.push(Row {
                pixels: current_row,
            });
            current_row = Vec::with_capacity(num_cols);

            if current_layer.len() == num_rows {
                layers.push(Layer {
                    rows: current_layer,
                });
                current_layer = Vec::with_capacity(num_rows);
            }
        }
    }

    if current_row.len() > 0 || current_layer.len() > 0 {
        panic!("Have some leftover characters, this is a code bug");
    }

    Image { layers }
}

struct Row {
    pixels: Vec<u8>,
}

struct Layer {
    rows: Vec<Row>,
}

struct Image {
    layers: Vec<Layer>,
}

fn count_pixels(layer: &Layer, val: u8) -> usize {
    layer
        .rows
        .iter()
        .map(|row| row.pixels.iter().filter(|&&b| b == val).count())
        .sum()
}

const WIDTH: usize = 25; // num_cols / row_length
const HEIGHT: usize = 6; // num_rows / col_length

pub fn a() {
    let image = to_image(DAY_8, HEIGHT, WIDTH);

    let mut fewest_zeros_so_far = None;
    let mut best_layer = None;

    for (i, layer) in image.layers.iter().enumerate() {
        let num_zeros = count_pixels(layer, 0);

        let is_better = fewest_zeros_so_far
            .as_ref()
            .map(|&old_record| old_record > num_zeros)
            .unwrap_or(true);

        if is_better {
            fewest_zeros_so_far = Some(num_zeros);
            best_layer = Some(i);
        }
    }

    let best_layer = best_layer.unwrap();
    let best_layer_ref = &image.layers[best_layer];

    let score = count_pixels(best_layer_ref, 1) * count_pixels(best_layer_ref, 2);

    println!("8a: {}", score);
}

pub fn b() {
    let image = to_image(DAY_8, HEIGHT, WIDTH);

    fn merge(top: u8, bot: u8) -> u8 {
        if top == 2 { bot } else { top }
    }

    let mut final_layer = [[2; WIDTH]; HEIGHT];

    for layer in image.layers {
        for (y, row) in layer.rows.into_iter().enumerate() {
            for (x, bot) in row.pixels.into_iter().enumerate() {
                let top = final_layer[y][x];
                let result = merge(top, bot);
                final_layer[y][x] = result;
            }
        }
    }

    println!("8b is an image:");
    println!();
    for row in final_layer.iter() {
        print!("  ");
        for c in row.iter() {
            let nice_char = if *c == 0 { ' ' } else { 'X' };
            print!("{}", nice_char);
        }
        println!();
    }
    println!();
}
