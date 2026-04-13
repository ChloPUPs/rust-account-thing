pub mod user_interface;

use chrono::{Local, Timelike};

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
        let local = Local::now();
        let time_str = format!("{}:{}:{} {}",
            if local.hour() > 12 { local.hour() - 12 } else { local.hour() },
            local.minute(),
            local.second(),
            if local.hour() > 12 { "PM" } else { "AM" },
        );

        println!("Created new post!\n\n[{}] {}: {}",
            time_str,
            self.display_name, msg
        );
    }
}
