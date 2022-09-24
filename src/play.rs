use crate::board::Board;
use regex::Regex;
use std::io;

fn parse_input() -> (bool, usize, usize, bool) {
    let err_msg = "Input must take format ( *f)? *\\d+ *\\d+ *";
    let re = Regex::new("(f *)?\\d+ *\\d+ *").expect("Invalid regex");
    let pat = Regex::new(" +").expect("Invalid regex");
    let mut invalid_input = false;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(&err_msg);
    input = input.trim().to_string();
    while re.find(&input).is_none() {
        invalid_input = true;
        println!("{}", err_msg);
        io::stdin().read_line(&mut input).expect(&err_msg);
        input = input.trim().to_string();
    }
    let mut mat = pat.split(&input);
    let first = mat.next().unwrap();
    if first.eq("f") {
        return (
            true,
            mat.next().unwrap().parse::<usize>().unwrap(),
            mat.next().unwrap().parse::<usize>().unwrap(),
            invalid_input,
        );
    } else {
        return (
            false,
            first.parse::<usize>().unwrap(),
            mat.next().unwrap().parse::<usize>().unwrap(),
            invalid_input,
        );
    }
}

pub fn play_game(board: Board) {
    let mut last_trigger = false;

    let input_msg = "Enter coordinates, (1, 1) is top-left";
    let input_eg = "eg, \"1 3\" for the first column, third row";
    let flag_msg = "To flag a square, put \"f\" at the start of your message";
    let flag_eg = "eg, \"f 1 3\"";
    let msg_short = format!("{}\n{}", input_msg, flag_msg);
    let msg_long = format!("{}\n{}\n{}\n{}", input_msg, input_eg, flag_msg, flag_eg);
    let mut f: bool;
    let mut x: usize;
    let mut y: usize;
    let mut last_input_invalid: bool = true;
    while !last_trigger {
        board.display();
        if last_input_invalid {
            println!("{}", msg_long);
        } else {
            println!("{}", msg_short);
        }
        (f, x, y, last_input_invalid) = parse_input();
        println!("{} {} {}", f, x, y)
    }
}
