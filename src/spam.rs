use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// ====================
// Template creation 
// ====================
pub fn create_template(name: &str, width: u32, height: u32) -> bool {
    let template = gen_template(width, height);
    create_file(name, &template);
    true
}

fn create_file(name: &str, template: &String) {
    let path = Path::new(name);

    // Open a file in read only mode
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create template file: {}", why),
        Ok(file) => file,
    };

    // Write the string to file
    match file.write_all(template.as_bytes()) {
        Err(why) => panic!("Couldn't write to template file: {}", why),
        Ok(_) => (), 
    };
}

fn gen_template(width: u32, height: u32) -> String {

    let line: String= format!("{:.>n$}", "\n", n = width as usize).to_owned();
    let mut template: String = "".to_owned();

    for _i in 0..height {
        template.push_str(&line);
    }

    template
}

// ====================
// Read input file 
// ====================
fn input_from_file(file_name: &String) -> Vec<Vec<char>> {
    let input: Vec<Vec<char>>;

    // Opening the input file
    let path = Path::new(file_name as &str);

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open input file: {}", why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read input: {}", why),
        Ok(_) => input = parse_input(&s),
    }

    input
}

fn parse_input(s: &String) -> Vec<Vec<char>> {
    let s = s.as_str();
    let lines = s.lines();
    // Getting the longuest line's length
    // and the taget image size
    let mut max_len = 0;
    let mut line_number = 0;
    for line in lines {
        line_number += 1;
        let mut line_len = 0;
        for _ch in line.chars() {
            line_len += 1;
        }
        if line_len >= max_len {
            max_len = line_len;
        }
    }
    let imgx = max_len;
    let imgy = line_number;

    let mut input = vec![vec!['.'; imgx];imgy];
    let mut x = 0;
    let mut y = 0;

    for line in s.lines() {
        for ch in line.chars() {
            input[x][y] = ch;
            y += 1;
        }
        x += 1;
        y = 0;
     }

    input
}

// ======================================
// Process input file and output image 
// ======================================
pub fn process_file(input: &String, output: &String, resize: u32) -> bool
{
    process(input, output, resize);
    true
}

fn process(input: &String, output: &String, resize_factor: u32) {

    let input = input_from_file(input);

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
    let output_name = format!("{}.png", output);
    resized.save(&output_name).unwrap();
}

