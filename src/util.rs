use std::io::{self, Write};

pub const INVALID_INPUT_MESSAGE: &str = "Invalid Input! Try again.";

pub fn get_input_for_str(input_name: &str) -> String {
    let ret_val: String;
    loop {
        print!("{}: ", input_name);
        io::stdout().flush().expect("`Stdout` should flush correctly");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_value) => {
                ret_val = input;
                break;
            },
            Err(_error) => {
                println!("{}", INVALID_INPUT_MESSAGE);
                continue;
            },
        }
    }
    ret_val.trim().to_string()
}

pub fn get_input_for_str_no_space(input_name: &str) -> String {
    loop {
        let input = get_input_for_str(input_name);
        if input.contains(' ') {
            println!("Password cannot contain character ' '");
            continue;
        }
        break input;
    }
}
