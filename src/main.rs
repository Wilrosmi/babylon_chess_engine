/*
    Todo 
        - Make basic eval and search
        - Test coverage
        - Refactor generally, but specifically to use better error handling (especially dealing with user input)
        - Split the project out into game functions library and a CLI library
        - Develop the search and evaluation further
        - Make a basic website to play versus the AI only
        - Make the website so you can play versus others
*/

use rand::seq::SliceRandom;
use std::{
    thread,
    io::{ stdin, stdout, Write }, 
    fmt, ops::{Neg, Index}
};

#[derive(Copy, Clone)]
struct Board {
    board: [SquareVal; 63]
}

#[derive(Copy, Clone)]
enum SquareVal {
    Invalid,
    Empty, 
    Piece(Piece),
}

#[derive(Copy, Clone)]
struct Piece {
    colour: Colour,
    kind: Kind
}

#[derive(Copy, Clone, PartialEq)]
enum Colour {
    White, 
    Black
}

#[derive(PartialEq, Copy, Clone)]
enum Kind {
    Pawn,
    Knight
}

#[derive(Copy, Clone)]
struct Move {
    from_square: u8,
    to_square: u8
}

#[derive(PartialEq)]
enum MoveType {
    Attack,
    MoveOnly
}

struct MovePair {
    white: Move,
    black: Move
}

enum GameState {
    Ongoing,
    Draw,
    Win(Colour)
}

macro_rules! WhitePawn {
    () => {
        SquareVal::Piece(Piece {
            colour: Colour::White,
            kind: Kind::Pawn
        })
    };
}

macro_rules! WhiteKnight {
    () => {
        SquareVal::Piece(Piece {
            colour: Colour::White,
            kind: Kind::Knight
        })
    };
}

macro_rules! BlackPawn {
    () => {
        SquareVal::Piece(Piece {
            colour: Colour::Black,
            kind: Kind::Pawn
        })
    };
}

macro_rules! BlackKnight {
    () => {
        SquareVal::Piece(Piece {
            colour: Colour::Black,
            kind: Kind::Knight
        })
    };
}

const PAWN_VALUES_WHITE: [f64; 63] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 1.2, 1.4, 1.2, 1.0, 0.0,
    0.0, 1.2, 1.4, 1.6, 1.4, 1.2, 0.0,
    0.0, 1.4, 1.6, 1.8, 1.6, 1.4, 0.0,
    0.0, 1.6, 1.8, 2.0, 1.8, 1.6, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
];

const PAWN_VALUES_BLACK: [f64; 63] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 1.6, 1.8, 2.0, 1.8, 1.6, 0.0,
    0.0, 1.4, 1.6, 1.8, 1.6, 1.4, 0.0,
    0.0, 1.2, 1.4, 1.6, 1.4, 1.2, 0.0,
    0.0, 1.0, 1.2, 1.4, 1.2, 1.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0
];

const KNIGHT_VALUES: [f64; 63] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 3.0, 3.2, 3.4, 3.2, 3.0, 0.0,
    0.0, 3.2, 3.4, 3.6, 3.4, 3.2, 0.0,
    0.0, 3.4, 3.6, 3.8, 3.6, 3.4, 0.0,
    0.0, 3.2, 3.4, 3.6, 3.4, 3.2, 0.0,
    0.0, 3.0, 3.2, 3.4, 3.2, 3.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
]; 

fn main() {
    let mut board = Board::new();
    loop {
        stdout().flush().unwrap();
        println!("{}", board);
        let user_move = thread::spawn(move || {
            get_user_move(&board)
        });
        let computer_move = thread::spawn(move || {
            board.get_move(Colour::Black).unwrap()
        });
        let move_pair = MovePair {
            white: user_move.join().unwrap(),
            black: computer_move.join().unwrap()
        };
        board.execute_moves(move_pair);
        let result = board.get_game_state();
        match result {
            GameState::Draw => {
                println!("{}", board);
                println!("Its a draw!");
                break;
            },
            GameState::Win(colour) => {
                println!("{}", board);
                println!("{} wins!", colour);
                break;
            }
            GameState::Ongoing => (),    
        };
    }
    println!("Thanks for playing");
}

impl Board {
    fn new() -> Board {
        Board {
            board: [SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid,
                    SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid,
                    SquareVal::Invalid, BlackKnight!(), BlackPawn!(), BlackPawn!(), BlackPawn!(), BlackKnight!(), SquareVal::Invalid,
                    SquareVal::Invalid, BlackPawn!(), SquareVal::Empty, SquareVal::Empty, SquareVal::Empty, BlackPawn!(), SquareVal::Invalid,
                    SquareVal::Invalid, SquareVal::Empty, SquareVal::Empty, SquareVal::Empty, SquareVal::Empty, SquareVal::Empty, SquareVal::Invalid,
                    SquareVal::Invalid, WhitePawn!(), SquareVal::Empty, SquareVal::Empty, SquareVal::Empty, WhitePawn!(), SquareVal::Invalid,
                    SquareVal::Invalid, WhiteKnight!(), WhitePawn!(), WhitePawn!(), WhitePawn!(), WhiteKnight!(), SquareVal::Invalid,
                    SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid,
                    SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid, SquareVal::Invalid]
        }
    }
    fn get_move(&self, colour: Colour) -> Option<Move> {
        // let all_cmoves = self.get_all_legal_moves(colour);
        // let best_move = all_moves.choose(&mut rand::thread_rng());
        // match best_move {
        //     Some(mov) => Some(*mov),
        //     None => None
        // }
        let all_our_moves = self.get_all_legal_moves(colour);
        let all_opponent_moves = self.get_all_legal_moves(-colour);
        let all_possible_board_values = 
            all_our_moves
            .iter()
            .map(|our_move| self.get_boards_possible_for_move(our_move, &colour, &all_opponent_moves))
            .map(|all_possible_move_outcomes| {
                all_possible_move_outcomes
                    .iter()
                    .map(|board| board.get_value(&-colour))
                    .collect()    
            })
            .collect();
                        
        /*
            steps for finding one move deep nash eq

            * get all legal moves for computer
            * get all legal moves for opposition
            * get the board that occurs for every move pair
            assign a value to each board from our opponents perspective, using some valuation function
            assign s/i as the probability for each choice i of all the choices 0..n-1 we have available. n has prob 1 - SUM(s/i for all i).
            calculate the expected value to our opponent of each of their moves in terms of the n-1 prob. varaibles. This is n equations.
            set each of these equal to some variable x, as in nash eq. they must all have the same expected value.
            this is now a n-variable, n-equation simultaneuos equation problem.
            solve the set of equations to get the probability distribution over our move choices.
            choose a move according to that probability distribution.  
        */
    }
    fn get_all_legal_moves(&self, colour: Colour) -> Vec<Move> {
        self.board
            .iter()
            .enumerate()
            .map(|(index, square_val)| {
                // Index should never be greater than 62 and if it is, something has gone unrecoverably wrong
                if index > 62 { panic!("Attempted to evaluate a board that was too large") };
                let u8_index = index as u8;
                match square_val {
                    SquareVal::Piece(piece) => {
                        if piece.colour == colour {
                            self.get_piece_moves(piece, u8_index)
                        } else {
                            vec![]
                        }
                    },
                    _ => vec![]
                }
            })
            .flatten()
            .collect()
    } 
    fn get_piece_moves(&self, piece: &Piece, index: u8) -> Vec<Move> {
        match piece.kind {
            Kind::Pawn => self.get_pawn_moves(piece, index),
            Kind::Knight => self.get_knight_moves(piece, index)
        }
    }
    fn get_pawn_moves(&self, piece: &Piece, index: u8) -> Vec<Move> {
        let mut legal_moves: Vec<Move> = vec![];
        if self.is_legal_pawn_move(&Move {from_square: index.clone(), to_square: index + 7 }, MoveType::MoveOnly, &piece.colour) { 
            legal_moves.push(Move { from_square: index, to_square: index + 7 }) 
        };
        if self.is_legal_pawn_move(&Move {from_square: index.clone(), to_square: index + 6 }, MoveType::Attack, &piece.colour) { 
            legal_moves.push(Move { from_square: index, to_square: index + 6 }) 
        };
        if self.is_legal_pawn_move(&Move {from_square: index.clone(), to_square: index + 8 }, MoveType::Attack, &piece.colour) { 
            legal_moves.push(Move { from_square: index, to_square: index + 8 }) 
        };
        legal_moves
    }
    fn get_knight_moves(&self, piece: &Piece, index: u8) -> Vec<Move> {
        ([13, 15, -13, -15, 5, 9, -5, -9] as [i8; 8])
            .iter()
            .map(|mov| Move {
                from_square: index,
                to_square: (index as i8 + mov) as u8
            })
            .filter(|mov| self.is_legal_knight_move(mov, &piece.colour))
            .collect()
    }
    fn is_legal_pawn_move(&self, mov: &Move, mov_type: MoveType, piece_colour: &Colour) -> bool {
        match &self.board[mov.from_square as usize] {
            SquareVal::Piece(Piece {
                colour,
                kind
            }) => { if !(colour == piece_colour && kind == &Kind::Pawn) { return false }},
            _ => { return false }
        };
        match self.board[mov.to_square as usize] {
            SquareVal::Invalid => false,
            SquareVal::Empty => mov_type == MoveType::MoveOnly,
            SquareVal::Piece(Piece {
                colour,
                kind: _
            }) => colour != *piece_colour && mov_type == MoveType::Attack,
        }
    }
    fn is_legal_knight_move(&self, mov: &Move, piece_colour: &Colour) -> bool {
        match &self.board[mov.from_square as usize] {
            SquareVal::Piece(Piece {
                colour,
                kind
            }) => { if !(colour == piece_colour && kind == &Kind::Knight) { return false } },
            _ => { return false }
        };
        match self.board[mov.to_square as usize] {
            SquareVal::Invalid => false,
            SquareVal::Empty => true,
            SquareVal::Piece(Piece {
                colour,
                kind: _
            }) => colour != *piece_colour,
        }
    }
    fn execute_moves(&mut self, mov_pair: MovePair) {
        if mov_pair.white.to_square == mov_pair.black.to_square {
            self.execute_moves_to_same_square(mov_pair);
        } else {
            self.execute_moves_to_different_squares(mov_pair);
        }
    }
    fn execute_moves_to_different_squares(&mut self, mov_pair: MovePair) {
        let white_piece = self.board[mov_pair.white.from_square as usize];
        let black_piece = self.board[mov_pair.black.from_square as usize]; 
        self.board[mov_pair.white.from_square as usize] = SquareVal::Empty;
        self.board[mov_pair.black.from_square as usize] = SquareVal::Empty;
        self.board[mov_pair.white.to_square as usize] = piece_to_place(white_piece, &Colour::White, mov_pair.white.to_square);
        self.board[mov_pair.black.to_square as usize] = piece_to_place(black_piece, &Colour::Black, mov_pair.black.to_square);
    }
    fn execute_moves_to_same_square(&mut self, mov_pair: MovePair) {
        let SquareVal::Piece(white_piece) = self.board[mov_pair.white.from_square as usize] else { panic!() };
        let SquareVal::Piece(black_piece) = self.board[mov_pair.black.from_square as usize] else { panic!() }; 
        if white_piece.kind == black_piece.kind {
            self.board[mov_pair.white.from_square as usize] = SquareVal::Empty;
            self.board[mov_pair.black.from_square as usize] = SquareVal::Empty;
            self.board[mov_pair.white.to_square as usize] = SquareVal::Empty;
        } else {
            let winner = if white_piece.kind == Kind::Knight { white_piece } else { black_piece };
            self.board[mov_pair.white.from_square as usize] = SquareVal::Empty;
            self.board[mov_pair.black.from_square as usize] = SquareVal::Empty;
            self.board[mov_pair.white.to_square as usize] = SquareVal::Piece(winner);
        }
    }
    fn get_game_state(&self) -> GameState {
        if self.is_game_drawn() {
            GameState::Draw
        } else if let Some(colour) = self.try_get_winner() {
            GameState::Win(colour)
        } else {
            GameState::Ongoing
        }
    }
    fn is_game_drawn(&self) -> bool {
        self.both_sides_no_footmen() || self.has_no_moves(&Colour::Black) || self.has_no_moves(&Colour::Black)
    }
    fn both_sides_no_footmen(&self) -> bool {
        self.has_no_footmen(Colour::White) && self.has_no_footmen(Colour::Black)
    }
    fn has_no_footmen(&self, colour: Colour) -> bool {
        for square in self.board.iter() {
            if is_right_colour_footman(square, &colour) {
                return false;
            } 
        }
        true
    }
    fn has_no_moves(&self, colour: &Colour) -> bool {
        self.get_all_legal_moves(*colour).len() == 0
    }
    fn try_get_winner(&self) -> Option<Colour> {
        if self.has_no_footmen(Colour::White) {
            Some(Colour::Black)
        } else if self.has_no_footmen(Colour::Black) {
            Some(Colour::White) 
        } else {
            None
        }
    }
    fn is_legal_move(&self, mov: &Move, colour: &Colour) -> bool {
        let SquareVal::Piece(piece) = self.board[mov.from_square as usize] else {
            return false;
        };
        if is_invalid_movement(mov, &piece) {
            return false;
        };
        match piece.kind {
            Kind::Knight => self.is_legal_knight_move(mov, colour),
            Kind::Pawn => {
                let mov_type = get_move_type(mov);
                self.is_legal_pawn_move(mov, mov_type, colour)
            }
        }
         
    }
    fn get_boards_possible_for_move(&self, our_move: &Move, our_colour: &Colour, all_opponent_moves: &Vec<Move>) -> Vec<Board> { 
        all_opponent_moves
            .iter()
            .map(|opp_move| {
                let white_move = match *our_colour {
                    Colour::White => our_move,
                    Colour::Black => opp_move
                };
                let black_move = match *our_colour {
                    Colour::White => opp_move,
                    Colour::Black => our_move
                };
                let mov_pair = MovePair {
                    white: *white_move,
                    black: *black_move
                };
                let mut new_board = self.clone();
                new_board.execute_moves(mov_pair);
                new_board
            })
            .collect()                
    }
    fn get_value(&self, colour: &Colour) -> f64 {
        // let our_pawn_board = match colour {
        //     &Colour::White => PAWN_VALUES_WHITE,
        //     &Colour::Black => PAWN_VALUES_BLACK
        // };
        // let opp_pawn_board = match colour {
        //     &Colour::White => PAWN_VALUES_BLACK,
        //     &Colour::Black => PAWN_VALUES_WHITE
        // }; 
        self.board
            .iter()
            .enumerate()
            .fold(0.0, |acc, ele| acc + get_square_val(ele, colour))
    }
} 

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Colour::White => write!(f, "White"),
            Colour::Black => write!(f, "Black")
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                
        let board_height = 5;
        let board_width = 5;

        for row in 0..board_height {
            for col in 0..board_width {
                let one_d_coordinate = grid_to_one_d(row, col);
                let square_val = self.board[one_d_coordinate];
                write!(f, "{}", square_val).unwrap();
            }
            writeln!(f).unwrap();
        }
        writeln!(f)
    }
}

impl fmt::Display for SquareVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SquareVal::Piece(piece) => write!(f, "{}", piece),
            SquareVal::Empty => write!(f, " "),
            SquareVal::Invalid => write!(f, "")
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.colour {
            Colour::White => match self.kind {
                Kind::Knight => write!(f, "♞"),
                Kind::Pawn => write!(f, "♟")
            },
            Colour::Black => match self.kind {
                Kind::Knight => write!(f, "♘"),
                Kind::Pawn => write!(f, "♙")
            }
        }
    }
}
    
impl Neg for Colour {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Colour::Black => Colour::White,
            Colour::White => Colour::Black
        }
    }
}

fn get_square_val(element: (usize, &SquareVal), colour: &Colour) -> f64 {
    match element.1 {
        SquareVal::Piece(piece) => get_piece_val(piece, element.0, colour),
        _ => 0.0
    }
}

fn get_piece_val(piece: &Piece, index: usize, colour: &Colour) -> f64 {
    let absolute_piece_value = get_absolute_piece_value(piece, index);
    if piece.colour == *colour {
        absolute_piece_value
    } else {
        absolute_piece_value * (-1 as f64)
    }
}

fn get_absolute_piece_value(piece: &Piece, index: usize) -> f64 {
    match piece.kind {
        Kind::Knight => KNIGHT_VALUES[index],
        Kind::Pawn => get_pawn_val(piece, index)
    }
}

fn get_pawn_val(piece: &Piece, index: usize) -> f64 {
    match piece.colour {
        Colour::White => PAWN_VALUES_WHITE[index],
        Colour::Black => PAWN_VALUES_BLACK[index]
    }
}

fn piece_to_place(square_val: SquareVal, colour: &Colour, to_square: u8) -> SquareVal {
    // This should never be anything other than a piece. Todo - make that relationship explicit.
    let SquareVal::Piece(piece) = square_val else { panic!() };
    let end_row = match *colour {
        Colour::White => [15, 16, 17, 18, 19],
        Colour::Black => [43, 44, 45, 46, 47]
    };
    if piece.kind == Kind::Pawn && end_row.contains(&(to_square as isize)) {
        SquareVal::Piece(Piece {
            colour: *colour,
            kind: Kind::Knight
        })
    } else {
        square_val
    }
}

fn is_invalid_movement(mov: &Move, piece: &Piece) -> bool {
    match piece.kind {
        Kind::Knight => is_invalid_knight_movement(mov),
        Kind::Pawn => is_invalid_pawn_movement(mov, &piece.colour)
    }
}

fn is_invalid_knight_movement(mov: &Move) -> bool {
    [13, 15, -13, -15, 5, 9, -5, -9].contains(&((mov.to_square - mov.from_square) as isize))
}

fn is_invalid_pawn_movement(mov: &Move, colour: &Colour) -> bool {
    let valid_moves = if *colour == Colour::White {
        [6, 7, 8]
    } else {
        [-6, -7, -8]
    };
    valid_moves.contains(&((mov.to_square - mov.from_square) as isize))
}

fn get_move_type(mov: &Move) -> MoveType {
    if mov.to_square - mov.from_square == 7 {
        MoveType::MoveOnly
    } else {
        MoveType::Attack
    }
}

fn grid_to_one_d(row: usize, col: usize) -> usize {
    15 + (7 * row) + (col)
}

fn is_right_colour_footman(square: &SquareVal, colour: &Colour) -> bool {
    match square {
        SquareVal::Piece(piece) => piece.kind == Kind::Pawn && piece.colour == *colour,
        _ => false
    }
}

fn get_user_move(board: &Board) -> Move {
    let Some(from_square) = get_square("Choose which square to move from (using A1-E5)") else {
        println!("Invalid square entered. Please try again.");
        return get_user_move(board);
    };
    let Some(to_square) = get_square("Choose which square to move to (using A1-E5)") else {
        println!("Invalid square entered. Please try again.");
        return get_user_move(board);
    };
    let mov =  Move {
        from_square,
        to_square
    };
    if board.is_legal_move(&mov, &Colour::White) {
        mov
    } else {
        println!("Invalid square entered. Please try again.");
        get_user_move(board)
    }
}

fn get_square(prompt: &str) -> Option<u8> {
    let mut s = String::new();
    println!("{}", prompt);
    let result = stdin().read_line(&mut s);
    match result {
        Err(_) => { return None },
        _ => ()
    };
    let mov = try_get_u8_from_algebraic(s);
    match mov {
        Some(val) => Some(val),
        None => None
    }
}

fn try_get_u8_from_algebraic(s: String) -> Option<u8> {
    let mut chars = s.chars();
    let row_string = chars.next().unwrap(); 
    let Some(row_val) = try_get_row(row_string) else { return None };
    let col_string = chars.next().unwrap();
    let Some(col_val) = try_get_col(col_string) else { return None };
    Some(row_val - (7 * (col_val - 1)))
}

fn try_get_row(row: char) -> Option<u8> {
    match row.to_string().to_lowercase().as_str() {
        "a" => Some(43),
        "b" => Some(44),
        "c" => Some(45),
        "d" => Some(46),
        "e" => Some(47),
        _ => None
    }
}

fn try_get_col(col: char) -> Option<u8> {
    match col.to_string().parse::<u8>() {
        Ok(num) => Some(num),
        Err(_) => None
    }
}
