use crate::eval::eval;
use crate::token::{tokenize, NumType};
use crate::transform::standardize;

pub fn compute(s: &str) -> anyhow::Result<NumType> {
    let token_stream = tokenize(s)?;
    let token_stream = standardize(&token_stream)?;
    let res = eval(&token_stream)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::compute;

    #[test]
    fn test_compute() {
        assert!(compute("1+1").unwrap() == 2);
        assert!(compute("(1+3)*(5-7)").unwrap() == -8);
        assert!(compute("-1+(-1-1)").unwrap() == -3);
        assert!(compute("(1)+1+2").unwrap() == 4);
    }

    #[test]
    fn test_compute_divide() {
        assert!(compute("1/2").unwrap() == 0);
        assert!(compute("10/3").unwrap() == 3);
    }

    #[test]
    fn test_compute_err() {
        assert!(compute("1+1+").is_err());
        assert!(compute("()").is_err());
        assert!(compute("1++1+2+3").is_err());
    }
}
