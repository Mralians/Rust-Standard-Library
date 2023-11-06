use std::{
    fs::{read, File, OpenOptions},
    io::{self, BufRead, BufReader, Lines, Write},
    io::{prelude::*, BufWriter},
};
fn main() {
    // Create a file and fill it with data
    let path = "./foo.txt";
    println!("Writing some data to '{}'", path);
    write_file(path, "Hello world\n").expect("Failed to write file");
    // Read entire file as a string
    let content = read_file(path).expect("Failed to read file");
    println!("The file '{}' contains:", path);
    println!("{}", content);
    // Overwrite the file
    println!("Writing new data to '{}'", path);
    write_file(path, "New content\n").expect("Failed to write file");
    let content = read_file(path).expect("Failed to read file");
    println!("The file '{}' now contains:", path);
    println!("{}", content);
    // Append data to the file
    println!("Appending data to '{}'", path);
    append_file(path, "Some more content\n").expect("Failed to append to file");
    println!("The file '{}' now contains:", path);
    // Read file line by line as an iterator
    let lines = read_file_iterator(path).expect("Failed to read file");
    for line in lines {
        let line = line.expect("Failed to read line");
        println!("{line}");
    }
    append_and_read(path, "last line in the file,goodbye").expect("Failed to read and write file");
    std::fs::remove_file(path).expect("Failed to remove file");
}
//////////////////////////////////////////////////////////////
/// FUNCTIONS
//////////////////////////////////////////////////////////////
fn write_file(path: &str, content: &str) -> io::Result<usize> {
    let file = File::create(path)?;
    let mut buf_write = BufWriter::new(file);
    let numWritten = buf_write.write(content.as_bytes())?;
    Ok(numWritten)
}
fn read_file(path: &str) -> io::Result<String> {
    let mut content = String::new();
    let file = File::open(path)?;
    let mut buf_read = BufReader::new(file);
    buf_read.read_to_string(&mut content);
    Ok(content)
}
fn read_file_iterator(path: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let mut buf_read = BufReader::new(file);
    Ok(buf_read.lines())
}
fn append_file(path: &str, content: &str) -> io::Result<()> {
    let file = OpenOptions::new().append(true).open(path)?;
    let mut buf_write = BufWriter::new(file);
    buf_write.write(content.as_bytes());
    Ok(())
}
fn append_and_read(path: &str, content: &str) -> io::Result<()> {
    let mut file_content = String::new();
    let file = OpenOptions::new().read(true).append(true).open(path)?;
    // Passing a reference of the file will not move it
    // allowing you to create both a reader and a writer
    let mut buf_write = BufWriter::new(&file);
    let mut buf_read = BufReader::new(&file);
    buf_read.read_to_string(&mut file_content)?;
    println!("[*] File before appending:\n{}", file_content);
    // Appending will shift your positional pointer
    // so you have to save and restore it
    let pos = buf_read.seek(io::SeekFrom::Start(0))?;
    buf_write.write_all(content.as_bytes())?;
    // Flushing forces the write to happen right now
    buf_write.flush()?;
    buf_read.seek(io::SeekFrom::Start(pos))?;
    buf_read.read_to_string(&mut file_content)?;
    println!("[*] File after appending:\n{}", file_content);

    Ok(())
}
