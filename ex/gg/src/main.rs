use std::io; 

// Obtain and return input.
fn get_input() -> String {
    let mut input = String::new();

    println!("Awaiting input:"); 

    // Read entry into 'input'.
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input.");

    input 
}

// Check if the guess is the same as the specified target.
fn check_guess(g: String, t: String) -> bool {
    if g.trim() == t {
        println!("Correct!");
        return true;
    }

    false
}

fn analyze_guess(g: String, t: String) {
    for (index, char) in g.iter().enumerate() {
        
    }
}

fn main() {
    let target: String = String::from("target");

    for n in 0..=2 { 
        let guess: String = get_input();

        if check_guess(guess, target) {
            return;
        }
    }

    loop {
        let guess: String = get_input();

        if check_guess(guess, target) {
            return;
        }

        analyze_guess(guess, target);
    }
}
