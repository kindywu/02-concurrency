#[macro_use]
extern crate named_tuple;

use anyhow::{anyhow, Result};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

const THREAD_COUNT: u8 = 4;

named_tuple!(
    #[derive(Clone, Copy, Debug)]
    struct Msg(thread_num, value);
);

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..THREAD_COUNT {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let consumer = thread::spawn(|| {
        for msg in rx {
            println!(
                "receive msg from thread {}, value is {}",
                msg.thread_num(),
                msg.value()
            )
        }
    });

    consumer.join().map_err(|e| anyhow!("{:?}", e))?;

    println!("Congratulations, we finish all job!");
    Ok(())
}

fn producer(thread_num: u8, tx: Sender<Msg<u8, i32>>) -> Result<()> {
    loop {
        let value = rand::random::<i32>();
        let msg = Msg::new(thread_num, value);
        tx.send(msg)?;
        thread::sleep(Duration::from_secs(1));
        if value % 5 == 0 {
            println!("thread {} exit", thread_num);
            break;
        }
    }
    Ok(())
}
