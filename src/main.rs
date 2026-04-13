mod account;
mod util;
use account::*;

fn main() {
    user_interface::print_title();
    let my_account = user_interface::prompt_info();
    println!("Logged in successfully.");
    //println!("{:?}", my_account);
}
