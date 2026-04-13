mod account;
mod util;
use account::*;
use account::user_interface::LoopedFunctionReturn;

fn main() {
    user_interface::print_title();

    let my_account = user_interface::prompt_info();
    println!("\nLogged in successfully.");

    let mut res = LoopedFunctionReturn { quit_wanted: false };
    while !res.quit_wanted {
        res = user_interface::prompt_logged_in(&my_account);
    }
}
