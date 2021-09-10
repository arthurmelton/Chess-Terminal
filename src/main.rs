use text_io::read;

fn main() {
    let mut board:Vec<&str> = vec!["♖", "♘", "♗", "♕", "♔", "♗", "♘", "♖",
                                   "♙", "♙", "♙", "♙", "♙", "♙", "♙", "♙",
                                   " ", " ", " ", " ", " ", " ", " ", " ",
                                   " ", " ", " ", " ", " ", " ", " ", " ",
                                   " ", " ", " ", " ", " ", " ", " ", " ",
                                   " ", " ", " ", " ", " ", " ", " ", " ",
                                   "♟", "♟", "♟", "♟", "♟", "♟", "♟", "♟",
                                   "♜", "♞", "♝", "♛", "♚", "♝", "♞", "♜"];
    let mut who_goes = "White";
    println!("{}", make_board(board.clone()));
    println!("Hello {} you go first, say the peice you want to move. (ex. a3)", who_goes);
    let mut move_peice:String = read!();
    while get_color(get_pos(move_peice), board.clone()) != who_goes {
        println!("uhhh I am sorry but you are {} but anyways where do you want to move your piece", who_goes.to_lowercase());
        move_peice = read!();
    }
    println!("Where do you want to move it?");
    let move_peice:String = read!();
}

fn make_board(board:Vec<&str>) -> String {
    ["  ---------------------------------
8 | ", board[0], " | ", board[1], " | ", board[2], " | ", board[3], " | ", board[4], " | ", board[5], " | ", board[6], " | ", board[7], " |
  ---------------------------------
7 | ", board[8], " | ", board[9], " | ", board[10], " | ", board[11], " | ", board[12], " | ", board[13], " | ", board[14], " | ", board[15], " |
  ---------------------------------
6 | ", board[16], " | ", board[17], " | ", board[18], " | ", board[19], " | ", board[20], " | ", board[21], " | ", board[22], " | ", board[23], " |
  ---------------------------------
5 | ", board[24], " | ", board[25], " | ", board[26], " | ", board[27], " | ", board[28], " | ", board[29], " | ", board[30], " | ", board[31], " |
  ---------------------------------
4 | ", board[32], " | ", board[33], " | ", board[34], " | ", board[35], " | ", board[36], " | ", board[37], " | ", board[38], " | ", board[39], " |
  ---------------------------------
3 | ", board[40], " | ", board[41], " | ", board[42], " | ", board[43], " | ", board[44], " | ", board[45], " | ", board[46], " | ", board[47], " |
  ---------------------------------
2 | ", board[48], " | ", board[49], " | ", board[50], " | ", board[51], " | ", board[52], " | ", board[53], " | ", board[54], " | ", board[55], " |
  ---------------------------------
1 | ", board[56], " | ", board[57], " | ", board[58], " | ", board[59], " | ", board[60], " | ", board[61], " | ", board[62], " | ", board[63], " |
  ---------------------------------
    a   b   c   d   e   f   g   h", ].join("")
}

fn get_pos(pos:String) -> i32 {
    let letter = pos.chars().nth(0).unwrap();
    let number = pos.chars().nth(1).unwrap();
    return ((8-(number.to_string()).parse::<i32>().unwrap())*8)+((char::to_digit(letter, 18).unwrap()-10) as i32)
}

fn get_color(pos:i32, board:Vec<&str>) -> &str {
    if vec!["♟", "♜", "♞", "♝", "♛", "♚"].contains(&board[pos as usize]) {
        return "White";
    }
    else if vec!["♙", "♜", "♞", "♝", "♛", "♚"].contains(&board[pos as usize]) {
        return "Black";
    }
    else {
        return "Empty";
    }
}

fn is_move_valid(move_from:String, move_to:String, board:Vec<&str>) -> bool {
    return false;
}