/*
    Todo 
        - Make basic CLI
            - Code to handle game state
            - Code to handle user inputs
            - Code to display the board in the command line
        - Make basic board valuations and check one move deep
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
    fmt
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

fn main() {
    let mut board = Board::new();
    loop {
        stdout().flush().unwrap();
        println!("{}", board);
        let user_move = thread::spawn(|| {
            get_user_move()
        });
        let computer_move = thread::spawn(move || {
            board.get_best_move(Colour::Black).unwrap()
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
    fn get_best_move(&self, colour: Colour) -> Option<Move> {
        let all_moves = self.get_all_legal_moves(colour);
        let best_move = all_moves.choose(&mut rand::thread_rng());
        match best_move {
            Some(mov) => Some(*mov),
            None => None
        }
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
        // Note - Doesn't implement en passant yet
        let mut legal_moves: Vec<Move> = vec![];
        if self.is_legal_pawn_move(Move {from_square: index.clone(), to_square: index + 7 }, MoveType::MoveOnly, &piece.colour) { 
            legal_moves.push(Move { from_square: index, to_square: index + 7 }) 
        };
        if self.is_legal_pawn_move(Move {from_square: index.clone(), to_square: index + 6 }, MoveType::Attack, &piece.colour) { 
            legal_moves.push(Move { from_square: index, to_square: index + 6 }) 
        };
        if self.is_legal_pawn_move(Move {from_square: index.clone(), to_square: index + 8 }, MoveType::Attack, &piece.colour) { 
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
    fn is_legal_pawn_move(&self, mov: Move, mov_type: MoveType, piece_colour: &Colour) -> bool {
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
        self.board[mov_pair.white.to_square as usize] = white_piece;
        self.board[mov_pair.black.to_square as usize] = black_piece;
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
        self.both_sides_no_footmen() || (self.has_no_moves(&Colour::Black) && self.has_no_moves(&Colour::Black))
    }
    fn both_sides_no_footmen(&self) -> bool {
        self.has_no_footmen(Colour::White) && self.has_no_footmen(Colour::Black)
    }
    fn has_no_footmen(&self, colour: Colour) -> bool {
        for square in self.board.iter() {
            if is_right_colour_footman(square, &colour) {
                return true;
            } 
        }
        false
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
        
        write!(f, "╔════════╗").unwrap();
        
        let board_height = 5;
        let board_width = 5;

        for row in 0..board_height {
            writeln!(f).unwrap();
            write!(f, "║").unwrap();            
            for col in 0..board_width {
                let one_d_coordinate = grid_to_one_d(row, col);
                let square_val = self.board[one_d_coordinate];
                write!(f, "{}", square_val).unwrap();
            }
            write!(f, "║").unwrap();
        }

        write!(f, " ╚════════╝")
    }
}

impl fmt::Display for SquareVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SquareVal::Piece(piece) => write!(f, "{}", piece),
            SquareVal::Empty => write!(f, "▓"),
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

fn grid_to_one_d(row: usize, col: usize) -> usize {
    8 + (7 * row) + (col)
}

fn is_right_colour_footman(square: &SquareVal, colour: &Colour) -> bool {
    match square {
        SquareVal::Piece(piece) => piece.kind == Kind::Pawn && piece.colour == *colour,
        _ => false
    }
}

// check this a legal move
fn get_user_move() -> Move {
    let Some(from_square) = get_square("Choose which square to move from (using A1-E5") else {
        println!("Invalid square entered. Please try again.");
        return get_user_move();
    };
    let Some(to_square) = get_square("Choose which square to move to (using A1-E5)") else {
        println!("Invalid square entered. Please try again.");
        return get_user_move();
    };
    return Move {
        from_square,
        to_square
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
