use std::{io::{Write, Read}, fs::File};

pub fn advfs_test(){
    create_file_option().unwrap();
    create_file();
    println!("{}", read_file().unwrap());
    write_file_macro();
    println!("{}", read_file().unwrap());
    write_file();
    println!("{}", read_file().unwrap());
    get_metadata();
}

fn get_metadata(){
    let f = File::open("test.txt").unwrap();
    let m = f.metadata().unwrap();
    println!("{:#?}", m.created().unwrap())
}

fn write_file_macro(){
    let mut f = std::fs::File::options().write(true).open("test.txt").unwrap();
    f.set_len(0).unwrap();
    write!(&mut f, "{}", 23).unwrap();
    writeln!(&mut f, "{}", 23).unwrap();
    write!(&mut f, "{}", 23).unwrap()
}

fn write_file(){
    let mut f = std::fs::File::options().write(true).open("test.txt").unwrap();
    f.set_len(0).unwrap();
    f.write_all(b"Sup").unwrap();
}

fn read_file() -> std::io::Result<String>{
    let mut f = std::fs::File::open("test.txt")?;
    let mut str = String::new();
    f.read_to_string(&mut str)?;
    Ok(str)
}

fn create_file_option() -> std::io::Result<()>{
    let mut f = std::fs::File::create("test.txt")?;
    f.write_all(b"Hello from Rust")?;
    Ok(())
}

fn create_file(){
    let mut f = std::fs::File::create("test1.txt").unwrap();
    f.write_all(b"Hello from Rust").unwrap();
}