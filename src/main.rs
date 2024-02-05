use std::io;
use indicatif::ProgressBar;
use std::thread;
use std::time::Duration;

fn main() {
    let mut input_string = String::new();
    println!("Welcome to THE weather app.\nPlease select your location: ");
    io::stdin().read_line(&mut input_string).unwrap();
    println!("\nYour location: {input_string}");

    println!("\nCalculating temperature...");
    let pb1 = ProgressBar::new(100);
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(50));
        pb1.inc(1);
    }
    println!("\n");
    thread::sleep(Duration::from_millis(383));

    println!("Getting Data...");
    let pb2 = ProgressBar::new(100);
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(118));
        pb2.inc(1);
    }
    println!("\n");
    thread::sleep(Duration::from_millis(528));

    println!("\nCross-checking Data...");
    let pb3 = ProgressBar::new(100);
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(195));
        pb3.inc(1);
    }
    println!("\n");
    thread::sleep(Duration::from_millis(3764));

    println!("\nidk look outside");
}