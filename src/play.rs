use crate::board::Board;
use regex::Regex;
use std::io;

fn parse_input(x_max: usize, y_max: usize) -> Result<(bool, usize, usize), String> {
    let re = Regex::new("(f *)?\\d+ *\\d+ *").expect("Invalid regex");
    let pat = Regex::new(" +").expect("Invalid regex");
    let mut x: usize;
    let mut y: usize;
    let mut first: &str;
    let mut flag: bool;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input Error");
    input = input.trim().to_string();
    if re.find(&input).is_none() {
        return Err("ERR: Input was not in the correct format".to_string());
    }
    let mut mat = pat.split(&input);
    first = mat.next().unwrap();
    if first.eq("f") {
        x = mat.next().unwrap().parse::<usize>().unwrap() - 1;
        flag = true;
    } else {
        x = first.parse::<usize>().unwrap() - 1;
        flag = false;
    }
    y = mat.next().unwrap().parse::<usize>().unwrap() - 1;
    if x >= x_max || y >= y_max {
        return Err("ERR: Row and Col input cannot be larger than board".to_string());
    }
    return Ok((flag, x, y));
}

pub fn play_game(mut board: Board) {
    let input_msg = "Enter coordinates, (1, 1) is top-left";
    let input_eg = "eg, \"1 3\" for the first column, third row";
    let flag_msg = "To flag a square, put \"f\" at the start of your message";
    let flag_eg = "eg, \"f 1 3\"";
    let msg_short = format!("{}\n{}", input_msg, flag_msg);
    let msg_long = format!("{}\n{}\n{}\n{}", input_msg, input_eg, flag_msg, flag_eg);
    let mut last_trigger: bool;
    let mut flag: bool;
    let mut x: usize;
    let mut y: usize;
    let mut last_input_invalid: bool = true;
    let mut input: Result<(bool, usize, usize), String>;
    loop {
        board.display();
        if last_input_invalid {
            println!("{}", msg_long);
        } else {
            println!("{}", msg_short);
        }
        last_input_invalid = false;
        input = parse_input(board.width, board.height);
        if input.is_err() {
            last_input_invalid = true;
            println!("{}", input.err().unwrap());
            continue;
        }
        (flag, x, y) = input.unwrap();
        if flag {
            board.toggle_flag(x, y);
        } else {
            last_trigger = board.trigger(x, y);
            if last_trigger {
                println!("=== 💥 BOOM 💥 ===\n--- GAME OVER ---");
                board.end_game();
                break;
            }
            if board.triggered == board.size - board.mines {
                println!("=== 🎉 YOU WIN 🎉 ===\n--- GAME OVER ---");
                board.end_game();
                break;
            }
        }
    }
}
