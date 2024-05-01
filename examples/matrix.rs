use anyhow::Result;
use template::{multiply, Matrix};

fn main() -> Result<()> {
    let a = Matrix::new(3, 3, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let b = Matrix::new(3, 3, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let c = multiply(&a, &b)?;
    println!("{}", c);
    Ok(())
}
