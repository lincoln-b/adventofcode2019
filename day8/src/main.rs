use std::fs;

fn main() {
    let layers = load_image("data.txt");

    // part 1

    let layer = layers
        .iter()
        .min_by_key(|layer| layer.iter().filter(|x| **x == '0').count())
        .unwrap();
    let num1 = layer.iter().filter(|x| **x == '1').count();
    let num2 = layer.iter().filter(|x| **x == '2').count();
    println!("{}", num1 * num2);

    // part 2

    let layer = combine_layers(&layers);
    print_layer(&layer);
}

fn combine_layers(layers: &Vec<Vec<char>>) -> Vec<char> {
    let mut combined_layer = vec!['2'; 150];
    for layer in layers {
        for i in 0..150 {
            if combined_layer[i] == '2' {
                combined_layer[i] = layer[i];
            }
        }
    }
    combined_layer
}

fn print_layer(layer: &Vec<char>) {
    for i in 0..6 {
        for j in 0..25 {
            if layer[i * 25 + j] == '0' {
                print!(" ");
            } else {
                print!("{}", layer[i * 25 + j]);
            }
        }
        println!();
    }
}

fn load_image(filename: &str) -> Vec<Vec<char>> {
    let data = fs::read_to_string(filename).unwrap();

    let mut layers = Vec::new();
    let mut layer = Vec::new();
    let mut i = 0;

    for c in data.chars() {
        layer.push(c);
        i = if i == 149 {
            layers.push(layer);
            layer = Vec::new();
            0
        } else {
            i + 1
        }
    }
    layers
}
