use std::f64::INFINITY;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};



pub fn f1_panic() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let n: f64 = (5.4 + 6.6) / 0.0;  // f64::INFINITY;
    println!("{:?}", n);
}

pub fn f2_result() {
    read_username_from_file_1().unwrap();
    read_username_from_file_2().unwrap();
    read_username_from_file_3().expect("");
    read_username_from_file_4().expect("fn 4 must be correct");
}



fn greet_file() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = greeting_file_result
        .unwrap_or_else(|error| match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
    });
}

fn read_username_from_file_1() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
