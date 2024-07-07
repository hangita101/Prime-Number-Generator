use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;
use std::thread;
use std::thread::JoinHandle;

fn one_thread(low: u64, high: u64, path: &String) {
    let _f = File::create(path).expect("Error in creating file");

    let mut data_file = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("cannot open file");

    let mut _str = String::new();
    for i in low..high {
        if primes::is_prime(i) {
            _str = i.to_string() + "\n";
            data_file.write(_str.as_bytes()).expect("write failed");
        }
    }
}

fn append_to_file(file_name: Vec<String>) -> io::Result<()> {
    let mut output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("Final.txt")
        .expect("Error while opening file");

    for i in file_name {
        let mut file = File::open(i).expect("Error");

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        output_file.write_all(buffer.as_bytes())?;

        //        output_file.write_all(b"\n")?;
    }

    Ok(())
}

fn delete_files(file_name: Vec<String>) -> io::Result<()> {
    for i in file_name {
        match fs::remove_file(i.clone()) {
            Ok(()) => continue,
            Err(e) => eprint!("Error While deleting file {} : {}", i, e),
        }
    }
    Ok(())
}

fn run_multiple_thread(low: u64, high: u64, count: u64) {
    let mut vectt: Vec<JoinHandle<()>> = Vec::new();

    let mut file_name: Vec<String> = Vec::new();

    let mut _str = String::new();

    let diff = (high - low) / (count as u64);

    for i in 1..count + 1 {
        _str = "foo".to_string() + &i.to_string() + ".txt";
        file_name.push(_str.clone());
        vectt.push(thread::spawn(move || {
            one_thread(low + (i - 1) * diff, low + diff * i, &_str);
        }));
    }

    for i in vectt {
        i.join().unwrap();
    }
    append_to_file(file_name.clone()).expect("Error While appending to a file");
    delete_files(file_name).expect("Fail to delete file");
}

fn main() {
    println!("HELLO");
    let mut low = String::new();
    let mut high = String::new();
    let mut count: String = String::new();
    println!("Low: ");

    io::stdout().flush().expect("Error flushing stdout");

    io::stdin()
        .read_line(&mut low)
        .expect("Error while reading");

    println!("High: ");

    io::stdout().flush().expect("Error flushing stdout");

    io::stdin()
        .read_line(&mut high)
        .expect("Error while reading");

    println!("Count: ");
    io::stdin()
        .read_line(&mut count)
        .expect("Error while reading");

    let low: u64 = low.trim().parse().expect("Error parsing low value");
    let high: u64 = high.trim().parse().expect("Error parsing high value");
    let count: u64 = count.trim().parse().expect("Error parsing high value");

    if low > high {
        print!("Low > High Error");
        return;
    }

    run_multiple_thread(low, high, count);

    println!("Done");
}
