use std::{
    error,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
};

fn main() {
    // NOTE: panic marco - which immediately crash the program
    // panic!("Error crushed");

    // sample 1;
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("File not found or any other error: {:?}", error),
    // };

    let file_name = "temp.txt";
    let open_file_error_message = "Unable to open file";

    // NOTE: this and math sample is the same
    // let file = File::open(file_name).expect(open_file_error_message);
    // let file = match File::open(file_name) {
    //     Ok(f) => f,
    //     Err(_) => panic!("{}", open_file_error_message),
    // };

    let username = read_username_from_file().expect("Error");
    println!("{}", username);

    let username = read_username_from_file_v2().expect("Oops!!");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let file_name = "login.txt";

    // NOTE: we can use ? instead of use match
    let mut f = File::open(file_name)?;

    // let mut f = match f {
    //     Ok(f) => f,
    //     Err(error) => return Err(error),
    // };

    let mut content = String::new();

    // return match f.read_to_string(&mut content) {
    //     Ok(_) => Ok(content),
    //     Err(error) => Err(error),
    // };

    f.read_to_string(&mut content)?;
    return Ok(content);
}

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let file_name = "login.txt2";
    let mut username = String::new();

    File::open(file_name)?.read_to_string(&mut username)?;
    return Ok(username);
}

fn work_with_files() {
    let file_name = "temp.txt";
    let f = File::open(file_name);
    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound  => match File::create(file_name) {
                Ok(f) => f,
                Err(error) => panic!("Unable to create a file: {:?}", error),
            },
            other_error => panic!("Unable to create a file: {:?}", other_error),
        },
    };

    match OpenOptions::new()
        .write(true)
        .open(file_name)
        .unwrap()
        .write(String::from("This is some contents").as_bytes())
    {
        Ok(size) => println!("Write - Done: size = {}", size),
        Err(error) => panic!("Unable to write content to file. Error: {}", error),
    };

    let mut file_content = String::new();
    let read_result = f.read_to_string(&mut file_content);

    match read_result {
        Ok(r) => {
            println!("Done: {}", r);
            println!("Content: {}", file_content);
        }
        Err(error) => panic!("Unable to read from file. Error: {:?}", error),
    };
}

// fn a() {
//     b();
// }

// fn b() {
//     c(22);
// }

// fn c(num: i32) {
//     if num == 22 {
//         panic!("Don't pass 22!");
//     }
// }
