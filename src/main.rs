use rand::Rng;
use std::io;

// @TODO: handle edge-case characters like 'Ç'
// @TODO: prevent invalid inputs from flooding the screen
fn main() {
    println!("FOODLE!");

    let food_list: [&str; 32] = [
        "Pecan", "Prune", "Betel", "Mooli", "Chive", "Lemon", "Grape", "Swede", "Galia", "Choko",
        "Papaw", "Cress", "Ackee", "Jaffa", "Guava", "Apple", "Onion", "Drupe", "Morel", "Peach",
        "Melon", "Olive", "Laver", "Enoki", "Savoy", "Gourd", "Cubeb", "Eater", "Areca", "Pulse",
        "Chard", "Mango",
    ];

    let ri: usize = rand::thread_rng().gen_range(0..=31); // random index
    let answer = food_list[ri].to_ascii_uppercase();

    let max_tries: i8 = 5;
    let mut tries: i8 = 0;

    println!("The selected word is {answer}.");
    println!(
        "Guess the selected word by typing below. You have {} tries.",
        max_tries
    );

    loop {
        let mut entry = String::new();

        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read line!");

        let entry = entry.trim();
        if entry.len() < 5 {
            continue;
        }

        let entry: &str = &(entry.trim())[0..=4].to_ascii_uppercase();
        let mut answer_state: [bool; 5] = [false; 5];
        let mut entry_state: [i32; 5] = [0; 5];

        for (i, c) in entry.chars().enumerate() {
            // first, check if it's a '2' case
            if c == answer.chars().nth(i).unwrap() {
                entry_state[i] = 2;
                answer_state[i] = true;
            } else {
                // else, check if it's a '1' case
                for (i2, c2) in answer.chars().enumerate() {
                    if c == c2 && !answer_state[i2] {
                        entry_state[i] = 1;
                        answer_state[i2] = true;
                        break;
                    }
                }
            }
        }

        for (i, c) in entry.chars().enumerate() {
            match entry_state[i] {
                0 => print!("  {}  ", c),
                1 => print!("( {} )", c),
                2 => print!("[ {} ]", c),
                _ => todo!(),
            }
        }
        tries += 1;
        println!("  //{}", tries);

        if !entry_state.contains(&0) && !entry_state.contains(&1) {
            println!("You win!");
            break;
        } else if tries >= max_tries {
            println!("You lose...");
            break;
        }

        // match guess.cmp(&secret_number) {
        //     Ordering::Less => println!("Too small!"),
        //     Ordering::Greater => println!("Too big!"),
        //     Ordering::Equal => {
        //         println!("You win!");
        //         break;
        //     }
        // }
    }
}
