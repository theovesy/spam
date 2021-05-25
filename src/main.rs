// mod output;
mod input;
mod template;

use std::io;

fn main() {

    let version = "0.0.1";


    println!("Welcome to SPAM {}", version);
    println!("(1) Generate template file");
    println!("(2) Process text file");

    loop {
        match &get_console_input("Choose and operation...")[..] {
            "1\n" => {
                input_template();
                break
            },
            "2\n" => {
                input_process();
                break
            },
            _ => println!("Option is not valid!"),
        }
    }
}

fn input_template() {

    println!("Choose the number of pixels");
    let width: u32;
    let height: u32;

    loop {
        width = match get_console_input("Width...").trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    loop {
        height = match get_console_input("Height...").trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    let name = get_console_input("Name the file...");
    let name = name.trim();

    template::create_template(&name, width, height);
}

fn input_process() {
    let path = get_console_input("Type the path to the file you want to process...");
    let path = path.trim();

    let resize_factor: u32;

    loop {
        resize_factor = match get_console_input("How much should I resize the image ? \n (for example : 2 will make the image twice as big)...").trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }

    process(path, resize_factor);
}

fn get_console_input(message: &str) -> String {
    let mut input = String::new();
    println!("{}", message);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn process(name: &str, resize_factor: u32) {


    let input = input::input_from_file(name);

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
    let output_name = format!("{}.png", name);
    resized.save(&output_name).unwrap();

    println!("Image save successfully as {}, it is of size {}x{}", output_name, x, y);
}

