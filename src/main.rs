use std::env;

use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let args: Vec<String> = env::args().collect();

    let output = format!(
        "{}{}\n{}{}{}{}{}{}\n{}{}",
        "/// @title: ",&args[1],"/// @author: ",&args[2] ," ","<", &args[3], ">","/// @notice: ",&args[4],
    );

    println!("{}", output); // Print the header to console.

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
}
