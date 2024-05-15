fn cc(c : u32, o : u32) -> char {
    let x : usize = ((c >> (o * 2)) & 3) as usize;
    if x == 0 {
        return (o as u8 + 49 as u8) as char;
    }
    return " XO?".as_bytes()[x] as char;
}

fn print_board(board : u32){
    println!("{}|{}|{}", cc(board, 0), cc(board, 1), cc(board, 2));
    println!("-----");
    println!("{}|{}|{}", cc(board, 3), cc(board, 4), cc(board, 5));
    println!("-----");
    println!("{}|{}|{}", cc(board, 6), cc(board, 7), cc(board, 8));
}

fn set_m(board : u32, pos : u8, player : u8) -> u32 {
    return board ^ (1 << (pos * 2 + player));
}

fn is_empty(board : u32, pos : u8) -> bool {
    return (board >> (pos * 2) & 3) == 0;
}

fn board_empty(board : u32){
    let mut r : bool = true;
    for pos in 0..9 {
        r &= !is_empty(board, pos);
    }
    return r;
}

fn get_w(board : u32) -> u8 {
    for i in 0..3 {
        if (board >> (i * 6)) & 21 == 21 {
            return 1;
        }

        if board & (4161 << i * 2) == (4161 << i * 2) {
            return 1;
        }

        if (board >> (i * 6)) & 42 == 42 {
            return 2;
        }

        if board & (8322 << i * 2) == ( 8322 << i * 2) {
            return 2;
        }
    }

    for i in 0..2 {
        if board & (65793 << i) == (65793 << i) {
            return i + 1;
        }

        if board & (273 << i) == (273 << i) {
           return i + 1;
        }
    }

    return 0;
}

fn main(){

    let mut board : u32 = 0;

    let mut cplayer : u8 = 0;

    print_board(board);

    while (get_w(board) == 0) && !board_empty(board) {
        
        let mut input_line = String::new();

        std::io::stdin().read_line(&mut input_line)
            .expect("Failed to read line");

        let mut input_num : u8 = match input_line.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        input_num -= 1;

        if input_num > 9 { continue; }
        if !is_empty(board, input_num) { continue; }

        board = set_m(board, input_num, cplayer);

        print_board(board);

        cplayer = !cplayer;
    }
}
