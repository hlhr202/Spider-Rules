use std::env;
use std::process::exit;

use spider_rules::identify;

fn main() {
    let mut args = env::args();
    let ua = match args.nth(1) {
        Some(ua) => ua,
        None => {
            eprintln!("Usage: spider_rules <user_agent>");
            exit(1);
        }
    };

    match identify(&ua) {
        Some(id) => {
            println!(
                "Matched: token={}, category={}",
                id.token,
                id.category.as_str()
            );
        }
        None => println!("No match"),
    }
}
