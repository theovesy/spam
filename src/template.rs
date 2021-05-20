use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn create_template(name: &str, width: u32, height: u32) {
    let template = gen_template(width, height);
    create_file(name, template);
}

fn create_file(name: &str, template: String) {
    let path = Path::new(name);
    let display = path.display();

    // Open a file in read only mode
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create template file: {}", why),
        Ok(file) => file,
    };

    // Write the string to file
    match file.write_all(template.as_bytes()) {
        Err(why) => panic!("Couldn't write to template file: {}", why),
        Ok(_) => println!("Template file successfully created as {}", display),
    };
}

fn gen_template(width: u32, height: u32) -> String {

    let line: String= format!("{:.>n$}", "\n", n = width as usize).to_owned();
    let mut template: String = "".to_owned();

    for i in 0..height {
        template.push_str(&line);
    }

    template
}

