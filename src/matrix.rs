use anyhow::{anyhow, Result};
use core::fmt;
use std::ops::{Add, AddAssign, Mul};
use std::sync::mpsc;
use std::thread;

use crate::dot_product_vec;

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T: fmt::Debug> Matrix<T> {
    pub fn new(row: usize, col: usize, data: impl Into<Vec<T>>) -> Self {
        Self {
            row,
            col,
            data: data.into(),
        }
    }
}

const NUM_THREAD: u8 = 4;

struct MsgInput<T> {
    idx: usize,
    row: Vec<T>,
    col: Vec<T>,
}

impl<T> MsgInput<T> {
    fn new(idx: usize, row: Vec<T>, col: Vec<T>) -> Self {
        Self { idx, row, col }
    }
}

struct MsgOutput<T> {
    idx: usize,
    value: T,
}

impl<T> MsgOutput<T> {
    fn new(idx: usize, value: T) -> Self {
        Self { idx, value }
    }
}

struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: fmt::Debug
        + fmt::Display
        + Copy
        + Default
        + Add<Output = T>
        + AddAssign
        + Mul<Output = T>
        + Send
        + 'static,
{
    if a.row == 0 || a.col == 0 {
        return Err(anyhow!(
            "Matrix multiply error: a.col={}, a.row={}",
            a.col,
            a.row
        ));
    }
    if b.row == 0 || b.col == 0 {
        return Err(anyhow!(
            "Matrix multiply error: b.col={}, b.row={}",
            b.col,
            b.row
        ));
    }
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: a.col != b.row"));
    }

    let mut senders = Vec::new();
    for i in 0..NUM_THREAD {
        let (tx, rx) = mpsc::channel::<Msg<T>>();
        thread::spawn(move || {
            for msg in rx {
                println!("thread {} working", i);
                let value = dot_product_vec(msg.input.row, msg.input.col)?;
                msg.sender
                    .send(MsgOutput::new(msg.input.idx, value))
                    .map_err(|e| anyhow::anyhow!("{:?}", e))?;
            }
            println!("thread {} exited", i);
            Ok::<_, anyhow::Error>(())
        });
        senders.push(tx);
    }

    let matrix_len = a.row * b.col;

    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);

    for i in 0..a.row {
        for j in 0..b.col {
            // for k in 0..a.col {
            //     data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            // }

            // let row = crate::Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            // let col = crate::Vector::new(
            //     b.data[j..]
            //         .iter()
            //         .step_by(b.col)
            //         .copied()
            //         .collect::<Vec<_>>(),
            // );
            // data[i * b.col + j] = crate::dot_product(row, col)?;

            // let row = &a.data[i * a.col..(i + 1) * a.col];
            // let col = b.data[j..]
            //     .iter()
            //     .step_by(b.col)
            //     .copied()
            //     .collect::<Vec<_>>();
            // data[i * b.col + j] = crate::dot_product_vec(row.into(), col)?;

            let row = &a.data[i * a.col..(i + 1) * a.col];
            let col = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let idx = i * b.col + j;

            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(MsgInput::new(idx, row.into(), col), tx);
            senders[idx % NUM_THREAD as usize]
                .send(msg)
                .map_err(|e| anyhow!("{:?}", e))?;
            receivers.push(rx);
        }
    }

    // drop the tx for thread to exit
    for tx in senders {
        drop(tx);
    }

    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
    }

    Ok(Matrix::new(a.row, b.col, data))
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }
            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix(row={},col={},data={})", self.row, self.col, self)?;
        Ok(())
    }
}

impl<T> Mul for Matrix<T>
where
    T: fmt::Debug
        + fmt::Display
        + Copy
        + Default
        + Add<Output = T>
        + AddAssign
        + Mul<Output = T>
        + Send
        + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix Mul Error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_display() -> anyhow::Result<()> {
        let m = Matrix::new(2, 3, [1, 2, 3, 4, 5, 6]);
        // println!("{}", m);
        assert_eq!(format!("{}", m), "{1 2 3, 4 5 6}");
        Ok(())
    }

    #[test]
    fn test_matrix_debug() -> anyhow::Result<()> {
        let m = Matrix::new(2, 3, [1, 2, 3, 4, 5, 6]);
        // println!("{:?}", m);
        assert_eq!(
            format!("{:?}", m),
            "Matrix(row=2,col=3,data={1 2 3, 4 5 6})"
        );
        Ok(())
    }

    #[test]
    fn test_a_multiply_b() -> anyhow::Result<()> {
        let a = Matrix::new(2, 3, [1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(3, 2, [1, 2, 3, 4, 5, 6]);
        let c = multiply(&a, &b)?;
        // println!("{}", c);
        assert_eq!(c.data, [22, 28, 49, 64]);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_a_multiply_b_use_mul() {
        let a = Matrix::new(2, 3, [1, 2, 3, 4]);
        let b = Matrix::new(2, 2, [1, 2, 3, 4]);
        let c = a * b;
        assert_eq!(c.data, vec![7, 10, 15, 22]);
    }

    #[test]
    fn test_a_can_not_multiply_b() {
        let a = Matrix::new(2, 3, [1, 2, 3, 4, 5, 6]);
        let b = Matrix::new(2, 2, [1, 2, 3, 4]);
        let result = multiply(&a, &b);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Matrix multiply error: a.col != b.row");
    }
}
