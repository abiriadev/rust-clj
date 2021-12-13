use std::{
    error::Error,
    fs::File,
    io::{self, ErrorKind, Read},
};

const path: &str = "hello.txt1";
fn main() -> Result<(), Box<dyn Error>> {
    let f: Result<File, std::io::Error> = File::open(path);

    let f = match f {
        Ok(v) => v,
        Err(ref err) => match err.kind() {
            // ref가 하는 게 뭘까?
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", e),
            },
            other_error => panic!("can't open file: {:?}", other_error),
        },
    };

    println!("{:?}", f);

    // File::open("dsakhadskj").expect("lol.txt");

    let username = match read_username_from_file() {
        Ok(v) => v,
        Err(err) => {
            lol(&err);
            String::new()
        }
    };

    println!("username: {:?}", username);

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("lol.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn lol(err: &dyn Error) -> &dyn Error {
    println!("{:?}", err);

    // Err(err)?

    err
}
