use std::io::Write;

fn read(s: String) -> String {
    s
}

fn eval(s: String) -> String {
    s
}

fn print(s: String) -> String {
    s
}

fn rep(s: String) -> String {
    print(eval(read(s)))
}

fn main() {
    loop {
        let mut input = String::new();
        print!("user> ");
        std::io::stdout().flush().unwrap();

        let bytes_read = std::io::stdin().read_line(&mut input);

        if matches!(bytes_read, Ok(0)) {
            break;
        };

        let rep_result = rep(input);
        print!("{}", rep_result);
    }
}
