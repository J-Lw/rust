use std::io; 

// Obtain and return input from user.
fn get_input() -> String {
    let mut input = String::new();

    println!("Awaiting input:"); 

    // Read user entry into 'input'.
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input.");

    input 
}

fn check_guess(g: String, t: String) -> bool {
    if g == t {
        println!("test: true");
        return true;
    }

    println!("test: true");
    false
}

fn main() {
    let target: String = String::from("target");

    let guess: String = get_input();
    check_guess(guess, target);
}
