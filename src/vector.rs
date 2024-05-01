use anyhow::{anyhow, Result};
use std::{
    ops::{Add, AddAssign, Deref, Mul},
    thread,
    time::Duration,
};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        // a.len => a.data.len() (Deref trait)
        return Err(anyhow!("Dot product error: a.len != b.len"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    Ok(sum)
}

pub fn dot_product_vec<T>(a: Vec<T>, b: Vec<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        return Err(anyhow!("Dot product error: a.len != b.len"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }

    let delay = (rand::random::<u8>() as u64) * 10;
    thread::sleep(Duration::from_millis(delay));

    Ok(sum)
}
// for i in 0..a.row {
//     for j in 0..b.col {
//         for k in 0..a.col {
//             data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
//         }
//     }
// }
