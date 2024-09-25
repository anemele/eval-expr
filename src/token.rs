pub type NumType = i64;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(NumType),
    Add,
    Sub,
    Mul,
    Div,
    LP,
    RP,
}

pub type TokenStream = Vec<Token>;
/// valid character set
/// 0123456789+-*/()
pub fn tokenize(expr: &str) -> anyhow::Result<TokenStream> {
    let mut res = vec![];
    let mut tmp = String::new();

    for c in expr.chars().into_iter() {
        if c.is_ascii_digit() {
            tmp.push(c);
            continue;
        }
        if !tmp.is_empty() {
            res.push(Token::Number(tmp.parse::<NumType>()?));
            tmp.clear();
        }
        let token = match c {
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '/' => Token::Div,
            '(' => Token::LP,
            ')' => Token::RP,
            _ => continue,
        };
        res.push(token);
    }
    if !tmp.is_empty() {
        res.push(Token::Number(tmp.parse::<NumType>()?));
        tmp.clear();
    }

    Ok(res)
}

#[test]
fn test_tokenize() {
    use Token::*;

    assert_eq!(tokenize("1+1").unwrap(), vec![Number(1), Add, Number(1)]);
    assert_eq!(
        tokenize("3*6+1").unwrap(),
        vec![Number(3), Mul, Number(6), Add, Number(1)]
    );
    assert_eq!(
        tokenize("-1+( -1- 1 )").unwrap(),
        vec![Sub, Number(1), Add, LP, Sub, Number(1), Sub, Number(1), RP]
    );
    assert_eq!(tokenize("1/2").unwrap(), vec![Number(1), Div, Number(2)]);
    assert_eq!(tokenize("123").unwrap(), vec![Number(123)]);
    assert_eq!(tokenize("").unwrap(), vec![]);
    assert_eq!(
        tokenize("1+2*3").unwrap(),
        vec![Number(1), Add, Number(2), Mul, Number(3)]
    );
    assert_eq!(
        tokenize("0001+002/03").unwrap(),
        vec![Number(1), Add, Number(2), Div, Number(3)]
    )
}
