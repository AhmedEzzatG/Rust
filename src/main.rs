mod matrix;

use std::fmt::{Debug, Formatter};
use std::ops::{Add, Mul};
use matrix::Matrix;

#[derive(Clone, Default)]
struct Int {
    val: i32
}

impl Add for Int {
    type Output = Int;

    fn add(self, rhs: Self) -> Self::Output {
        Int {
            val: self.val + rhs.val
        }
    }
}

impl Mul for Int {
    type Output = Int;


    fn mul(self, rhs: Self) -> Self::Output {
        Int {
            val: self.val * rhs.val
        }
    }
}

impl Debug for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self.val.to_string())
    }
}

fn main() {
    let mut m = Matrix::new(3, 3, Int { val: 10 });
    let m2 = Matrix::identity(3, Int { val: 1 }, Int { val: 0 });
    println!("{m:?}");
    println!("{m2:?}");
    let result = m + m2;
    println!("Add result = {result:?}");

    let mut m = Matrix::new(3, 3, Int { val: 10 });
    let m2 = Matrix::identity(3, Int { val: 1 }, Int { val: 0 });
    let result = m * m2;
    println!("Mul result = {result:?}");
}