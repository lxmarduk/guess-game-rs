use rand::{self, Rng};
use std::io::{self, Write};

fn main() {
    println!("Guess the sequence!");

    let secret_number: String = gen_sequence(4);
    let mut num_tries: i32 = 1;

    loop {
        print!("Input your guess: ");
        io::stdout().flush().expect("output is flushed");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("string can be read");

        let user_input: String = String::from(input.trim());

        if user_input.len() != secret_number.len() {
            println!("Length does not match required length! \"{}\"", user_input);
            continue;
        }

        let seq_res: Vec<u8> = compare_seq(&user_input, &secret_number);
        println!("{}", user_input);
        seq_res.iter().for_each(|i| {
            let letter = match i {
                0 => "_",
                1 => "[33mO[0m",
                2 => "[92mX[0m",
                _ => " ",
            };

            print!("{}", letter);
        });
        println!("");
        if seq_res.iter().all(|&item| item == 2) {
            println!("YOU WIN!");
            println!("You used {}/5 tries.", num_tries);
            break;
        }

        num_tries += 1;
        if num_tries > 5 {
            println!("YOU LOSE!");
            println!("You used more than 5 tries.");
            break;
        }
    }
}

fn gen_sequence(size: usize) -> String {
    let mut rng = rand::rng();
    let mut buff: String = String::with_capacity(size);

    let mut count: usize = 0;
    loop {
        let num: u32 = rng.random_range(0..=9);
        let ch: char = match char::from_digit(num, 10) {
            Some(x) => x,
            None => panic!("Not valid digit"),
        };
        // without repeats
        if buff.contains(ch) {
            continue;
        }
        buff.push(ch);

        count = count + 1;
        if count >= size {
            break;
        }
    }
    buff
}

fn compare_seq(user_input: &String, to_match: &String) -> Vec<u8> {
    assert!(user_input.len() == to_match.len());
    let mut vec: Vec<u8> = Vec::with_capacity(user_input.len());

    let len: usize = user_input.len();
    for i in 0..len {
        let to_char = to_match.chars().nth(i);
        let user_char = user_input.chars().nth(i);
        if to_char == user_char {
            vec.push(2);
        } else if to_match.contains(user_char.unwrap()) {
            vec.push(1);
        } else {
            vec.push(0);
        }
    }

    vec
}
