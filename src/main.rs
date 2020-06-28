mod dice;
use std::io;
use std::process;

fn main() {
    //loop == automagisk evighetsloop
    loop {
        println!("Input dice roll! Skriv quit för att sluta.");
        let mut dice_input = String::new();
        let input_result = io::stdin().read_line(&mut dice_input);

        if input_result.is_err() {
            println!("Nu gick något fel.");
            process::exit(1);
        }
        
        //Trim för att städa \r\n, annars funkar inte quit-commandot
        let dice_input = dice_input.trim().to_lowercase();

        if dice_input == "quit" {
            println!("Shutting down!");
            break;
        }

        if !dice_input.to_lowercase().starts_with("$d") {
            println!("Fel format!");
            continue;
        }

        let args = dice_input.split_whitespace();
        let mut sum = 0;
        let mut results = vec![];

        for arg in args {
            let dice_sides = arg.parse::<u8>();
            if dice_sides.is_ok() {
                let dice_sides: u8 = dice_sides.unwrap();
                let roll = dice::roll(&dice_sides);
                let result = format!("D{}: {}", dice_sides, roll);
                sum += roll;
                results.push(result);
            }
        }
        let output = results.join(", ");
        println!("Result: {}, Dice results: {}", sum, output);
    }
}
