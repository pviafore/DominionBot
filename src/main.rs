use std::io;
mod parser;

fn main() {
    run_game_loop();
}


fn run_game_loop() {
    loop {
        let input = get_input();
        let output = parser::parse_message(input);
        if !output.is_empty(){
            println!("{}", output);
        }
        else {
            break;
        }
    } 
}

fn get_input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Did not get proper input");
    String::from(input.trim())
}

fn process_input(_input: String) -> String {
    String::from("String")
}

