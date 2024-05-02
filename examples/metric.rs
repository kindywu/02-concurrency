use std::{thread, time::Duration};

use anyhow::Result;
use concurrency::Metrics;
use rand::{rngs::ThreadRng, Rng};

const N: u8 = 2;
const M: u8 = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();
    for idx in 0..N {
        page_worker(idx, metrics.clone());
    }
    for idx in 0..M {
        thread_worker(idx, metrics.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(5));
        let data = metrics.snapshot();
        println!("{:?}", data);
    }
}

fn page_worker(idx: u8, metrics: Metrics) {
    thread::spawn(move || -> Result<()> {
        let mut rng: ThreadRng = rand::thread_rng();
        loop {
            thread::sleep(Duration::from_millis(rng.gen_range(500..5000)));
            metrics.inc(format!("req.page.{}", idx))?;
        }
    });
}

fn thread_worker(idx: u8, metrics: Metrics) {
    thread::spawn(move || {
        let mut rng: ThreadRng = rand::thread_rng();
        loop {
            thread::sleep(Duration::from_millis(rng.gen_range(100..1000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
}
