use std::io;
mod parser;
mod cards;


fn main() {
    run_game_loop();
}


fn run_game_loop() {
    loop {
        let input = get_input();
        let output = parser::parse_message(input);
        match output {
            parser::Message::InvalidMessage => break,
            _ => println!("Valid Message")
        }
    } 
}

fn get_input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Did not get proper input");
    String::from(input.trim())
}

