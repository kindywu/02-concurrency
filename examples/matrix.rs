use anyhow::Result;
use template::{multiply, Matrix};

fn main() -> Result<()> {
    let a = Matrix::new(2, 3, [1, 2, 3, 4, 5, 6]);
    let b = Matrix::new(3, 2, [1, 2, 3, 4, 5, 6]);
    let c = multiply(a, b)?;
    println!("{}", c);
    Ok(())
}