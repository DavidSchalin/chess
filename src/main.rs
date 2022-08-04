pub mod gamestate;
pub mod piece;

use gamestate::*;
use piece::*;

const WHITE: Color = Color::WHITE;
const BLACK: Color = Color::BLACK;

fn main() {
    play();
    println!("Chess Finished!");
}

fn serialize(gs: &mut GameState) -> bool {
    gs.serialize_me();
    true
}

/// It creates a new game state, prints the board, and then loops forever, calling player_move on the
/// game state.
fn play(){

    let mut gamestate: GameState = GameState::new();
    gamestate.print_board();
    loop {
        //gamestate.print_me();
        gamestate.print_unicode();
        if let gamestate = player_command(&mut gamestate) {
            //Continue
        } else {
            println!("Something went wrong! Could not create new gamestate");
            break;
        }
    }
}

///
/// A function to read a players move and returns the validity of the move as well as the move itself.
///    
/// ### Returns 
/// a tuple (bool, usize, usize) representing (validity, start index of move, target index of move)
///
fn player_command(gamestate: &mut GameState) -> Option<usize> {

    let mut command: bool = false;
    let turn = match gamestate.current_player {
        WHITE => "WHITE",
        BLACK => "BLACK",
        _ => {panic!("what?");}
    };
    println!("Debug: {}", gamestate.debug_flag);
    let mut line = String::new();
    println!("Player: {}'s move: ", turn);
    if gamestate.checked_flag {
        println!("Checked: {:?}!", gamestate.checked_player);
    }
    std::io::stdin().read_line(&mut line).unwrap();

    if gamestate.debug_flag {
        if line == "place_piece\n" {
            println!("Enter: coord, color, piecetype");
            line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();

            let command_iter: Vec<&str> = line.split(" ").collect();

            let coord = GameState::coordinate_translator_str(command_iter[0]);
            gamestate.place_piece(coord, command_iter[1], command_iter[2].strip_suffix('\n')?);
            command = true;
        } else if line == "leave_debug\n" {
            println!("left debug mode...");
            gamestate.debug_flag = false;
            command = true;
        } else if line == "serialize\n"{
            println!("serializing...");
            serialize(gamestate);
            println!("done!");
            line = String::new();
            command = true;
        } else if line == "black\n"{
            println!("swapping to black...");
            gamestate.current_player = BLACK;
            command = true;
        } else if line == "white\n" {
            println!("swapping to white...");
            gamestate.current_player = WHITE;
            command = true;
        } else if line == "new_game_custom\n" {
            println!("Enter a specific game state: ");
            line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let arg: &str = line.strip_suffix('\n')?;
            println!("argument given: '{}'", arg);
            gamestate.new_custom(arg);
            return Some(0);
        }
    }

    if line == "debug_mode\n" {
        println!("Entered debug mode");
        gamestate.debug_flag = true;

        // line = String::new();
        // std::io::stdin().read_line(&mut line).unwrap();

    } else {

        if !command {
            let player_move_command_iter: Vec<&str> = line.split(" ").collect();
            if player_move_command_iter.len() == 3 {
                let valid_move = gamestate.move_validity_checker(player_move_command_iter[0], player_move_command_iter[2].strip_suffix('\n')?);
                if valid_move {
                    println!("Valid move!: {} -> {}", player_move_command_iter[0], player_move_command_iter[2].strip_suffix('\n')?);
                    gamestate.do_valid_move(player_move_command_iter[0], player_move_command_iter[2].strip_suffix('\n')?);
                } else {
                    println!("Invalid move!: {} -> {}", player_move_command_iter[0], player_move_command_iter[2].strip_suffix('\n')?);
                    
                }
            } else if player_move_command_iter.len() == 1 {
                println!("Valid moves: {}", gamestate.get_moves_from_tile_as_string(GameState::coordinate_translator_str(player_move_command_iter[0].strip_suffix('\n')?)));
            }

        }
    }

    Some(0)

}