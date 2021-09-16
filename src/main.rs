use text_io::read;

fn main() {
    let mut board: Vec<&str> = vec![
        "♖", "♘", "♗", "♕", "♔", "♗", "♘", "♖", "♙", "♙", "♙", "♙", "♙", "♙", "♙", "♙", " ", " ",
        " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ",
        " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "♟", "♟", "♟", "♟", "♟", "♟",
        "♟", "♟", "♜", "♞", "♝", "♛", "♚", "♝", "♞", "♜",
    ];
    let mut who_goes = "White";
    let mut can_castle: Vec<i32> = Vec::new();
    let mut has_move: Vec<bool> = vec![false, false, false, false, false, false];
    while !checkmate(board.clone(), who_goes, can_castle.clone()) {
        can_castle = Vec::new();
        if !has_move[4]
            && !has_move[0]
            && get_piece(1, board.clone()) == "Empty"
            && get_piece(2, board.clone()) == "Empty"
            && get_piece(3, board.clone()) == "Empty"
        {
            can_castle.push(2);
        }
        if !has_move[4]
            && !has_move[1]
            && get_piece(5, board.clone()) == "Empty"
            && get_piece(6, board.clone()) == "Empty"
        {
            can_castle.push(6);
        }
        if !has_move[5]
            && !has_move[2]
            && get_piece(57, board.clone()) == "Empty"
            && get_piece(58, board.clone()) == "Empty"
            && get_piece(59, board.clone()) == "Empty"
        {
            can_castle.push(58);
        }
        if !has_move[5]
            && !has_move[3]
            && get_piece(61, board.clone()) == "Empty"
            && get_piece(62, board.clone()) == "Empty"
        {
            can_castle.push(62);
        }
        println!("{}", make_board(board.clone()));
        println!(
            "Hello {} you go first, say the peice you want to move. (ex. a3)",
            who_goes
        );
        let mut move_piece: String = read!();
        while !move_piece.chars().nth(0).ok_or(0).is_ok()
            || !move_piece.chars().nth(1).ok_or(0).is_ok()
            || get_pos(move_piece.clone()) > 63
            || get_pos(move_piece.clone()) < 0
        {
            println!("That is not a valid position it has to be the letters between a and h and the numbers between 1 and 8");
            move_piece = read!();
        }
        while !move_piece.chars().nth(0).ok_or(0).is_ok()
            || !move_piece.chars().nth(1).ok_or(0).is_ok()
            || get_color(get_pos(move_piece.clone()), board.clone()) != who_goes
        {
            println!(
                "uhhh I am sorry but you are {} but I guess you can pick a new piece to move",
                who_goes.to_lowercase()
            );
            move_piece = read!();
        }
        println!("Where do you want to move it?");
        let mut move_piece_to: String = read!();
        while !move_piece_to.chars().nth(0).ok_or(0).is_ok()
            || !move_piece_to.chars().nth(1).ok_or(0).is_ok()
            || get_pos(move_piece_to.clone()) > 63
            || get_pos(move_piece_to.clone()) < 0
        {
            println!("That is not a valid position it has to be the letters between a and h and the numbers between 1 and 8");
            move_piece_to = read!();
        }
        while !move_piece_to.chars().nth(0).ok_or(0).is_ok()
            || !move_piece_to.chars().nth(1).ok_or(0).is_ok()
            || !move_piece.chars().nth(0).ok_or(0).is_ok()
            || !move_piece.chars().nth(1).ok_or(0).is_ok()
            || !is_move_valid(
                get_pos(move_piece.clone()),
                get_pos(move_piece_to.clone()),
                board.clone(),
                can_castle.clone(),
            )
        {
            println!("I am sorry but you cant move {} ({}) to {}, but I guess you can pick a new piece to move", board[get_pos(move_piece.clone()) as usize], move_piece.clone(), move_piece_to.clone());
            move_piece = read!();
            while !move_piece.chars().nth(0).ok_or(0).is_ok()
                || !move_piece.chars().nth(1).ok_or(0).is_ok()
                || get_color(get_pos(move_piece.clone()), board.clone()) != who_goes
            {
                println!(
                    "uhhh I am sorry but you are {} but anyways what piece do you want to move",
                    who_goes.to_lowercase()
                );
                move_piece = read!();
            }
            println!("Where do you want to move it?");
            move_piece_to = read!();
        }
        board = move_piece_fn(
            get_pos(move_piece_to.clone()),
            get_pos(move_piece.clone()),
            board.clone(),
            can_castle.clone(),
        );
        if get_piece(get_pos(move_piece_to.clone()), board.clone()) == "Pawn"
            && [0, 1, 2, 3, 4, 5, 6, 7, 55, 56, 57, 58, 59, 60, 61, 62, 63]
                .contains(&get_pos(move_piece_to.clone()))
        {
            println!("what do you want your pawn at {} to be now? (1: queen, 2: rook, 3: knight, 4: bishop) (ex. 1)", move_piece_to.clone());
            let mut want: String = read!();
            while !["1", "2", "3", "4"].contains(&&*want) {
                println!("You have to type a number between 1 and 4, what do you want your pawn at {} to be now? (1: queen, 2: rook, 3: knight, 4: bishop) (ex. 1)", move_piece_to.clone());
                want = read!();
            }
            if get_color(get_pos(move_piece_to.clone()), board.clone()) == "White" {
                board[get_pos(move_piece_to.clone()) as usize] =
                    ["♛", "♜", "♞", "♝"][want.parse::<usize>().unwrap() - 1];
            } else {
                board[get_pos(move_piece_to.clone()) as usize] =
                    ["♕", "♖", "♘", "♗"][want.parse::<usize>().unwrap() - 1];
            }
        }
        match get_pos(move_piece.clone()) {
            0 => {
                has_move[0] = true;
            }
            7 => {
                has_move[1] = true;
            }
            56 => {
                has_move[2] = true;
            }
            63 => {
                has_move[3] = true;
            }
            4 => {
                has_move[4] = true;
            }
            60 => {
                has_move[5] = true;
            }
            _ => {}
        }
        if who_goes == "White" {
            who_goes = "Black";
        } else {
            who_goes = "White";
        }
    }
    if who_goes == "White" {
        who_goes = "Black";
    } else {
        who_goes = "White";
    }
    println!("{}, you won!", who_goes);
}

fn make_board(board: Vec<&str>) -> String {
    [
        "  ---------------------------------
8 | ",
        board[0],
        " | ",
        board[1],
        " | ",
        board[2],
        " | ",
        board[3],
        " | ",
        board[4],
        " | ",
        board[5],
        " | ",
        board[6],
        " | ",
        board[7],
        " |
  ---------------------------------
7 | ",
        board[8],
        " | ",
        board[9],
        " | ",
        board[10],
        " | ",
        board[11],
        " | ",
        board[12],
        " | ",
        board[13],
        " | ",
        board[14],
        " | ",
        board[15],
        " |
  ---------------------------------
6 | ",
        board[16],
        " | ",
        board[17],
        " | ",
        board[18],
        " | ",
        board[19],
        " | ",
        board[20],
        " | ",
        board[21],
        " | ",
        board[22],
        " | ",
        board[23],
        " |
  ---------------------------------
5 | ",
        board[24],
        " | ",
        board[25],
        " | ",
        board[26],
        " | ",
        board[27],
        " | ",
        board[28],
        " | ",
        board[29],
        " | ",
        board[30],
        " | ",
        board[31],
        " |
  ---------------------------------
4 | ",
        board[32],
        " | ",
        board[33],
        " | ",
        board[34],
        " | ",
        board[35],
        " | ",
        board[36],
        " | ",
        board[37],
        " | ",
        board[38],
        " | ",
        board[39],
        " |
  ---------------------------------
3 | ",
        board[40],
        " | ",
        board[41],
        " | ",
        board[42],
        " | ",
        board[43],
        " | ",
        board[44],
        " | ",
        board[45],
        " | ",
        board[46],
        " | ",
        board[47],
        " |
  ---------------------------------
2 | ",
        board[48],
        " | ",
        board[49],
        " | ",
        board[50],
        " | ",
        board[51],
        " | ",
        board[52],
        " | ",
        board[53],
        " | ",
        board[54],
        " | ",
        board[55],
        " |
  ---------------------------------
1 | ",
        board[56],
        " | ",
        board[57],
        " | ",
        board[58],
        " | ",
        board[59],
        " | ",
        board[60],
        " | ",
        board[61],
        " | ",
        board[62],
        " | ",
        board[63],
        " |
  ---------------------------------
    a   b   c   d   e   f   g   h",
    ]
    .join("")
}

fn get_pos(pos: String) -> i32 {
    let letter = pos.chars().nth(0).unwrap();
    let number = pos.chars().nth(1).unwrap();
    return ((8 - (number.to_string()).parse::<i32>().unwrap()) * 8)
        + ((char::to_digit(letter, 18).unwrap() - 10) as i32);
}

fn get_color(pos: i32, board: Vec<&str>) -> &str {
    return if vec!["♟", "♜", "♞", "♝", "♛", "♚"].contains(&board[pos as usize]) {
        "White"
    } else if vec!["♙", "♖", "♘", "♗", "♕", "♔"].contains(&board[pos as usize]) {
        "Black"
    } else {
        "Empty"
    };
}

fn get_piece(pos: i32, board: Vec<&str>) -> &str {
    return if vec!["♟", "♙"].contains(&board[pos as usize]) {
        "Pawn"
    } else if vec!["♖", "♜"].contains(&board[pos as usize]) {
        "Rook"
    } else if vec!["♘", "♞"].contains(&board[pos as usize]) {
        "Knight"
    } else if vec!["♝", "♗"].contains(&board[pos as usize]) {
        "Bishop"
    } else if vec!["♕", "♛"].contains(&board[pos as usize]) {
        "Queen"
    } else if vec!["♔", "♚"].contains(&board[pos as usize]) {
        "King"
    } else {
        "Empty"
    };
}

fn is_move_valid(move_from: i32, move_to: i32, board: Vec<&str>, can_castle: Vec<i32>) -> bool {
    match get_piece(move_from, board.clone()) {
        "Pawn" => {
            if get_color(move_to, board.clone()) == "Empty" {
                if get_color(move_from, board.clone()) == "White" {
                    if [48, 49, 50, 51, 52, 53, 54, 55].contains(&move_from)
                        && (move_to == move_from - 8
                            || (move_to == move_from - 16
                                && get_color(move_from - 8, board.clone()) == "Empty"))
                    {
                        let mut new_board = board.clone();
                        new_board[move_to as usize] = new_board[move_from as usize];
                        new_board[move_from as usize] = " ";
                        if check(new_board, can_castle.clone())
                            .contains(&get_color(move_from, board.clone()))
                        {
                            return false;
                        }
                        return true;
                    } else if move_to == move_from - 8 {
                        let mut new_board = board.clone();
                        new_board[move_to as usize] = new_board[move_from as usize];
                        new_board[move_from as usize] = " ";
                        if check(new_board, can_castle.clone())
                            .contains(&get_color(move_from, board.clone()))
                        {
                            return false;
                        }
                        return true;
                    }
                } else {
                    if [8, 9, 10, 11, 12, 13, 14, 15].contains(&move_from)
                        && (move_to == move_from + 8
                            || (move_to == move_from + 16
                                && get_color(move_from + 8, board.clone()) == "Empty"))
                    {
                        let mut new_board = board.clone();
                        new_board[move_to as usize] = new_board[move_from as usize];
                        new_board[move_from as usize] = " ";
                        if check(new_board, can_castle.clone())
                            .contains(&get_color(move_from, board.clone()))
                        {
                            return false;
                        }
                        return true;
                    } else if move_to == move_from + 8 {
                        let mut new_board = board.clone();
                        new_board[move_to as usize] = new_board[move_from as usize];
                        new_board[move_from as usize] = " ";
                        if check(new_board, can_castle.clone())
                            .contains(&get_color(move_from, board.clone()))
                        {
                            return false;
                        }
                        return true;
                    }
                }
            } else if get_color(move_from, board.clone()) != get_color(move_to, board.clone()) {
                if get_color(move_from, board.clone()) == "White" {
                    if (move_to == move_from - 7
                        && (move_from as f32 / 8 as f32).floor()
                            == (move_to as f32 / 8 as f32).floor() + 1 as f32)
                        || (move_to == move_from - 9
                            && (move_from as f32 / 8 as f32).floor()
                                == (move_to as f32 / 8 as f32).floor() + 1 as f32)
                    {
                        let mut new_board = board.clone();
                        new_board[move_to as usize] = new_board[move_from as usize];
                        new_board[move_from as usize] = " ";
                        if check(new_board, can_castle.clone())
                            .contains(&get_color(move_from, board.clone()))
                        {
                            return false;
                        }
                        return true;
                    }
                } else {
                    if (move_to == move_from + 7
                        && (move_from as f32 / 8 as f32).floor()
                            == (move_to as f32 / 8 as f32).floor() - 1 as f32)
                        || (move_to == move_from + 9
                            && (move_from as f32 / 8 as f32).floor()
                                == (move_to as f32 / 8 as f32).floor() - 1 as f32)
                    {
                        let mut new_board = board.clone();
                        new_board[move_to as usize] = new_board[move_from as usize];
                        new_board[move_from as usize] = " ";
                        if check(new_board, can_castle.clone())
                            .contains(&get_color(move_from, board.clone()))
                        {
                            return false;
                        }
                        return true;
                    }
                }
            }
        }
        "King" => {
            if ((move_to == move_from + 8
                || move_to == move_from - 8
                || move_to == move_from - 1
                || move_to == move_from + 1)
                && (get_color(move_from, board.clone()) != get_color(move_to, board.clone())))
                || (get_color(move_from, board.clone()) == "White"
                    && (move_to == 58 || move_to == 62)
                    && can_castle.contains(&move_to))
                || (get_color(move_from, board.clone()) == "Black"
                    && (move_to == 2 || move_to == 6)
                    && can_castle.contains(&move_to))
            {
                if check(
                    move_piece_fn(move_to, move_from, board.clone(), can_castle.clone()),
                    can_castle.clone(),
                )
                .contains(&get_color(move_from, board.clone()))
                {
                    return false;
                }
                return true;
            }
        }
        "Knight" => {
            let mut poses = Vec::new();
            if (((move_from + 10) as f32) / 8 as f32).floor()
                == (move_from as f32 / 8 as f32).floor() + 1 as f32
            {
                poses.push(move_from + 10);
            }
            if (((move_from - 10) as f32) / 8 as f32).floor()
                == (move_from as f32 / 8 as f32).floor() - 1 as f32
            {
                poses.push(move_from - 10);
            }
            if (((move_from - 6) as f32) / 8 as f32).floor()
                == (move_from as f32 / 8 as f32).floor() - 1 as f32
            {
                poses.push(move_from - 6);
            }
            if (((move_from + 6) as f32) / 8 as f32).floor()
                == (move_from as f32 / 8 as f32).floor() + 1 as f32
            {
                poses.push(move_from + 6);
            }
            if (((move_from + 17) as f32) / 8 as f32).floor()
                == (move_from as f32 / 8 as f32).floor() + 2 as f32
            {
                poses.push(move_from + 17);
            }
            if (((move_from + 15) as f32) / 8 as f32).floor()
                == (move_from as f32 / 8 as f32).floor() + 2 as f32
            {
                poses.push(move_from + 15);
            }
            if (((move_from - 15) as f32) / 8 as f32).floor()
                == (move_from as f32 / 8 as f32).floor() - 2 as f32
            {
                poses.push(move_from - 15);
            }
            if (((move_from - 17) as f32) / 8 as f32).floor()
                == (move_from as f32 / 8 as f32).floor() - 2 as f32
            {
                poses.push(move_from - 17);
            }
            if poses.contains(&move_to)
                && (get_color(move_from, board.clone()) != get_color(move_to, board.clone()))
            {
                let mut new_board = board.clone();
                new_board[move_to as usize] = new_board[move_from as usize];
                new_board[move_from as usize] = " ";
                if check(new_board, can_castle.clone())
                    .contains(&get_color(move_from, board.clone()))
                {
                    return false;
                }
                return true;
            }
        }
        "Rook" => {
            let mut poses = Vec::new();
            let mut pos = move_from + 1;
            while pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor() == ((pos - 1) as f32 / 8 as f32).floor()
                && (pos + 1) % 8 != 0
                && get_color(pos, board.clone()) == "Empty"
            {
                poses.push(pos);
                pos += 1;
            }
            if pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor() == ((pos - 1) as f32 / 8 as f32).floor()
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            let mut pos = move_from + 8;
            while pos >= 0 && pos < 64 && get_color(pos, board.clone()) == "Empty" {
                poses.push(pos);
                pos += 8;
            }
            if pos >= 0
                && pos < 64
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            let mut pos = move_from - 8;
            while pos >= 0 && get_color(pos, board.clone()) == "Empty" {
                poses.push(pos);
                pos -= 8;
            }
            if pos >= 0
                && pos < 64
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            let mut pos = move_from - 1;
            while pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor() == ((pos + 1) as f32 / 8 as f32).floor()
                && (pos % 8) != 0
                && get_color(pos, board.clone()) == "Empty"
            {
                poses.push(pos);
                pos -= 1;
            }
            if pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor() == ((pos + 1) as f32 / 8 as f32).floor()
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            if poses.contains(&move_to)
                && (get_color(move_from, board.clone()) != get_color(move_to, board.clone()))
            {
                let mut new_board = board.clone();
                new_board[move_to as usize] = new_board[move_from as usize];
                new_board[move_from as usize] = " ";
                if check(new_board, can_castle.clone())
                    .contains(&get_color(move_from, board.clone()))
                {
                    return false;
                }
                return true;
            }
        }
        "Bishop" => {
            let mut poses = Vec::new();
            let mut pos = move_from - 7;
            while pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos + 7) as f32 / 8 as f32).floor() - 1 as f32
                && get_color(pos, board.clone()) == "Empty"
            {
                poses.push(pos);
                pos -= 7;
            }
            if pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos + 7) as f32 / 8 as f32).floor() - 1 as f32
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            let mut pos = move_from + 7;
            while pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos - 7) as f32 / 8 as f32).floor() + 1 as f32
                && get_color(pos, board.clone()) == "Empty"
            {
                poses.push(pos);
                pos += 7;
            }
            if pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos - 7) as f32 / 8 as f32).floor() + 1 as f32
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            let mut pos = move_from + 9;
            while pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos - 9) as f32 / 8 as f32).floor() + 1 as f32
                && get_color(pos, board.clone()) == "Empty"
            {
                poses.push(pos);
                pos += 9;
            }
            if pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos - 9) as f32 / 8 as f32).floor() + 1 as f32
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            let mut pos = move_from - 9;
            while pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos - 9) as f32 / 8 as f32).floor() + 1 as f32
                && get_color(pos, board.clone()) == "Empty"
            {
                poses.push(pos);
                pos -= 9;
            }
            if pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos + 9) as f32 / 8 as f32).floor() - 1 as f32
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            if poses.contains(&move_to)
                && (get_color(move_from, board.clone()) != get_color(move_to, board.clone()))
            {
                let mut new_board = board.clone();
                new_board[move_to as usize] = new_board[move_from as usize];
                new_board[move_from as usize] = " ";
                if check(new_board, can_castle.clone())
                    .contains(&get_color(move_from, board.clone()))
                {
                    return false;
                }
                return true;
            }
        }
        "Queen" => {
            let mut poses = Vec::new();
            let mut pos = move_from - 7;
            while pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos + 7) as f32 / 8 as f32).floor() - 1 as f32
                && get_color(pos, board.clone()) == "Empty"
            {
                poses.push(pos);
                pos -= 7;
            }
            if pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos + 7) as f32 / 8 as f32).floor() - 1 as f32
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            let mut pos = move_from + 7;
            while pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos - 7) as f32 / 8 as f32).floor() + 1 as f32
                && get_color(pos, board.clone()) == "Empty"
            {
                poses.push(pos);
                pos += 7;
            }
            if pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos - 7) as f32 / 8 as f32).floor() + 1 as f32
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            let mut pos = move_from + 9;
            while pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos - 9) as f32 / 8 as f32).floor() + 1 as f32
                && get_color(pos, board.clone()) == "Empty"
            {
                poses.push(pos);
                pos += 9;
            }
            if pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos - 9) as f32 / 8 as f32).floor() + 1 as f32
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            let mut pos = move_from - 9;
            while pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos + 9) as f32 / 8 as f32).floor() - 1 as f32
                && get_color(pos, board.clone()) == "Empty"
            {
                poses.push(pos);
                pos -= 9;
            }
            if pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor()
                    == ((pos + 9) as f32 / 8 as f32).floor() - 1 as f32
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            let mut pos = move_from + 1;
            while pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor() == ((pos - 1) as f32 / 8 as f32).floor()
                && (pos + 1) % 8 != 0
                && get_color(pos, board.clone()) == "Empty"
            {
                poses.push(pos);
                pos += 1;
            }
            if pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor() == ((pos - 1) as f32 / 8 as f32).floor()
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            let mut pos = move_from + 8;
            while pos >= 0 && pos < 64 && get_color(pos, board.clone()) == "Empty" {
                poses.push(pos);
                pos += 8;
            }
            if pos >= 0
                && pos < 64
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            let mut pos = move_from - 8;
            while pos >= 0 && get_color(pos, board.clone()) == "Empty" {
                poses.push(pos);
                pos -= 8;
            }
            if pos >= 0
                && pos < 64
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            let mut pos = move_from - 1;
            while pos >= 0
                && pos < 64
                && (pos as f32 / 8 as f32).floor() == ((pos + 1) as f32 / 8 as f32).floor()
                && (pos % 8) != 0
                && get_color(pos, board.clone()) == "Empty"
            {
                poses.push(pos);
                pos -= 1;
            }
            if pos >= 0
                && pos < 64
                && get_color(pos, board.clone()) != get_color(move_from, board.clone())
            {
                poses.push(pos);
            }
            if poses.contains(&move_to)
                && (get_color(move_from, board.clone()) != get_color(move_to, board.clone()))
            {
                let mut new_board = board.clone();
                new_board[move_to as usize] = new_board[move_from as usize];
                new_board[move_from as usize] = " ";
                if check(new_board.clone(), can_castle.clone())
                    .contains(&get_color(move_from, board.clone()))
                {
                    return false;
                }
                return true;
            }
        }
        _ => {}
    }
    return false;
}

fn checkmate(board: Vec<&str>, color: &str, can_castle: Vec<i32>) -> bool {
    for move_from in 0..63 {
        if color == get_color(move_from, board.clone()) {
            for move_to in 0..63 {
                if is_move_valid(move_from, move_to, board.clone(), can_castle.clone()) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn check(board: Vec<&str>, can_castle: Vec<i32>) -> Vec<&str> {
    let mut returns = Vec::new();
    for find_king in 0..63 {
        if get_piece(find_king, board.clone()) == "King" {
            for get_all_pieces in 0..63 {
                if get_color(find_king, board.clone()) != get_color(get_all_pieces, board.clone())
                    && is_move_valid(get_all_pieces, find_king, board.clone(), can_castle.clone())
                {
                    returns.push(get_color(find_king, board.clone()));
                }
            }
        }
    }
    returns
}

fn move_piece_fn(
    move_to: i32,
    move_from: i32,
    board: Vec<&str>,
    can_castle: Vec<i32>,
) -> Vec<&str> {
    if get_piece(move_from, board.clone()) == "King"
        && ((get_color(move_from, board.clone()) == "Black"
            && move_from == 4
            && can_castle.contains(&move_to))
            || (get_color(move_from, board.clone()) == "White"
                && move_from == 60
                && can_castle.contains(&move_to)))
    {
        let mut new_board = board.clone();
        if move_to % 8 == 2 {
            new_board[(move_to + 1) as usize] = new_board[(move_to - 2) as usize];
            new_board[(move_to - 2) as usize] = " ";
        } else {
            new_board[(move_to - 1) as usize] = new_board[(move_to + 1) as usize];
            new_board[(move_to + 1) as usize] = " ";
        }
        new_board[move_to as usize] = new_board[move_from as usize];
        new_board[move_from as usize] = " ";
        new_board
    } else {
        let mut new_board = board.clone();
        new_board[move_to as usize] = new_board[move_from as usize];
        new_board[move_from as usize] = " ";
        new_board
    }
}
