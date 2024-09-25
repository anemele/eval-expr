pub type NumType = i64;

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
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
/// 0123456789.+-*/()
pub(crate) fn tokenize(expr: &str) -> anyhow::Result<TokenStream> {
    let mut res = vec![];
    let mut tmp = String::new();

    for c in expr.chars().into_iter() {
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                tmp.push(c);
            }
            '+' => {
                if !tmp.is_empty() {
                    res.push(Token::Number(tmp.parse::<NumType>()?));
                    tmp.clear();
                }
                res.push(Token::Add)
            }
            '-' => {
                if !tmp.is_empty() {
                    res.push(Token::Number(tmp.parse::<NumType>()?));
                    tmp.clear();
                }

                res.push(Token::Sub)
            }
            '*' => {
                if !tmp.is_empty() {
                    res.push(Token::Number(tmp.parse::<NumType>()?));
                    tmp.clear();
                }
                res.push(Token::Mul)
            }
            '/' => {
                if !tmp.is_empty() {
                    res.push(Token::Number(tmp.parse::<NumType>()?));
                    tmp.clear();
                }
                res.push(Token::Div)
            }
            '(' => {
                if !tmp.is_empty() {
                    res.push(Token::Number(tmp.parse::<NumType>()?));
                    tmp.clear();
                }
                res.push(Token::LP)
            }
            ')' => {
                if !tmp.is_empty() {
                    res.push(Token::Number(tmp.parse::<NumType>()?));
                    tmp.clear();
                }
                res.push(Token::RP)
            }
            _ => continue,
        }
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
}
