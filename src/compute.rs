use crate::eval::eval;
use crate::token::{tokenize, NumType};

pub fn compute(s: &str) -> anyhow::Result<NumType> {
    let token_stream = tokenize(s)?;
    let res = eval(&token_stream)?;
    Ok(res)
}

#[cfg(test)]

mod tests {
    use crate::token::NumType;

    use super::compute;

    #[inline]
    fn f(s: &str, v: NumType) -> bool {
        compute(s).unwrap() == v
    }

    #[test]
    fn test_compute() {
        assert!(f("1+1", 2));
        assert!(f("(1+3)*(5-7)", -8));
    }
}
