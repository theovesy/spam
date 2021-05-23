// mod output;
mod input;
mod template;

use std::io;

fn main() {

    let version = "0.0.1";

    let mut input = String::new();

    println!("Welcome to SPAM {}", version);
    println!("(1) Generate template file");
    println!("(2) Process text file");
    println!("Choose an option...");

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");


        if "1\n".to_string() == input {
            input_template();
            break
        } else if "2\n".to_string() == input {
            input_process();
            break
        } else {
            println!("Choose a valid option");
        }
    }
}

fn input_template() {

    println!("Choose the number of pixels");
    //template::create_template("logo", 32, 8);
}

fn input_process() {
    println!("Type the path to the file you want to process...");
}

fn process(name: String, resize_factor: u32) {


    let input = input::input_from_file(&name);

    let imgx = input[0].len() as u32;
    let imgy = input.len() as u32;

    //let t = '.';
    let b = 'b';
    let r = 'r';
    let g = 'g';
    let w = 'w';
    let a = 'a';

    let max: u8 = 255;

    // Create a new ImgBuf
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let inx: usize = x as usize;
        let iny: usize = y as usize;

        if input[iny][inx] == r {
            *pixel = image::Rgba([max, 0, 0, max]);
        } else if input[iny][inx] == g {
            *pixel = image::Rgba([0, max, 0, max]);
        } else if input[iny][inx] == b {
            *pixel = image::Rgba([0, 0, max, max]);
        } else if input[iny][inx] == w {
            *pixel = image::Rgba([max, max, max, max])
        } else if input[iny][inx] == a {
            *pixel = image::Rgba([0, 0, 0, max]);
        } else {
            *pixel = image::Rgba([0, 0, 0, 0]);
        }
    }


    // Resize the image
    let factor = resize_factor;
    let dim = imgbuf.dimensions();
    let x = factor * dim.0;
    let y = factor * dim.1;
    let resized = image::imageops::resize(&imgbuf, x, y, image::imageops::Nearest);
    // Save the image
    let output_name = format!("{}.png", &name);
    resized.save(&output_name).unwrap();

    println!("Image save successfully as {}, it is of size {}x{}", output_name, x, y);
}

