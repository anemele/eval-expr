use crate::token::Token::{self, *};
use crate::token::TokenStream;

#[derive(PartialEq, Debug)]
enum Rule {
    Deny,
    Allow,
    Middle,
}

fn init_rules(token: &Token) -> Rule {
    match token {
        Number(_) => Rule::Allow,
        Add | Sub => Rule::Middle,
        Mul | Div => Rule::Deny,
        LP => Rule::Allow,
        RP => Rule::Deny,
    }
}

fn next_rule(this: &Token, next: &Token) -> Rule {
    match this {
        Number(_) => match next {
            Number(_) => Rule::Allow,
            Add | Sub => Rule::Allow,
            Mul | Div => Rule::Allow,
            LP => Rule::Deny,
            RP => Rule::Allow,
        },
        Add | Sub => match next {
            Number(_) => Rule::Allow,
            Add | Sub => Rule::Deny,
            Mul | Div => Rule::Deny,
            LP => Rule::Allow,
            RP => Rule::Deny,
        },
        Mul | Div => match next {
            Number(_) => Rule::Allow,
            Add | Sub => Rule::Deny,
            Mul | Div => Rule::Deny,
            LP => Rule::Allow,
            RP => Rule::Deny,
        },
        LP => match next {
            Number(_) => Rule::Allow,
            Add | Sub => Rule::Middle,
            Mul | Div => Rule::Deny,
            LP => Rule::Allow,
            RP => Rule::Deny,
        },
        RP => match next {
            Number(_) => Rule::Deny,
            Add | Sub => Rule::Allow,
            Mul | Div => Rule::Allow,
            LP => Rule::Deny,
            RP => Rule::Allow,
        },
    }
}

pub fn standardize(stream: &TokenStream) -> anyhow::Result<TokenStream> {
    let mut res = TokenStream::new();

    let mut stream = stream.iter();
    let Some(mut this) = stream.next() else {
        return Ok(res);
    };

    match init_rules(this) {
        Rule::Deny => anyhow::bail!("Invalid expression"),
        Rule::Allow => res.push(*this),
        Rule::Middle => {
            res.push(Number(0));
            res.push(*this)
        }
    }

    for token in stream {
        match next_rule(this, &token) {
            Rule::Deny => anyhow::bail!("Invalid expression"),
            Rule::Allow => res.push(*token),
            Rule::Middle => {
                res.push(Number(0));
                res.push(*token);
            }
        }
        this = token;
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::token::Token::*;

    use super::standardize;

    #[test]
    fn test_standardize() {
        // -1+(-1-1)
        let input = vec![Sub, Number(1), Add, LP, Sub, Number(1), Sub, Number(1), RP];
        // 0-1+(0-1-1)
        let expected = vec![
            Number(0),
            Sub,
            Number(1),
            Add,
            LP,
            Number(0),
            Sub,
            Number(1),
            Sub,
            Number(1),
            RP,
        ];
        assert_eq!(standardize(&input).unwrap(), expected);
    }
}
