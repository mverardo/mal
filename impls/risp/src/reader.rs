#[derive(PartialEq, Debug)]
enum TokenType {
    LeftParen,
    RightParen,
}

#[derive(PartialEq, Debug)]
struct Token {
    t_type: TokenType,
}

struct Reader<'a> {
    input: &'a [Token],
    position: usize,
}

impl<'a> Reader<'a> {
    fn next(&mut self) -> &Token {
        self.position += 1;
        &self.input[self.position]
    }

    fn peek(&mut self) -> &Token {
        &self.input[self.position]
    }

    fn new(input: &'a [Token]) -> Self {
        Reader { input, position: 0 }
    }
}

fn read_form<'a>(reader: &Reader<'a>) -> &'a [Token] {
    todo!()
}

fn tokenize(input: String) -> &'static [Token] {
    //[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)
    todo!()
}

fn read_str(input: String) {
    todo!()
}

#[cfg(test)]
mod test {
    use super::{tokenize, Token, TokenType};

    #[test]
    fn tokenize_works() {
        let result = tokenize("()".to_string());
        assert_eq!(
            result,
            &[
                Token {
                    t_type: TokenType::LeftParen,
                },
                Token {
                    t_type: TokenType::RightParen,
                },
            ],
        )
    }
}
