use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rand::Rng;

fn main() {
    //Set prompt
    let mut rand = rand::thread_rng();
    let random_line = rand.gen_range(0..4);
    //println!("{}", random_line);

    let mut prompt = String::new(); //Line the prompt will be stored in

    let f = File::open("src/prompts.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    for (index, line) in f.lines().enumerate() {
        let line = line.expect("Unable to read line");
        if index == random_line {
            prompt = line;
            break;
        }
    }
    //println!("{}", prompt);

    let mut player_answer = String::new();
    for c in prompt.chars() {
        if c.is_alphabetic() {
            player_answer.push('_');
        }else{
            player_answer.push(c);
        }
    }

    let mut lives = 6; //Set lives
    println!("lives: {}", lives);

    prt_hang_man(lives);

    //lives -= 1; 
    //prt_hang_man(lives);

    loop {
        println!("{}", player_answer);
        let stdin = io::stdin();
        let user_input = stdin.lock().lines().next().unwrap().unwrap();
        //println!("{}", user_input);

        for guess in user_input.chars() {
            if guess.is_alphabetic() {
                let mut is_in_prompt = false; //flag to check if the character was correct
                for (i, c) in prompt.chars().enumerate() {
                    if c.eq_ignore_ascii_case(&guess) {
                        is_in_prompt = true;
                        player_answer.replace_range(i..i+1, &c.to_string());
                    }
                }
                if is_in_prompt == false {
                    lives -= 1;
                }
            }
        }

        prt_hang_man(lives);

        if lives == 0 {
            break;
        }

        if prompt == player_answer {
            break;
        }
    }

    if lives == 0 {
        println!("Game Over");
        println!("You Lose");
    } else {
        println!("Congratulations");
        println!("You Win");
    }
}

fn prt_hang_man(lives: i32) {
    if lives == 6 {
        println!(" ┏━━━┓ ");
        println!(" ┃     ");
        println!(" ┃     ");
        println!(" ┃     ");
        println!("━┻━    ");
    }else if lives == 5 {
        println!(" ┏━━━┓ ");
        println!(" ┃   O ");
        println!(" ┃     ");
        println!(" ┃     ");
        println!("━┻━    ");
    }else if lives == 4 {
        println!(" ┏━━━┓ ");
        println!(" ┃   O ");
        println!(" ┃   | ");
        println!(" ┃   | ");
        println!("━┻━    ");
    }else if lives == 3 {
        println!(" ┏━━━┓ ");
        println!(" ┃   O ");
        println!(" ┃  /| ");
        println!(" ┃   | ");
        println!("━┻━    ");
    }else if lives == 2 {
        println!(" ┏━━━┓ ");
        println!(" ┃   O ");
        println!(" ┃  /|\\");
        println!(" ┃   | ");
        println!("━┻━    ");
    }else if lives == 1 {
        println!(" ┏━━━┓ ");
        println!(" ┃   O ");
        println!(" ┃  /|\\");
        println!(" ┃   | ");
        println!("━┻━ /  ");
    }else if lives == 0 {
        println!(" ┏━━━┓ ");
        println!(" ┃   O ");
        println!(" ┃  /|\\");
        println!(" ┃   | ");
        println!("━┻━ / \\");
    }
    println!("Lives Left: {}", lives);
}