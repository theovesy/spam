// mod output;
mod input;
mod template;

fn main() {

    // template::create_template("logo", 32, 8);

    let input_file = "logo".to_string();

    let input = input::input_from_file(input_file);

    let imgx = input[0].len() as u32;
    let imgy = input.len() as u32;

    let t = '.';
    let b = 'b';
    let r = 'r';
    let g = 'g';
    let w = 'w';
    let a = 'a';

/*
    let input = [[t, b, t, b, t, b, t, b],
                [t, t, r, b, b, r, t, t],
                [t, r, b, b, b, b, r, t],
                [r, b, g, b, b, g, b, r],
                [r, b, g, b, b, g, b, r],
                [t, r, b, g, g, b, r, t],
                [t, t, r, b, b, r, t, t],
                [t, t, t, r, r, t, t, t]];
*/


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

    let name = "output.png";

    // Resize the image
    let factor = 50;
    let dim = imgbuf.dimensions();
    let x = factor * dim.0;
    let y = factor * dim.1;
    let resized = image::imageops::resize(&imgbuf, x, y, image::imageops::Nearest);
    // Save the image
    resized.save(name).unwrap();

    println!("image save successfully as {}, it is of size {}x{}", name, x, y);

    //output::save(&imgbuf, "output.png", 100);
}
