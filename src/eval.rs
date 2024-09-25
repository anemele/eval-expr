use crate::token::NumType;
use crate::token::Token::{self, *};
use crate::token::TokenStream;

fn operate(opd: &mut Vec<NumType>, opr: &Token) -> anyhow::Result<()> {
    let b = opd.pop().ok_or(anyhow::anyhow!("empty operand"))?;
    let a = opd.pop().ok_or(anyhow::anyhow!("empty operand"))?;

    let c = match *opr {
        Add => a + b,
        Sub => a - b,
        Mul => a * b,
        Div => a / b,
        _ => anyhow::bail!("never go here!"),
    };
    opd.push(c);

    Ok(())
}

pub fn eval(stream: &TokenStream) -> anyhow::Result<NumType> {
    let mut operand_stack = vec![];
    let mut operator_stack = vec![];
    for token in stream {
        match token {
            Number(number) => {
                operand_stack.push(number.to_owned());
            }
            Add | Sub => {
                while !operator_stack.is_empty() {
                    // why here requires type annotation?
                    let op: &Token = operator_stack
                        .pop()
                        .ok_or(anyhow::anyhow!("empty operator"))?;
                    if *op == LP {
                        operator_stack.push(op);
                        break;
                    }
                    operate(&mut operand_stack, op)?;
                }
                operator_stack.push(token);
            }
            Mul | Div => {
                while !operator_stack.is_empty() {
                    let op = operator_stack
                        .pop()
                        .ok_or(anyhow::anyhow!("empty oprator"))?;
                    if *op == LP {
                        operator_stack.push(op);
                        break;
                    }
                    if *op == Add || *op == Sub {
                        operator_stack.push(op);
                        break;
                    }
                    operate(&mut operand_stack, op)?;
                }
                operator_stack.push(token);
            }
            LP => {
                operator_stack.push(token);
            }
            RP => loop {
                let op = operator_stack
                    .pop()
                    .ok_or(anyhow::anyhow!("empty oprator"))?;
                if *op == LP {
                    break;
                }
                operate(&mut operand_stack, op)?;
            },
        }
    }

    while !operator_stack.is_empty() {
        let op = operator_stack.pop().ok_or(anyhow::anyhow!("empty?"))?;
        operate(&mut operand_stack, op)?;
    }

    let res = operand_stack.pop().ok_or(anyhow::anyhow!("empty stack"))?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::token::Token::*;

    use super::eval;

    #[test]
    fn test_eval() {
        assert_eq!(
            // 1 + 1
            eval(&vec![Number(1), Add, Number(1)]).unwrap(),
            2
        );
        assert_eq!(
            // 10 * 9
            eval(&vec![Number(10), Mul, Number(9)]).unwrap(),
            90
        );
        assert_eq!(
            // (1+2)*(3-4)
            eval(&vec![
                LP,
                Number(1),
                Add,
                Number(2),
                RP,
                Mul,
                LP,
                Number(3),
                Sub,
                Number(4),
                RP
            ])
            .unwrap(),
            -3
        )
    }
}
