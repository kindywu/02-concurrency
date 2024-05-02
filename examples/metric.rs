use std::{thread, time::Duration};

use anyhow::Result;
use rand::{rngs::ThreadRng, Rng};
use template::Metric;

const N: u8 = 2;
const M: u8 = 4;

fn main() -> Result<()> {
    let metric = Metric::new();
    for idx in 0..N {
        page_worker(idx, metric.clone());
    }
    for idx in 0..M {
        thread_worker(idx, metric.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(3));
        let data = metric.snapshot();
        println!("{:?}", data);
    }
}

fn page_worker(idx: u8, mut metric: Metric) {
    thread::spawn(move || {
        let mut rng: ThreadRng = rand::thread_rng();
        loop {
            thread::sleep(Duration::from_millis(rng.gen_range(500..5000)));
            metric.inc(format!("thead {} req.page.1", idx));
        }
    });
}

fn thread_worker(idx: u8, mut metric: Metric) {
    thread::spawn(move || {
        let mut rng: ThreadRng = rand::thread_rng();
        loop {
            thread::sleep(Duration::from_millis(rng.gen_range(500..5000)));
            metric.inc(format!("thead {} call.thread.worker.1", idx));
        }
    });
}
