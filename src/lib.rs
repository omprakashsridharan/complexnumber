use std::ops::Add;
use std::convert::From;
use std::fmt::{Formatter, Display, Result};

#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T: Add<T, Output=T>> Add for Complex<T> {
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> From<(T,T)> for Complex<T> {
    fn from(value: (T, T)) -> Self {
        Complex{re: value.0,im: value.1}
    }
}

impl<T: Display> Display for Complex<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"{} + {}i",self.re,self.im)
    }
}

#[cfg(test)]
mod tests {
    use crate::Complex;

    #[test]
    fn complex_basics(){
        let first = Complex::new(2,4);
        let second: Complex<i32> = Complex::default();
        assert_eq!(first.re,2);
        assert_eq!(first.im,4);
        assert_eq!(second.re, second.im);
    }

    #[test]
    fn complex_addition(){
        let a = Complex::new(2,3);
        let b = Complex::new(2,3);
        let res = a + b;
        assert_eq!(res.re, 4);
        assert_eq!(res.im, 6);
    }

    #[test]
    fn complex_from(){
        let a = (12,34);
        let complex_number = Complex::from(a);
        assert_eq!(complex_number.re,12);
        assert_eq!(complex_number.im,34);
    }

    #[test]
    fn complex_display(){
        let cn = Complex::from((2,3));
        println!("{}",cn);
    }
}
