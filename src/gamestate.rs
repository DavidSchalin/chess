use crate::piece::{ChessPiece, Color, PieceType};
use serde::{Serialize, Deserialize};

use serde_big_array::{BigArray};


const WHITE: Color = Color::WHITE;
const BLACK: Color = Color::BLACK;
const UNCOLORED: Color = Color::UNCOLORED;

#[derive(Serialize, Deserialize)]
pub struct GameState {
    #[serde(with = "BigArray")]
    pub board: [Option<ChessPiece>; 64],
    #[serde(with = "BigArray")]
    pub prev_board: [Option<ChessPiece>; 64],
    pub current_player: Color,
    pub checked_flag: bool,
    pub checked_player: Color,
    pub debug_flag: bool,
    pub castling_flag: bool,
    pub wkc: usize,
    pub bkc: usize,
    pub old_wkc: usize,
    pub old_bkc: usize,
}

impl GameState {
/// It creates a new GameState with a new board, with the current player being white, and with no one
/// checked
/// 
/// Returns:
/// 
/// A new GameState struct with a new board, current player set to white, and checked set to false.
    pub fn new() -> GameState {
        let new_board: [Option<ChessPiece>; 64] = [
            Some(ChessPiece::new(PieceType::ROOK(false), WHITE)), Some(ChessPiece::new(PieceType::KNIGHT, WHITE)), Some(ChessPiece::new(PieceType::BISHOP, WHITE)), Some(ChessPiece::new(PieceType::QUEEN, WHITE)), Some(ChessPiece::new(PieceType::KING(false), WHITE)), Some(ChessPiece::new(PieceType::BISHOP, WHITE)), Some(ChessPiece::new(PieceType::KNIGHT, WHITE)), Some(ChessPiece::new(PieceType::ROOK(false), WHITE)), 
            Some(ChessPiece::new(PieceType::PAWN(false), WHITE)), Some(ChessPiece::new(PieceType::PAWN(false), WHITE)), Some(ChessPiece::new(PieceType::PAWN(false), WHITE)), Some(ChessPiece::new(PieceType::PAWN(false), WHITE)), Some(ChessPiece::new(PieceType::PAWN(false), WHITE)), Some(ChessPiece::new(PieceType::PAWN(false), WHITE)), Some(ChessPiece::new(PieceType::PAWN(false), WHITE)), Some(ChessPiece::new(PieceType::PAWN(false), WHITE)),
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None,
            Some(ChessPiece::new(PieceType::PAWN(false), BLACK)), Some(ChessPiece::new(PieceType::PAWN(false), BLACK)), Some(ChessPiece::new(PieceType::PAWN(false), BLACK)), Some(ChessPiece::new(PieceType::PAWN(false), BLACK)), Some(ChessPiece::new(PieceType::PAWN(false), BLACK)), Some(ChessPiece::new(PieceType::PAWN(false), BLACK)), Some(ChessPiece::new(PieceType::PAWN(false), BLACK)), Some(ChessPiece::new(PieceType::PAWN(false), BLACK)),
            Some(ChessPiece::new(PieceType::ROOK(false), BLACK)), Some(ChessPiece::new(PieceType::KNIGHT, BLACK)), Some(ChessPiece::new(PieceType::BISHOP, BLACK)), Some(ChessPiece::new(PieceType::QUEEN, BLACK)), Some(ChessPiece::new(PieceType::KING(false), BLACK)), Some(ChessPiece::new(PieceType::BISHOP, BLACK)), Some(ChessPiece::new(PieceType::KNIGHT, BLACK)), Some(ChessPiece::new(PieceType::ROOK(false), BLACK)), 
        ];

        GameState {
            board: new_board,
            prev_board: new_board,
            current_player: Color::WHITE,
            checked_flag: false,
            checked_player: UNCOLORED,
            debug_flag: false,
            castling_flag: false,
            wkc: 4,
            bkc: 60,
            old_wkc: 4,
            old_bkc: 60
        }
    }

/// `serialize_me` takes a mutable reference to `self` and returns nothing
    pub fn serialize_me(&mut self) ->  String {
        let serialized: String = serde_json::to_string(&self).unwrap();
        println!("Serialized: '{:?}'", serialized);
        serialized
    }


    pub fn get_player_color_as_bool(&self) -> bool {
        return match self.current_player {
            WHITE => true,
            BLACK => false,
            UNCOLORED => {
                self.debug_print("WARNING! UNCOLORED RETURNED AS WHITE!");
                true
            }
        };
    }

    pub fn bool_to_color(b: bool) -> Color {
        return match b {
            true => WHITE,
            false => BLACK,
            _ => UNCOLORED
        };
    }

/// If the tile at the given index is occupied, print the piece type and color. Otherwise, print an
/// empty tile
/// 
/// Arguments:
/// 
/// * `index`: The index of the piece you want to print.
    fn print_piece_at(&self, index: usize) {
        match self.board[index] {
            Some(tile) => {
                match (tile.piecetype, tile.color) {
                    (PieceType::PAWN(_), WHITE) => { println!(" White Pawn ");}
                    (PieceType::ROOK(_), WHITE) => { println!(" White Rook ");}
                    (PieceType::KING(_), WHITE) => { println!(" White King ");}
                    (PieceType::KNIGHT, WHITE) => { println!(" White Knight ");}
                    (PieceType::QUEEN, WHITE) => { println!(" White Queen ");}
                    (PieceType::BISHOP, WHITE) => { println!(" White Bishop ");}
                    (PieceType::PAWN(_), BLACK) => { println!(" Black Pawn ");}
                    (PieceType::ROOK(_), BLACK) => { println!(" Black Rook ");}
                    (PieceType::KING(_), BLACK) => { println!(" Black King ");}
                    (PieceType::KNIGHT, BLACK) => { println!(" Black Knight ");}
                    (PieceType::QUEEN, BLACK) => { println!(" Black Queen ");}
                    (PieceType::BISHOP, BLACK) => { println!(" Black Bishop ");}
                    (_, _) => {eprintln!(" ? ");}
                }                   
            }
            None => {
                println!(" Empty Tile ");
            }            
        }
    }

/// > For each row, print the piece at each column, or a blank space if there is no piece
    pub fn print_me(&self){
        for i in (0..8).rev() {
            for j in 0..8 {
                let x = i * 8;
                match self.board[x+j] {
                    Some(tile) => {
                        match (tile.piecetype, tile.color) {
                            (PieceType::PAWN(_), WHITE) => { print!(" PW ");}
                            (PieceType::ROOK(_), WHITE) => { print!(" RW ");}
                            (PieceType::KING(_), WHITE) => { print!(" KW ");}
                            (PieceType::KNIGHT, WHITE) => { print!(" KnW ");}
                            (PieceType::QUEEN, WHITE) => { print!(" QW ");}
                            (PieceType::BISHOP, WHITE) => { print!(" BW ");}
                            (PieceType::PAWN(_), BLACK) => { print!(" PB ");}
                            (PieceType::ROOK(_), BLACK) => { print!(" RB ");}
                            (PieceType::KING(_), BLACK) => { print!(" KB ");}
                            (PieceType::KNIGHT, BLACK) => { print!(" KnB ");}
                            (PieceType::QUEEN, BLACK) => { print!(" QB ");}
                            (PieceType::BISHOP, BLACK) => { print!(" BB ");}
                            (_, _) => {eprint!(" ? ");}
                        }                   
                    }
                    None => {
                        print!("  _  ");
                    }
                }
            }
            println!();
        }
        println!();
    }

/// For each tile in the board, print the unicode character for the piece on that tile, or a space if
/// there is no piece
    pub fn print_unicode(&self){
        for i in (0..8).rev(){
            for j in 0..8{
                let x = i*8;
                match self.board[x+j]{
                    Some(tile) => {
                        match (tile.piecetype, tile.color){
                            (PieceType::PAWN(_), BLACK) => { print!(" ♙ ");}
                            (PieceType::ROOK(_), BLACK) => { print!(" ♖ ");}
                            (PieceType::KING(_), BLACK) => { print!(" ♔ ");}
                            (PieceType::KNIGHT, BLACK) => { print!(" ♘ ");}
                            (PieceType::QUEEN, BLACK) => { print!(" ♕ ");}
                            (PieceType::BISHOP, BLACK) => { print!(" ♗ ");}
                            (PieceType::PAWN(_), WHITE) => { print!(" ♟ ");}
                            (PieceType::ROOK(_), WHITE) => { print!(" ♜ ");}
                            (PieceType::KING(_), WHITE) => { print!(" ♚ ");}
                            (PieceType::KNIGHT, WHITE) => { print!(" ♞ ");}
                            (PieceType::QUEEN, WHITE) => { print!(" ♛ ");}
                            (PieceType::BISHOP, WHITE) => { print!(" ♝ ");}
                            (_, _) => {eprint!(" ? ");}
                        }
                    }
                    None => {
                        print!(" _ ");
                    }
                }
            }
            println!();
        }
    }

/// It prints the board
    pub fn print_board(&self){
        for i in (1..9).rev(){
            for j in 1..9 {
                let c = match j {
                    1 => 'A',
                    2 => 'B',
                    3 => 'C',
                    4 => 'D',
                    5 => 'E',
                    6 => 'F',
                    7 => 'G',
                    8 => 'H',
                    _ => 'X'
                };
                print!(" {}{} ", c, i);
            }
            println!();
        }
        println!();
    }

/// `coord_x` returns the x coordinate of a given coordinate
/// 
/// Arguments:
/// 
/// * `coord`: The coordinate to get the x value of.
/// 
/// Returns:
/// 
/// The x coordinate of the given coordinate.
    pub fn coord_x(coord: usize) -> usize {
        return coord % 8;
    }

/// It takes a coordinate and returns the y-coordinate of that coordinate
/// 
/// Arguments:
/// 
/// * `coord`: The coordinate of the piece.
/// 
/// Returns:
/// 
/// The y coordinate of the given coordinate.
    pub fn coord_y(coord: usize) -> usize {
        return coord/8;
    }

/// It takes two coordinates and returns the absolute difference between the x-coordinates
/// 
/// Arguments:
/// 
/// * `coord1`: The first coordinate.
/// * `coord2`: The coordinate of the piece you want to move.
/// 
/// Returns:
/// 
/// The absolute difference between the x coordinates of the two squares.
    pub fn abs_diff_x(coord1: usize, coord2: usize) -> usize {
        return ((coord1 as isize % 8) - (coord2 as isize % 8)).abs() as usize;
    }

/// It returns the absolute difference between the y coordinates of two squares
/// 
/// Arguments:
/// 
/// * `coord1`: The first coordinate
/// * `coord2`: The coordinate of the piece you want to move
/// 
/// Returns:
/// 
/// The absolute difference between the y coordinates of the two squares.
    pub fn abs_diff_y(coord1: usize, coord2: usize) -> usize {
        return ((coord1 as isize/8) - (coord2 as isize/8)).abs() as usize;
    }

/// `diff_x` returns the difference between the x coordinates of two chess board squares
/// 
/// Arguments:
/// 
/// * `coord1`: The first coordinate.
/// * `coord2`: The coordinate of the piece you want to move.
/// 
/// Returns:
/// 
/// The difference between the x coordinates of the two coordinates.
    pub fn diff_x(coord1: usize, coord2: usize) -> isize {
        return (coord2 as isize % 8) - (coord1 as isize % 8);
    }

/// It takes two coordinates and returns the difference between the y coordinates
/// 
/// Arguments:
/// 
/// * `coord1`: The first coordinate
/// * `coord2`: The coordinate of the piece you want to move
/// 
/// Returns:
/// 
/// The difference between the y coordinates of the two coordinates.
    pub fn diff_y(coord1: usize, coord2: usize) -> isize {
        return (coord2 as isize/8) - (coord1 as isize/8);
    }

    fn next_player(&mut self){
        self.current_player = match self.current_player {
            WHITE => BLACK,
            BLACK => WHITE,
            UNCOLORED => UNCOLORED
        };
    }

/// It takes a string of two characters, and returns a number between 0 and 63
/// 
/// Arguments:
/// 
/// * `coord`: &str - The coordinate to be translated.
/// 
/// Returns:
/// 
/// A usize
    pub fn coordinate_translator_str(coord: &str) -> usize {
        let mut input_tuple: (usize, usize) = (0, 0);
        for c in coord.chars() {
            match c {
                'A' => input_tuple.0 = 0,
                'B' => input_tuple.0 = 1,
                'C' => input_tuple.0 = 2,
                'D' => input_tuple.0 = 3,
                'E' => input_tuple.0 = 4,
                'F' => input_tuple.0 = 5,
                'G' => input_tuple.0 = 6,
                'H' => input_tuple.0 = 7,
                'a' => input_tuple.0 = 0,
                'b' => input_tuple.0 = 1,
                'c' => input_tuple.0 = 2,
                'd' => input_tuple.0 = 3,
                'e' => input_tuple.0 = 4,
                'f' => input_tuple.0 = 5,
                'g' => input_tuple.0 = 6,
                'h' => input_tuple.0 = 7,
                '1' => input_tuple.1 = 0,
                '2' => input_tuple.1 = 1,
                '3' => input_tuple.1 = 2,
                '4' => input_tuple.1 = 3,
                '5' => input_tuple.1 = 4,
                '6' => input_tuple.1 = 5,
                '7' => input_tuple.1 = 6,
                '8' => input_tuple.1 = 7,
                _ => {
                    println!("Entered: '{}'", coord);
                    panic!("Coordinates should match 'XY' where X is a letter A-H and Y is an integer 1-8")
                }
            }
        }
        return input_tuple.1*8 + input_tuple.0;
    }

/// It takes a usize, and returns a string
/// 
/// Arguments:
/// 
/// * `coord`: The coordinate to be translated.
/// 
/// Returns:
/// 
/// A string
    pub fn coordinate_translator_usize(coord: usize) -> String {
        if coord > 63 {
            panic!("Coord is out of bounds");
        }

        let c_x = GameState::coord_x(coord);
        let c_y = GameState::coord_y(coord);
        let mut return_tuple: (char, char) = (' ', ' ');

        match c_x {
            0 => return_tuple.0 = 'A',
            1 => return_tuple.0 = 'B',
            2 => return_tuple.0 = 'C',
            3 => return_tuple.0 = 'D',
            4 => return_tuple.0 = 'E',
            5 => return_tuple.0 = 'F',
            6 => return_tuple.0 = 'G',
            7 => return_tuple.0 = 'H',
            _ => panic!("huh?")
        }
        match c_y {
            0 => return_tuple.1 = '1',
            1 => return_tuple.1 = '2',
            2 => return_tuple.1 = '3',
            3 => return_tuple.1 = '4',
            4 => return_tuple.1 = '5',
            5 => return_tuple.1 = '6',
            6 => return_tuple.1 = '7',
            7 => return_tuple.1 = '8',
            _ => panic!("huh?")
        }

        let mut s = String::new();
        s.push(return_tuple.0);
        s.push(return_tuple.1);
        return s;
    }

/// It takes a string representing a chess coordinate, and returns a copy of the chess piece at that
/// coordinate
/// 
/// Arguments:
/// 
/// * `coord`: The coordinate of the piece you want to get.
/// 
/// Returns:
/// 
/// A clone of the ChessPiece at the given coordinate.
    pub fn get_piece_at(&self, coord: &str) -> Option<ChessPiece> {
        let index: usize = GameState::coordinate_translator_str(coord);
        return self.board[index].clone();
    }


/// > This function takes a piece and a target coordinate, and returns a boolean indicating whether or
/// not the move is valid
/// 
/// Arguments:
/// 
/// * `piece_coord`: The coordinate of the piece you want to move.
/// * `target_coord`: The coordinate of the target square.
/// 
/// Returns:
/// 
/// A boolean value.
    pub fn move_validity_checker(&mut self, piece_coord: &str, target_coord: &str) -> bool {
        self.debug_print("Move checker entered");

        let valid_move: bool;
        let pc = GameState::coordinate_translator_str(piece_coord);
        let tc = GameState::coordinate_translator_str(target_coord);
        self.debug_print("Piece: ");
        self.debug_print(pc);
        self.debug_print(GameState::coordinate_translator_usize(pc));
        self.debug_print("Target: ");
        self.debug_print(tc);
        self.debug_print(GameState::coordinate_translator_usize(tc));

        if let Some(piece) = self.get_piece_at(piece_coord) {
            //piece.print_piece();
            match piece.piecetype {
                PieceType::PAWN(_) => {
                    valid_move = self.pawn_move_checker(piece, pc, tc);
                },
                PieceType::KING(_) =>  {
                    valid_move = self.king_move_checker(piece, pc, tc);
                },
                PieceType::KNIGHT => {
                    valid_move = self.knight_move_checker(piece, pc, tc);
                },
                PieceType::BISHOP => {
                    valid_move = self.bishop_move_checker(piece, pc, tc);
                },
                PieceType::ROOK(_) => {
                    valid_move = self.rook_move_checker(piece, pc, tc);
                },
                PieceType::QUEEN => {
                    valid_move = self.queen_move_checker(piece, pc, tc);
                },
                _ => panic!("TODO!")
            }
        } else {
            valid_move = false;
        }

        return valid_move;
    }

/// If the current coordinate is on the left side of the board and the target coordinate is on the right
/// side of the board, or vice versa, then the move is invalid
/// 
/// Arguments:
/// 
/// * `current_coord_numeric`: The current coordinate in numeric form.
/// * `target_coord_numeric`: The numeric value of the target coordinate.
/// 
/// Returns:
/// 
/// A boolean value.
    fn out_of_bounds_checker(&self, current_coord_numeric: usize, target_coord_numeric: usize) -> bool {
        if current_coord_numeric > 63 {
            self.debug_print("Out of bounds Y val");
            return false;
        }
        if (current_coord_numeric % 8 == 0) && (target_coord_numeric % 7 == 0){
            if target_coord_numeric == 0 {
                return true; //Valid
            }
            self.debug_print("out of bounds 1");
            return false; //Invalid crossing A -> H
        } else if (current_coord_numeric % 7 == 0) && (target_coord_numeric % 8 == 0) {
            if current_coord_numeric == 0 {
                return true;
            }
            self.debug_print("out of bounds 2");
            return false;
        }
        true
    }

/// > If the piece is a pawn, and the piece is moving forward, and the piece is moving one or two steps
/// forward, and the piece is moving to an empty space, then the move is valid
/// 
/// Arguments:
/// 
/// * `piece`: The piece that is being moved
/// * `piece_coord`: The coordinate of the piece you want to move
/// * `target_coord`: The coordinate of the target square
/// 
/// Returns:
/// 
/// A boolean value that is true if the move is valid and false if the move is invalid.
    fn pawn_move_checker(&self, piece: ChessPiece, pc: usize, tc: usize) -> bool {
        let bool_tuple: (bool, bool) = match (piece.piecetype, piece.color) {
            (PieceType::PAWN(false), WHITE) => (true, true),
            (PieceType::PAWN(false), BLACK) => (false, true),
            (PieceType::PAWN(true), WHITE) => (true, false),
            (PieceType::PAWN(true), BLACK) => (false, false),
            (_, _) => panic!("Pawn move Checker failed to read piecetype/color")
        };
        match self.current_player {
            WHITE => if bool_tuple.0 {/* continue */},
            BLACK => if bool_tuple.0 {/* Can't move other players pieces */ return false;},
            UNCOLORED => {/* continue */}
        }


        let up_or_down: i8;
        if bool_tuple.0 {
            up_or_down = 1;
        } else {
            up_or_down = -1;
        }

        let mut returner: bool = false;

        //Step forward
        if pc as i8 + 8*up_or_down == tc as i8 {

            //Check that spot is empty
            match self.board[tc] {
                Some(_) => {return false;}, //Target Coord is not empty, Invalid Move
                None    => {returner = true;} //Target Coord is empty, Valid Move
            }
        } 
        //Step 2 steps forward
        else if pc as i8 + 16*up_or_down == tc as i8 {

            //Check that pawn hasn't moved
            if !bool_tuple.1 {
                return false;
            }

            //Try to move 1 step forward
            match self.board[(pc as i8 + 8*up_or_down as i8) as usize] {
                Some(_) => {return false;}, //Invalid Move, Something is in the way
                None    => {/*Do nothing*/}
            };

            //Move another step forward
            match self.board[tc] {
                Some(_) => {return false;}, //Invalid something is 
                None    => {returner = true;} //Valid move
            }
        }
        //Diagonal step (attack move)
        else if (pc as i8 + 7*up_or_down == tc as i8) || (pc as i8 + 9*up_or_down == tc as i8) {
            //Check that it is in fact a diagonal move and not A <-> H move
            if self.out_of_bounds_checker(pc, tc) {
                //Valid move, did not cross A <-> H
                self.debug_print("Pawn move checker: Valid move, did not cross A <-> H");
                match self.board[tc] {
                    Some(target_piece) => {
                        self.debug_print("Pawn move checker: Target piece is not empty");
                        let target_color:bool = target_piece.get_color_as_bool();
                        returner = bool_tuple.0 != target_color;
                        if returner {
                            self.debug_print("Pawn move checker: Valid move, did not cross A <-> H, target piece is not same color");
                        } else {
                            self.debug_print("Pawn move checker: Invalid move, did not cross A <-> H, target piece is same color");
                        }
                    },
                    None => return false
                }
            } else {
                //Invalid Move, crossed A <-> H
                return false;
            }
        }
        
        if GameState::abs_diff_x(pc, tc) > 1 {
            return false;
        }

        if GameState::abs_diff_y(pc, tc) > 2 {
            return false;
        }

        return returner;
    }


/// If the target is one square away, check if it's empty or if it's an enemy. If the target is two
/// squares away, check if the square in between is empty and if the target is empty or an enemy
/// 
/// Arguments:
/// 
/// * `piece`: The piece that is moving
/// * `piece_coord`: The coordinate of the piece that is moving
/// * `target_coord`: The coordinate of the target square
/// 
/// Returns:
/// 
/// A boolean value that is true if the move is valid, false if not.
    fn king_move_checker(&mut self, piece: ChessPiece, pc: usize, tc: usize) -> bool {
        self.debug_print("king_checker entered");

        match piece.piecetype {
            PieceType::KING(_) => {/* continue */},
            _ =>  panic!("Not a king!")
        };

        let color_bool: bool = piece.get_color_as_bool();
        let mut returner: bool = false;


        if pc + 8 == tc || pc + 1 == tc || pc as isize -1 == tc as isize || pc as isize - 8 == tc as isize {
            self.debug_print("king_check first if");
            //Check that spot is empty, else try to attack
            if self.out_of_bounds_checker(pc, tc) {
                self.debug_print("king_check not out of bounds");
                match self.board[tc] {
                    Some(other_piece) => { 
                        self.debug_print("king_check 1");
                        returner = color_bool != other_piece.get_color_as_bool(); 
                    },
                    None => { 
                        self.debug_print("king_check 2");
                        returner = true; 
                    }
                }
            }
        } else if pc + 7 == tc || pc + 9 == tc || pc as isize - 7 == tc as isize || pc as isize - 9 == tc as isize {
            self.debug_print("king_check elif");
            let up_down: i8 = match tc as i8 - pc as i8 {
                7 => 8,
                9 => 8,
                -7 => -8,
                -9 => -8,
                _ => panic!("huh?")
            };
            if self.out_of_bounds_checker(((pc as i8) + up_down) as usize, tc) {
                match self.board[tc] {
                    Some(other_piece) => {

                        self.debug_print("king_check 3");
                        returner = color_bool != other_piece.get_color_as_bool(); 
                    },
                    None => { 

                        self.debug_print("king_check 4");
                        returner = true; 
                    }
                }
            }
        } else if pc + 2 == tc || pc as isize - 2 == tc as isize {
            //Castling check
            self.debug_print("Castling check!");
            return self.castling_check(pc, tc);
        }

        if returner {
            let diff_x = (GameState::coord_x(pc) as isize - GameState::coord_x(tc) as isize).abs();
            let diff_y = (GameState::coord_y(pc) as isize - GameState::coord_y(tc) as isize).abs();
            if diff_x > 1 || diff_y > 1 {
                self.debug_print("King moved too much");
                returner = false;
            }
        }

        self.debug_print("king_checker left");


        return returner;
    }



/// If the target coordinate is one of the 8 possible knight moves, and the target coordinate is either
/// empty or occupied by an enemy piece, then return true
/// 
/// Arguments:
/// 
/// * `piece`: The piece that is moving
/// * `piece_coord`: The coordinate of the piece you're moving
/// * `target_coord`: The coordinate of the target square
/// 
/// Returns:
/// 
/// A boolean value that is true if the move is valid and false if it is not.
    fn knight_move_checker(&self, piece: ChessPiece, pc: usize, tc: usize) -> bool {
        self.debug_print("Knight Checker entered");


        match piece.piecetype {
            PieceType::KNIGHT => {/*continue*/},
            _ => panic!("Not a Knight")
        };

        let color_bool = piece.get_color_as_bool();
        let mut returner;

        let diff_x = (GameState::coord_x(pc) as isize - GameState::coord_x(tc) as isize).abs();
        let diff_y = (GameState::coord_y(pc) as isize - GameState::coord_y(tc) as isize).abs();
        if diff_x > 1 && diff_y > 1 {
            self.debug_print("Knight moved too much (x and y)");
            return false;
        }
        if diff_x < 1 && diff_y < 1 {
            self.debug_print("Knight moved too little");
            return false;
        }
        if diff_x > 2 || diff_y > 2 {
            self.debug_print("Knight moved too much (x or y)");
            return false;
        }
        if diff_x == 0 || diff_y == 0 {
            self.debug_print("Knight didnt move x or y");
            return false;
        } if diff_x  == 1 && diff_y == 1 {
            self.debug_print("Knight moved 1 sted diagonally");
            return false;
        }

        // 2 steps up/down 1 to the side
        if pc + 17 == tc || pc + 15 == tc {
            if self.out_of_bounds_checker(pc + 16, tc) {
                //continue
                self.debug_print("Knight 2 steps forward valid bounds");
            } else {
                self.debug_print("Knight 2 steps forward Out of bounds");
                return false;
            }
        } else if (pc as isize - 17) == tc as isize || (pc as isize - 15) == tc as isize {
            if self.out_of_bounds_checker(pc - 16, tc) {
                //continue
                self.debug_print("Knight 2 steps back valid bounds");
            } else {
                self.debug_print("Knight 2 steps back out of bounds");
                return false;
            }
        } else if pc + 10 == tc{
            //1 step up
            let cord: usize = pc + 8;
            // 1 step right
            if self.out_of_bounds_checker(cord +1, tc){
                self.debug_print("Knight 1 step up 1 step right success");
                //another step to the right
                if self.out_of_bounds_checker(cord + 2, tc) {
                    self.debug_print("Knight 1 step up 2 steps right success");
                    //continue
                } else {
                    self.debug_print("Knight 1 step up 2 steps right out of bounds");
                    return false;
                }
            } else {
                self.debug_print("Knight 1 step up 1 step right out of bounds");
                return false;
            }
        } else if pc + 6 == tc{
            //1 step up
            let cord: usize = pc +8;

            // 1 step left
            if self.out_of_bounds_checker(cord-1, tc){
                self.debug_print("Knight 1 step up 1 step left success");
                //another step to the left
                if self.out_of_bounds_checker(cord-2, tc){
                    self.debug_print("Knight 1 step up 2 steps left success");
                    //continue
                } else {
                    self.debug_print("Knight 1 step up 2 steps left out of bounds");
                    return false;
                }
            } else {
                self.debug_print("Knight 1 step up 1 step left out of bounds");
                return false;
            }
        } else if pc as isize - 10 == tc as isize {
            //1 step down
            let cord: usize = pc-8;

            // 1 step left
            if self.out_of_bounds_checker(cord-1, tc){
                self.debug_print("Knight 1 step down 1 step left success");
                //another step to the left
                if self.out_of_bounds_checker(cord-2, tc){
                    self.debug_print("Knight 1 step down 2 steps left success");
                    //continue
                } else {
                    self.debug_print("Knight 1 step down 2 steps left out of bounds");
                    return false;
                }
            } else {
                self.debug_print("Knight 1 step down 1 step left out of bounds");
                return false;
            }
        } else if pc as isize - 6 == tc as isize {
            //1 step down
            let cord: usize = pc-8;

            // 1 step right
            if self.out_of_bounds_checker(cord+1, tc){
                self.debug_print("Knight 1 step down 1 step right success");
                //another step to the right
                if self.out_of_bounds_checker(cord+2, tc){
                    self.debug_print("Knight 1 step down 2 steps right success");
                    //continue
                } else {
                    self.debug_print("Knight 1 step down 2 steps right out of bounds");
                    return false;
                }
            } else {
                self.debug_print("Knight 1 step down 1 step right out of bounds");
                return false;
            }
        }

        match self.board[tc] {
            Some(other_piece) => {
                self.debug_print("Knight move found piece");
                returner = color_bool != other_piece.get_color_as_bool();
            },
            None =>{
                self.debug_print("Knight move did not find piece");
                returner = true;
            }
        }



        self.debug_print("Knight Checker left");


        

        return returner;
    }


/// If the piece is a rook or queen, and the move is either horizontal or vertical, and there are no
/// pieces in the way, and the target piece is not the same color, then the move is valid
/// 
/// Arguments:
/// 
/// * `piece`: The piece that is moving
/// * `pc`: The position of the piece
/// * `tc`: Target Coordinate
/// 
/// Returns:
/// 
/// A boolean value that is true if the move is valid, false if it is not.
    fn rook_move_checker(&self, piece: ChessPiece, pc: usize, tc: usize) -> bool {
        self.debug_print("Rook Move Checker Entered");

        let is_queen: bool = piece.piecetype == PieceType::QUEEN;

        if !is_queen {
            let has_moved: bool = match piece.piecetype {
                PieceType::ROOK(true) => true,
                PieceType::ROOK(false) => false,
                _ => panic!("Something went wrong (Not a Rook?)")
            };
        }

        let color_bool = piece.get_color_as_bool();

        let mut returner: bool;

        let diff_x = GameState::diff_x(pc, tc);
        let diff_y = GameState::diff_y(pc, tc);

        let abs_diff_x = GameState::abs_diff_x(pc, tc);
        let abs_diff_y = GameState::abs_diff_y(pc, tc);

        self.debug_print(diff_x);
        self.debug_print(diff_y);
        self.debug_print(abs_diff_x);
        self.debug_print(abs_diff_y);

        if abs_diff_x > 0 && abs_diff_y > 0 {
            self.debug_print("Rook moved diagonally");
            return false;
        }

        if abs_diff_x > 0 {
            self.debug_print("Rook moved horizontally");
            for i in 1..diff_x {
                if self.board[(pc as isize + i) as usize].is_some() {
                    self.debug_print("Rook move found piece");
                    return false;
                }
            }
        } else if abs_diff_y > 0 {
            self.debug_print("Rook moved vertically");
            if diff_y < 0 {
                for i in diff_y+1..0 {
                    self.debug_print("Rook move checking i:");
                    self.debug_print(i);
                    self.debug_print((format!("{}", (pc as isize + i*8) as usize)).as_str());
                    if self.board[((pc as isize) + i*8) as usize].is_some() {
                        self.debug_print("Rook move found piece");
                        return false;
                    }
                }
            } else {
                for i in 1..diff_y {
                    self.debug_print("Rook move checking i:");
                    self.debug_print(i);
                    self.debug_print((format!("{}", (pc as isize + i*8) as usize)).as_str());
                    if self.board[((pc as isize) + i*8) as usize].is_some() {
                        self.debug_print("Rook move found piece");
                        return false;
                    }
                }
            }

        }

        self.debug_print("Matching target piece");
        match self.board[tc] {
            Some(other_piece) => {
                self.debug_print("Rook move found piece");
                returner = color_bool != other_piece.get_color_as_bool();
                if !returner {
                    self.debug_print("Rook move found piece of same color");
                    return false;
                }
            },
            None =>{
                self.debug_print("Rook move did not find piece");
                returner = true;
            }
        }


        self.debug_print("Rook Checker left");

        return returner;
    }

/// If the bishop is moving diagonally, and there are no pieces in the way, and the target square is
/// either empty or has a piece of the opposite color, then the move is valid
/// 
/// Arguments:
/// 
/// * `piece`: The piece that is moving
/// * `pc`: The position of the piece
/// * `tc`: Target Cell
/// 
/// Returns:
/// 
/// A boolean value that is true if the move is valid and false if it is not.
    fn bishop_move_checker(&self, piece: ChessPiece, pc: usize, tc: usize) -> bool {

        self.debug_print("Bishop Move Checker Entered");

        let color_bool = piece.get_color_as_bool();
        let mut returner: bool = false;

        let diff_x = GameState::diff_x(pc, tc);
        let diff_y = GameState::diff_y(pc, tc);
        let abs_diff_x = GameState::abs_diff_x(pc, tc);
        let abs_diff_y = GameState::abs_diff_y(pc, tc);
        
        if abs_diff_x < 1 || abs_diff_y < 1 {
            self.debug_print("Bishop moved too little");
            return false;
        }

        if abs_diff_x != abs_diff_y {
            self.debug_print("Bishop did not move diagonally");
            return false;
        }
        
        for i in 1..abs_diff_x {
            let mut p: isize = 1;
            let mut p1: isize = 1;
            if diff_y < 0 {
                p = -1;
            }
            if diff_x < 0 {
                p1 = -1;
            }
            self.debug_print("target while checking for pieces: ");
            self.debug_print(self.board[(pc as isize + (i as isize * p1) + (i as isize *p *8)) as usize]);
            if self.board[(pc as isize + (i as isize * p1) + (i as isize * p * 8)) as usize].is_some() {
                self.debug_print("Bishop move found piece");
                return false;
            }
        }
        
        // Matching target piece
        match self.board[tc] {
            Some(other_piece) => {
                self.debug_print("Bishop move found piece");
                returner = color_bool != other_piece.get_color_as_bool();
                if !returner {
                    self.debug_print("Bishop move found piece of same color");
                    return false;
                }
            },
            None =>{
                self.debug_print("Bishop move did not find piece");
                returner = true;
            }
        }


        self.debug_print("Bishop checker left");
        return returner;
    }
    
/// If the move is a bishop or rook move, return true
/// 
/// Arguments:
/// 
/// * `piece`: The piece that is being moved
/// * `pc`: The current position of the piece
/// * `tc`: Target Column
/// 
/// Returns:
/// 
/// A boolean value that is true if the move is valid and false if it is not.
    fn queen_move_checker(&self, piece: ChessPiece, pc: usize, tc: usize) -> bool {
        self.debug_print("Queen Move Checker Entered");

        let is_queen: bool = piece.piecetype == PieceType::QUEEN;
        if !is_queen {
            panic!("Something went wrong (Not a Queen?)");
        }

        let color_bool = piece.get_color_as_bool();
        let mut returner: bool = false;

        //Check if the move is that of a bishop or rook
        if self.bishop_move_checker(piece, pc, tc) || self.rook_move_checker(piece, pc, tc) {
            self.debug_print("Queen move is a bishop or rook move");
            returner = true;
        }


        return returner;
    }


/// It takes a piece coordinate as an argument, and returns a vector of all the valid moves that piece
/// can make
/// 
/// Arguments:
/// 
/// * `pc`: The piece's current location.
    pub fn get_moves_from_tile(&mut self, pc: usize) -> Vec<usize> {
        let pc_string = GameState::coordinate_translator_usize(pc);
        let pcs = pc_string.as_str();
        let mut v: Vec<usize> = Vec::new();
        if let Some(_piece) = self.get_piece_at(pcs) {
            for tc in 0..64 {
                let tc_string = GameState::coordinate_translator_usize(tc);
                let tcs = tc_string.as_str();
                if self.move_validity_checker(pcs, tcs) {
                    v.push(tc);
                }
            }
        }
        // println!("Valid moves from {}", pcs);
        // for elem in v {
        //     println!("{}", GameState::coordinate_translator_usize(elem));
        // }
        v
    }

/// It takes a tile number and returns a vector of all the pieces that can move to that tile
/// 
/// Arguments:
/// 
/// * `tile`: The tile you want to move to.
/// 
/// Returns:
/// 
/// A vector of all the possible moves that can be made to a certain tile.
    pub fn get_moves_to_tile(&mut self, tile: usize) -> Vec<usize> {
        let tile_string = GameState::coordinate_translator_usize(tile);
        let tile_str = tile_string.as_str();
        let mut v: Vec<usize> = Vec::new();
        for pc in 0..64 {
            let pc_string = GameState::coordinate_translator_usize(pc);
            let pcs = pc_string.as_str();
            if self.move_validity_checker(pcs, tile_str) {
                v.push(pc);
            }
        }
        v
    }

/// It takes a tile and a color, and returns a vector of all the pieces of that color that can move to
/// that tile
/// 
/// Arguments:
/// 
/// * `tile`: The tile you want to check if a piece can move to.
/// * `color`: Color - The color of the piece you want to move.
/// 
/// Returns:
/// 
/// A vector of usize
    pub fn get_color_moves_to_tile(&mut self, tile: usize, color: Color) -> Vec<usize> {
        let tile_string = GameState::coordinate_translator_usize(tile);
        let tile_str = tile_string.as_str();
        let mut v: Vec<usize> = Vec::new();
        for pc in 0..64 {
            if let Some(piece) = self.board[pc] {
                if piece.get_color_as_bool() == color.match_color_as_bool() {
                    let pc_string = GameState::coordinate_translator_usize(pc);
                    let pcs = pc_string.as_str();
                    if self.move_validity_checker(pcs, tile_str) {
                        v.push(pc);
                    }
                }
            }
        }
        v
    }

/// It takes a usize, converts it to a string, checks if there's a piece at that location, and if there
/// is, it checks if the piece can move to every other tile on the board, and if it can, it adds the
/// tile to a vector of strings
/// 
/// Arguments:
/// 
/// * `pc`: The piece's current position.
/// 
/// Returns:
/// 
/// A string of all the possible moves from a given tile.
    pub fn get_moves_from_tile_as_string(&mut self, pc: usize) -> String {
        let pc_string = GameState::coordinate_translator_usize(pc);
        let pcs = pc_string.as_str();
        let mut v: Vec<String> = Vec::new();
        if let Some(_piece) = self.get_piece_at(pcs) {
            for tc in 0..64 {
                let tc_string = GameState::coordinate_translator_usize(tc);
                let tcs = tc_string.as_str();
                if self.move_validity_checker(pcs, tcs) {
                    v.push(tc_string.to_string());
                }
            }
        }
        let mut returner = String::new();
        for elem in v {
            returner.push_str(&elem);
            returner.push_str(", ");
        }
        return returner;        
    }


/// If the move is valid, make the move.
/// Check if the player is checked, 
///     if they are, revert the move,
///     if they aren't, change the player
/// 
/// Arguments:
/// 
/// * `pc`: The piece's current coordinate
/// * `tc`: Target Coordinate
    pub fn do_valid_move(&mut self, pc: &str, tc: &str) {
        self.debug_print("Do Valid Move Entered");
        self.debug_print("pc: ");
        self.debug_print(pc);
        self.debug_print("tc: ");
        self.debug_print(tc);
        let pc_usize = GameState::coordinate_translator_str(pc);
        let tc_usize = GameState::coordinate_translator_str(tc);
        if let Some(piece) = self.board[pc_usize] {
            let mut was_checked = false;
            if self.current_player == self.checked_player && self.checked_flag == true {
                was_checked = true;
            }
            if pc_usize == self.wkc {
                self.debug_print("Piece Coord is White King Coord");
                self.debug_print(pc_usize);
                self.debug_print(self.wkc);
                self.old_wkc = self.wkc;
                self.wkc = tc_usize;
            } else if pc_usize == self.bkc {
                self.debug_print("Piece Coord is Black King Coord");
                self.debug_print(pc_usize);
                self.debug_print(self.bkc);
                self.old_bkc = self.bkc;
                self.bkc = tc_usize;
            }
            self.prev_board = self.board;
            self.board[tc_usize] = Some(piece);
            self.board[pc_usize] = None;

            //TODO: En Passant function
            //TODO: Checkmate function
            //TODO: Stalemate function
            self.debug_print("before checked checker: ");
            self.checked_checker();
            self.debug_print("after checked checker: ");

            if was_checked && self.checked_player == self.current_player {
                self.board = self.prev_board;
                self.wkc = self.old_wkc;
                self.bkc = self.old_bkc;
                self.debug_print("Reverting!!! Was checked, is still checked");
                if self.castling_flag {
                    self.debug_print("Invalid castling move????");
                    self.castling_flag = false;
                }
            } else {
                self.debug_print("Next player!");
                self.next_player();
            }
            match piece.piecetype {
                PieceType::KING(false) => {
                    self.debug_print("Setting King to moved = true");
                    self.board[tc_usize] = Some(ChessPiece::new(PieceType::KING(true), piece.color));
                    self.debug_print(self.board[tc_usize]);
                },
                PieceType::PAWN(false) => {
                    self.debug_print("Setting Pawn to moved = true");
                    self.board[tc_usize] = Some(ChessPiece::new(PieceType::PAWN(true), piece.color));
                    self.debug_print(self.board[tc_usize]);
                },
                PieceType::ROOK(false) => {
                    self.debug_print("Setting Rook to moved = true");
                    self.board[tc_usize] = Some(ChessPiece::new(PieceType::ROOK(true), piece.color));
                    self.debug_print(self.board[tc_usize]);
                },
                _ => {
                    //Do nothing
                }
            }
            if self.castling_flag {
                if pc_usize + 2 == tc_usize {
                    if let Some( mut rook) = self.board[pc_usize + 3] {
                        rook.piecetype = PieceType::ROOK(true);
                        self.board[pc_usize + 3] = None;
                        self.board[pc_usize + 1] = Some(rook);
                    } else {
                        panic!("This move should have been valid!");
                    }
                } else if pc_usize - 2 == tc_usize {
                    if let Some(mut rook) = self.board[pc_usize - 4] {
                        rook.piecetype = PieceType::ROOK(true);
                        self.board[pc_usize - 4] = None;
                        self.board[pc_usize - 1] = Some(rook);
                    } else {
                        panic!("This move should have been valid!");
                    }
                }
                self.castling_flag = false;
            }
        } else {
            panic!("This should have been a valid move!");
        }
    }

/// For each piece on the board, check if the piece's moveset contains the king's position. If it does,
/// set the checked flag to true and break
pub fn checked_checker(&mut self) {
    self.debug_print("Checked checker entered...");

    for i in 0..64 {
        let piece = match self.board[i] {
            None => continue,
            Some(piece) => piece
        };
        let piece_color = piece.get_color_as_bool();
        let moveset = self.get_moves_from_tile(i);

        if piece_color {
            if moveset.contains(&self.bkc) {
                self.debug_print("Piece that is checking: ");
                self.debug_print(piece);
                self.debug_print("Checking BLACK");
                self.debug_print("bkc: ");
                self.debug_print(self.bkc);
                self.debug_print(GameState::coordinate_translator_usize(self.bkc));
                self.debug_print("moveset: ");
                self.debug_print(moveset);


                self.checked_flag = true;
                self.checked_player = BLACK;
                println!("Player {:?} has been checked", BLACK);
                self.debug_print("Black has been checked");
                break;
            }   
        } else {
            if moveset.contains(&self.wkc) {
                self.debug_print("Piece that is checking: ");
                self.debug_print(piece);
                self.debug_print("Checking WHITE");
                self.debug_print("wkc: ");
                self.debug_print(self.wkc);
                self.debug_print(GameState::coordinate_translator_usize(self.wkc));
                self.debug_print("moveset: ");
                self.debug_print(moveset);

                self.checked_flag = true;
                self.checked_player = WHITE;
                println!("Player {:?} has been checked", WHITE);
                self.debug_print("WHITE has been checked");
                break;
            }      
        }
        self.checked_flag = false;
        self.checked_player = UNCOLORED;
    }
    self.debug_print("Checked checker left...");
}

/// If the king is not checked, and the king has not moved, and the king is not moving to a tile that is
/// checked, and the king is not moving to a tile that is checked along the way, and the king is moving
/// to the correct tile, and the rook is of the correct color and has not moved, then the move is valid
/// 
/// Arguments:
/// 
/// * `pc`: The position of the piece
/// * `tc`: Target coordinate
/// 
/// Returns:
/// 
/// A bool that is true if the move is valid and false if it is not.
pub fn castling_check(&mut self, pc: usize, tc: usize) -> bool {
    self.debug_print("Castling check entered...");
    if let Some(piece) = self.board[pc] {
        let piece_color = piece.get_color_as_bool();
        if self.checked_flag {
            if self.checked_player.match_color_as_bool() == piece_color {
                self.debug_print("Cannot castle when checked");
                return false;            
            }
        }
        match (piece.piecetype, piece_color) {
            (PieceType::KING(true), _) => {
                self.debug_print("King has already castled");
                return false;
            },
            (PieceType::KING(false), _) => {
                //Do nothing
            },
            _ => {
                panic!("This aint no king!");
            }
        }
        if tc == pc + 2 {
            self.debug_print("Castling to the right");
            if let Some(rook) = self.board[tc +1] {
                match (rook.piecetype, rook.color) {
                    (PieceType::ROOK(false), piece_color) => {
                        //Seems correct!
                        self.debug_print("Piece is a rook of correct color and hasnt moved");
                    },
                    _ => {
                        self.debug_print("Piece is not a rook of correct color that hasnt mvoed");
                        return false;
                    }
                }
                for i in 1..3 {
                    match self.board[pc + i] {
                        Some(piece) => {
                            self.debug_print("There seems to be a piece in the way at: ");
                            self.debug_print(pc + i);
                            self.debug_print(piece);
                            return false;
                        }
                        None => {
                            //Do nothing
                        }
                    }
                    if !self.get_color_moves_to_tile(pc + i, Color::match_bool_as_color(!piece_color)).is_empty() {
                        self.debug_print("Move would result in being checked along the way!");
                        return false;
                    }
                }
            } else {
                self.debug_print("There was no rook to the right!");
                return false;
            }
        } else if tc == pc - 2 {
            self.debug_print("Castling to the left");
            self.debug_print("Tc: ");
            self.debug_print(tc);
            self.debug_print(tc as isize - 2);
            if tc as isize - 2 < 0 {
                self.debug_print("Tc is less than 0");
                return false;
            }
            if let Some(rook) = self.board[(tc as isize - 2) as usize] {
                match (rook.piecetype, rook.get_color_as_bool()) {
                    (PieceType::ROOK(false), piece_color) => {
                        //Seems correct!
                        self.debug_print("Piece is a rook of correct color and hasnt moved");
                    },
                    _ => {
                        self.debug_print("Piece is not a rook of correct color that hasnt mvoed");
                        return false;
                    }
                }
                for i in 1..3 {
                    match self.board[pc - i] {
                        Some(piece) => {
                            self.debug_print("There seems to be a piece in the way at: ");
                            self.debug_print(pc - i);
                            return false;
                        }
                        None => {
                            //Do nothing
                        }
                    }
                    if !self.get_color_moves_to_tile(pc - i, Color::match_bool_as_color(!piece_color)).is_empty() {
                        self.debug_print("Move would result in being checked along the way!");
                        return false;
                    }
                }
            } else {
                self.debug_print("There was no rook to the left!");
                return false;
            }
        } else {
            self.debug_print("Castling to the wrong side!");
            return false;
        }
        self.castling_flag = true;
        self.debug_print("Castling check left...");
        return true;
    }
    self.debug_print("Castling check left...");
    return false;
}

/// "Return the index of the white king, or 64 if there is no white king."
/// 
/// The function starts by looping over all 64 squares on the board. For each square, it checks if there
/// is a piece on that square. If there is no piece, it continues to the next square. If there is a
/// piece, it checks if that piece is a white king. If it is, it returns the index of that square. If it
/// is not, it continues to the next square. If it reaches the end of the loop, it returns 64
/// 
/// Returns:
/// 
/// The index of the white king.
pub fn find_white_king(&self) -> usize {
    for i in 0..64 {
        match self.board[i] {
            None => continue,
            Some(piece) => match (piece.piecetype, piece.color) {
                (PieceType::KING(_), WHITE) => return i,
                _ => continue
            }
        };
    }
    64
}

/// "Return the index of the black king, or 64 if there is no black king."
/// 
/// The function starts by looping over all 64 squares on the board. For each square, it checks if there
/// is a piece on that square. If there is no piece, it continues to the next square. If there is a
/// piece, it checks if that piece is a black king. If it is, it returns the index of that square. If it
/// is not, it continues to the next square. If it gets to the end of the loop without finding a black
/// king, it returns 64
/// 
/// Returns:
/// 
/// The index of the black king.
pub fn find_black_king(&self) -> usize {
    for i in 0..64 {
        match self.board[i] {
            None => continue,
            Some(piece) => match (piece.piecetype, piece.color) {
                (PieceType::KING(_), BLACK) => return i,
                _ => continue
            }
        };
    }
    64
}

    //Debug Functions:

/// > This function takes a coordinate, a color, and a piece type, and places a piece of that color and
/// type on the board at that coordinate
/// 
/// Arguments:
/// 
/// * `coord_numeric`: The coordinate of the piece you want to place.
/// * `color_string`: "Black" or "White"
/// * `piece_string`: The type of piece you want to place.
    pub fn place_piece(&mut self, coord_numeric: usize, color_string: &str, piece_string: &str){
        let color: Color = match color_string {
            "Black" =>  BLACK,
            "black" => BLACK,
            "BLACK" => BLACK,
            "white" => WHITE,
            "White" => WHITE,
            "WHITE" => WHITE,
            _ => panic!("color should be written as 'Black' or 'White'")
        };

        let piecetype: PieceType = match piece_string {
            "Pawn" => PieceType::PAWN(false),
            "Rook" => PieceType::ROOK(false),
            "King" =>  PieceType::KING(false),
            "Bishop" =>  PieceType::BISHOP,
            "Knight" => PieceType::KNIGHT,
            "Queen" => PieceType::QUEEN,
            _ => panic!("Invalid Piece: '{}'", piece_string)
        };

        let piece = ChessPiece::new(piecetype, color);
        self.board[coord_numeric] = Some(piece);
    }

/// `debug_print` takes a string and prints it to the console
/// 
/// Arguments:
/// 
/// * `arg`: &str - This is the parameter that we're passing into the function. It's a string slice.
    pub fn debug_print<T: std::fmt::Debug>(&self, arg: T){
        if self.debug_flag {
            println!("Debug print: '{:?}'", arg);
        }
    }

}
