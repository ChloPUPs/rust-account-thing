pub mod user_interface {
    use crate::util::{self, get_input_for_str, get_input_for_str_no_space};
    use crate::account::Account;

    pub struct LoopedFunctionReturn {
        pub quit_wanted: bool,
    }

    pub fn print_title() {
        println!("--ACCOUNT v0.1.0--");
    }

    pub fn prompt_info() -> Account {
        println!("Enter Account Information ->");

        let display_name = get_input_for_str("Display Name");
        let password = get_input_for_str_no_space("Password");
        let email = get_input_for_str_no_space("EMail");

        Account {
            user_id: rand::random_range(0..i64::MAX),
            display_name: display_name,
            password: password,
            email: email,
        }
    }

    pub fn prompt_logged_in(account: &Account) -> LoopedFunctionReturn {
        println!("(1) View account details, (2) Post, (3) Log out");
        let input = get_input_for_str("");

        if input == "1" {
            account.print_details();
            LoopedFunctionReturn { quit_wanted: false }
        } else if input == "2" {
            println!("What would you like to post?");
            let input = get_input_for_str("ans");
            account.post(&input);
            LoopedFunctionReturn { quit_wanted: false }
        } else if input == "3" {
            println!("Goodbye! Logging out...");
            LoopedFunctionReturn { quit_wanted: true }
        } else {
            println!("\n{}", util::INVALID_INPUT_MESSAGE);
            LoopedFunctionReturn { quit_wanted: false }
        }
    }
}

#[derive(Debug)]
pub struct Account {
    pub user_id: i64,
    pub display_name: String,
    email: String,
    password: String,
}

impl Account {
    fn print_details(&self) {
        println!(
            "Your Account -> \n\tUser ID: {}\n\tDisplay Name: {}\n\tEMail: {}\n\tPassword: {}\n",
            self.user_id, self.display_name, self.email, self.password
        );
    }
    
    fn post(&self, msg: &str) {
        println!("Created new post!\n\n{}: {}", self.display_name, msg);
    }
}
