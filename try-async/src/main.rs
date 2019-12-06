use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    let a = String::from("hi");
    let b = String::from("pon");
    thread::spawn(move || {
            tx1.send(a).unwrap();
            thread::sleep(Duration::from_secs(1));
    });

    thread::spawn(move || {
            tx.send(b).unwrap();
            thread::sleep(Duration::from_secs(1));
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
