use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Fraction {
    pub num: i64, // numerator
    pub den: u32, // denominator
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

impl Fraction {
    fn new(a: i64, b: u32) -> Self {
        if b == 0 {
            panic!("Denominator cannot be zero");
        }
        let c = gcd(a.abs() as u32, b);
        Self {
            num: a / c as i64,
            den: b / c,
        }
    }

    pub fn eval(&self) -> f64 {
        self.num as f64 / self.den as f64
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.num * rhs.den as i64 + rhs.num * self.den as i64,
            self.den * rhs.den,
        )
    }
}

impl<T> Add<T> for Fraction
where
    T: Into<i64>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Fraction::new(self.num + rhs.into() * self.den as i64, self.den)
    }
}

impl Add<Fraction> for u32 {
    type Output = Fraction;

    fn add(self, rhs: Fraction) -> Self::Output {
        Fraction::new(self as i64 * rhs.den as i64 + rhs.num, rhs.den)
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Fraction::new(
            self.num * rhs.den as i64 - rhs.num * self.den as i64,
            self.den * rhs.den,
        )
    }
}

impl<T> Sub<T> for Fraction
where
    T: Into<i64>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Fraction::new(self.num - rhs.into() * self.den as i64, self.den)
    }
}

impl Sub<Fraction> for u32 {
    type Output = Fraction;

    fn sub(self, rhs: Fraction) -> Self::Output {
        Fraction::new(self as i64 * rhs.den as i64 - rhs.num, rhs.den)
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl<T> Mul<T> for Fraction
where
    T: Into<i64>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Fraction::new(self.num * rhs.into(), self.den)
    }
}

impl Mul<Fraction> for u32 {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Self::Output {
        Fraction::new(self as i64 * rhs.num, rhs.den)
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let a = if self.num < 0 && rhs.num < 0 || self.num > 0 && rhs.num > 0 {
            self.num.abs()
        } else {
            self.num
        };
        Fraction::new(a * rhs.den as i64, self.den * rhs.num.abs() as u32)
    }
}

impl<T> Div<T> for Fraction
where
    T: Into<i64>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let a = if self.num < 0 && rhs < 0 || self.num > 0 && rhs > 0 {
            self.num.abs()
        } else {
            self.num
        };
        Fraction::new(a, self.den * rhs.abs() as u32)
    }
}

impl Div<Fraction> for u32 {
    type Output = Fraction;

    fn div(self, rhs: Fraction) -> Self::Output {
        let sign = if rhs.num < 0 { -1 } else { 1 };
        Fraction::new(sign * self as i64 * rhs.den as i64, rhs.num.abs() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(24, 36), 12);
        assert_eq!(gcd(100, 200), 100);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Fraction::new(1, 2)), "1/2");
        assert_eq!(format!("{}", Fraction::new(3, 4)), "3/4");
        assert_eq!(format!("{}", Fraction::new(-5, 2)), "-5/2");
        assert_eq!(format!("{}", Fraction::new(0, 1)), "0");
    }

    #[test]
    fn test_eval() {
        assert_eq!(Fraction::new(1, 2).eval(), 0.5);
        assert_eq!(Fraction::new(3, 4).eval(), 0.75);
        assert_eq!(Fraction::new(-5, 2).eval(), -2.5);
        assert_eq!(Fraction::new(0, 1).eval(), 0.0);
    }

    #[test]
    fn test_new() {
        assert_eq!(Fraction::new(1, 2), Fraction { num: 1, den: 2 });
        assert_eq!(Fraction::new(3, 4), Fraction { num: 3, den: 4 });
        assert_eq!(Fraction::new(-5, 2), Fraction { num: -5, den: 2 });
        assert_eq!(Fraction::new(0, 1), Fraction { num: 0, den: 1 });
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        Fraction::new(1, 0);
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Fraction::new(1, 2) + Fraction::new(1, 2),
            Fraction::new(1, 1)
        );
        assert_eq!(Fraction::new(1, 2) + 1, Fraction::new(3, 2));
        assert_eq!(1 + Fraction::new(1, 2), Fraction::new(3, 2));
        assert_eq!(
            Fraction::new(1, 2) + Fraction::new(1, 4),
            Fraction::new(3, 4)
        );
        assert_eq!(Fraction::new(1, 2) + 2, Fraction::new(5, 2));
        assert_eq!(2 + Fraction::new(1, 2), Fraction::new(5, 2));
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Fraction::new(1, 2) - Fraction::new(1, 2),
            Fraction::new(0, 1)
        );
        assert_eq!(Fraction::new(1, 2) - 1, Fraction::new(-1, 2));
        assert_eq!(1 - Fraction::new(1, 2), Fraction::new(1, 2));
        assert_eq!(
            Fraction::new(1, 2) - Fraction::new(1, 4),
            Fraction::new(1, 4)
        );
        assert_eq!(Fraction::new(1, 2) - 2, Fraction::new(-3, 2));
        assert_eq!(2 - Fraction::new(1, 2), Fraction::new(3, 2));
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            Fraction::new(1, 2) * Fraction::new(1, 2),
            Fraction::new(1, 4)
        );
        assert_eq!(Fraction::new(1, 2) * 2, Fraction::new(1, 1));
        assert_eq!(2 * Fraction::new(1, 2), Fraction::new(1, 1));
        assert_eq!(
            Fraction::new(1, 2) * Fraction::new(1, 4),
            Fraction::new(1, 8)
        );
        assert_eq!(Fraction::new(1, 2) * 4, Fraction::new(2, 1));
        assert_eq!(4 * Fraction::new(1, 2), Fraction::new(2, 1));
    }

    #[test]
    fn test_div() {
        assert_eq!(
            Fraction::new(1, 2) / Fraction::new(1, 2),
            Fraction::new(1, 1)
        );
        assert_eq!(Fraction::new(1, 2) / 2, Fraction::new(1, 4));
        assert_eq!(2 / Fraction::new(1, 2), Fraction::new(4, 1));
        assert_eq!(
            Fraction::new(1, 2) / Fraction::new(1, 4),
            Fraction::new(2, 1)
        );
        assert_eq!(Fraction::new(1, 2) / 4, Fraction::new(1, 8));
        assert_eq!(4 / Fraction::new(1, 2), Fraction::new(8, 1));
    }
}
