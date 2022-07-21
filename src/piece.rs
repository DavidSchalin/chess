use serde::{Serialize, Deserialize};
/// Creating an enum with the name PieceType.
#[derive(PartialEq, Eq, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum PieceType {
    PAWN(bool),
    BISHOP,
    ROOK(bool),
    KNIGHT,
    QUEEN,
    KING(bool),
    NONE
}

/// Creating an enum with the name Color.
#[derive(PartialEq, Eq, Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Color {
    WHITE,
    BLACK,
    UNCOLORED
}

impl Color {
    // Create a function that matches the color to a bool.
    pub fn match_color_as_bool(&self) -> bool {
        match self {
            Color::WHITE => true,
            Color::BLACK => false,
            Color::UNCOLORED => {
                panic!("Color is not colored!");
            }
        }
    }

    pub fn match_bool_as_color(b: bool) -> Color {
        match b {
            true => Color::WHITE,
            false => Color::BLACK,
            _ => {
                panic!("Color is not colored!");
            }
        }
    }
}

/// A ChessPiece is a struct that contains a PieceType and a Color.
/// 
/// Properties:
/// 
/// * `piecetype`: The type of piece.
/// * `color`: The color of the piece.
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct ChessPiece {
    pub piecetype: PieceType,
    pub color: Color,
}

const WHITE: Color = Color::WHITE;
const BLACK: Color = Color::BLACK;
const UNCOLORED: Color = Color::UNCOLORED;

impl ChessPiece {

/// "This function creates a new ChessPiece object with the given type and color."
/// 
/// The first line of the function is the function signature. It tells us the name of the function, the
/// type of the arguments, and the return type
/// 
/// Arguments:
/// 
/// * `new_type`: The type of piece you want to create.
/// * `new_color`: Color - The color of the piece.
/// 
/// Returns:
/// 
/// A new ChessPiece object with the given type and color.
    pub fn new(new_type: PieceType, new_color: Color) -> ChessPiece {
        ChessPiece{
            piecetype: new_type,
            color: new_color
        }
    }


/// > This function returns a boolean value that represents the color of the piece
/// 
/// Returns:
/// 
/// A boolean value.
    pub fn get_color_as_bool(&self) -> bool {
        match self.color {
            WHITE => {return true;},
            BLACK => {return false;},
            UNCOLORED => panic!("Uncolored Piece")
        }
    }


/// It prints the piece type and color to the console
    pub fn print_piece(&self) {
        match (self.piecetype, self.color) {
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
}
