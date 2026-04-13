pub mod user_interface {
    use crate::util::{get_input_for_str, get_input_for_str_no_space};
    use crate::account::Account;

    pub fn print_title() {
        println!("--ACCOUNTS v0.1.0--");
    }

    pub fn prompt_info() -> Account {
        println!("Enter Account Information:");

        let display_name = get_input_for_str("Display Name");
        let password = get_input_for_str_no_space("Password");
        let email = get_input_for_str_no_space("Email");

        Account {
            user_id: rand::random_range(0..i64::MAX),
            display_name: display_name,
            password: password,
            email: email,
        }
    }

    pub fn prompt_logged_in(account: &Account) {
        println!("(1) View account details, (2) Post, (3) Log out");
        let input = get_input_for_str("");

        if input == "1" {
            account.print_details();
        } else if input == "2" {
            
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
            "User ID:{}\nDisplay Name: {}\nEMail: {}\nPassword: {}",
            self.user_id, self.display_name, self.email, self.password
        );
    }
    
    fn post(&self, msg: &str) {
        println!("Created new post!\n{}: {}", self.display_name, msg);
    }
}
