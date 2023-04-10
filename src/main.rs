/*
    Todo 
        - Deal with the colours of the pieces correctly
            (Need to refactor the match statements, as they don't work how I expected)
        - Test coverage
        - Make basic CLI
            - Code to handle game state
            - Code to handle user inputs
            - Code to display the board in the command line
        - Make basic board valuations and check one move deep
        - Make a basic website to play versus the AI only
        - Make the website so you can play versus others
        - Develop the search and evaluation further
*/

use rand::seq::SliceRandom;

struct Board {
    board: [SquareVal; 63]
}

enum SquareVal {
    Invalid,
    Empty, 
    Piece(Piece),
}

struct Piece {
    colour: Colour,
    kind: Kind
}

#[derive(Copy, Clone)]
enum Colour {
    White, 
    Black
}

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
    let board = Board::new();
    let best_move = board.get_best_move();
    match best_move {
      Some(mov) => println!("The best move is from {} to {}", mov.from_square, mov.to_square),
      None => println!("There are no legal moves available")  
    }
       
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
    fn get_best_move(&self) -> Option<Move> {
        let all_moves = self.get_all_legal_moves();
        let best_move = all_moves.choose(&mut rand::thread_rng());
        match best_move {
            Some(mov) => Some(*mov),
            None => None
        }
    }
    fn get_all_legal_moves(&self) -> Vec<Move> {
        self.board
            .iter()
            .enumerate()
            .map(|(index, square_val)| {
                // Index should never be greater than 62 and if it is, something has gone unrecoverably wrong
                if index > 62 { panic!("Attempted to evaluate a board that was too large") };
                let u8_index = index as u8;
                match square_val {
                    SquareVal::Piece(piece) => self.get_piece_moves(piece, u8_index),
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
    // This should check the colour of the mover
    fn is_legal_pawn_move(&self, mov: Move, mov_type: MoveType, piece_colour: &Colour) -> bool {
        match self.board[mov.from_square as usize] {
            SquareVal::Piece(Piece {
                colour: piece_colour,
                kind: Kind::Pawn
            }) => (),
            _ => { return false }
        };
        match self.board[mov.to_square as usize] {
            SquareVal::Invalid => false,
            SquareVal::Empty => mov_type == MoveType::MoveOnly,
            SquareVal::Piece(Piece {
                colour: piece_colour,
                kind: _
            }) => false,
            SquareVal::Piece(_) => mov_type == MoveType::Attack
        }
    }
    // This should check the colour of the mover
    fn is_legal_knight_move(&self, mov: &Move, piece_colour: &Colour) -> bool {
        match self.board[mov.from_square as usize] {
            SquareVal::Piece(Piece {
                colour: piece_colour,
                kind: Kind::Knight
            }) => (),
            _ => { return false }
        };
        match self.board[mov.to_square as usize] {
            SquareVal::Invalid => false,
            SquareVal::Empty => true,
            SquareVal::Piece(Piece {
                colour: piece_colour,
                kind: _
            }) => false,
            SquareVal::Piece(_) => true
        }
    }
}
