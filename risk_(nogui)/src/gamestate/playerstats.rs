use crate::gamestate::tile::Tile;
use crate::libs::{ask_player_move, choice};
use crate::GameState;
use crate::PlayerTag;

use rand::Rng;
use std::io;

#[derive(Debug)]
pub struct PlayerStats {
    pub player_name: PlayerTag,
    cards: Vec<i32>,
    owned_ters: i32,
}

impl PlayerStats {
    pub fn new(tag: PlayerTag) -> PlayerStats {
        PlayerStats {
            player_name: tag,
            cards: Vec::new(),
            owned_ters: 0,
        }
    }
    pub fn count_ters(&self, cur_gamestate_board: &Vec<Vec<Tile>>) -> i32 {
        let mut count = 0;
        for element in cur_gamestate_board {
            for sub_element in element {
                if sub_element.owner == self.player_name {
                    count += 1
                }
            }
        }
        return count;
    }
    pub fn update_ters(&mut self, cur_gamestate_board: &Vec<Vec<Tile>>) {
        self.owned_ters = self.count_ters(cur_gamestate_board)
    }

    pub fn player_turn(&mut self, cur_gamestate: &mut GameState) {
        //update player info:
        self.update_ters(&cur_gamestate.board);

        //give troops (TODO)
        let mut new_troops: i32 = self.calculate_troops();

        //place troops
        self.place_troops_loop(new_troops, cur_gamestate);

        //attack:
        self.player_attack(cur_gamestate);

        //reorganise troops (TODO)
        self.reorganise_troops(cur_gamestate);
        
        return;
    }

    pub fn place_troops_loop(&self, mut new_troops: i32, cur_gamestate: &mut GameState) {
        while new_troops > 0 {
            let troops_to_move: i32 = how_many_troops_2place(new_troops);
            let location = ask_player_move("where do you want to put the troops?");
            let (x, y) = cur_gamestate.find_tile(location);
            if cur_gamestate.board[x][y].owner == self.player_name {
                cur_gamestate.board[x][y].unit_count += troops_to_move;
                new_troops -= troops_to_move;
            }
            else {
                println!("You picked an invalid tiles");
                self.place_troops_loop(new_troops, cur_gamestate);
            }
        }
    }

    pub fn player_attack(&mut self, cur_gamestate: &mut GameState){
        if choice("Want to attack?") {
            let from_tile = ask_player_move("from where to stage attack?"); 
            let (from_tile_x, from_tile_y) = cur_gamestate.find_tile(from_tile);
            if cur_gamestate.board[from_tile_x][from_tile_y].owner == self.player_name{
                let to_tile = ask_player_move("which tile to attack?");
                let (check_tile_x, check_tile_y)  = cur_gamestate.find_tile(to_tile);
                if cur_gamestate.board[check_tile_x][check_tile_y].owner != self.player_name{
                    if cur_gamestate.attack(from_tile, to_tile) {
                        self.cards.push(gen_card())
                    }
                }
                else {
                    println!("You attacked an invalid attack location");
                    self.player_attack(cur_gamestate);
                }
            }
            else {
                println!("Incorrect staging ground");
                self.player_attack(cur_gamestate);
            } 
        }
        return 
    }

    pub fn calculate_troops(&self) -> i32 {
        let new_troops:i32;
        if self.owned_ters <= 11 {
            new_troops = 3
        } else if self.owned_ters <= 14 {
            new_troops = 4
        } else if self.owned_ters <= 17 {
            new_troops = 5
        } else {
            new_troops = 6
        }


        //TODO
        /*
        if owns_cont_1_flag {
            //to implement
        }

        if owns_cont_2_flag {
            //to implement
        }

        if owns_cont_3_flag {
            //to implement
        }

        if owns_cont_4_flag {
            //to implement
        }

        if owns_cont_5_flag {
            //to implement
        }

        if owns_cont_1_flag {
            //to implement
        }
        */
        return new_troops;
    }

    fn reorganise_troops(&mut self, cur_gamestate: &mut GameState){
        if choice("Re-organise armies?") {
            let from_tile = ask_player_move("From which tile to take units?"); 
            let (from_tile_x, from_tile_y) = cur_gamestate.find_tile(from_tile);
            if cur_gamestate.board[from_tile_x][from_tile_y].owner == self.player_name{
                let to_tile = ask_player_move("which tile to place troops?");
                let (check_tile_x, check_tile_y)  = cur_gamestate.find_tile(to_tile);
                    if cur_gamestate.board[check_tile_x][check_tile_y].owner == self.player_name{
                        let troops_from_tile = cur_gamestate.board[from_tile_x][from_tile_y].unit_count - 1;
                        let troops_to_move = how_many_troops_2place(troops_from_tile);
                        cur_gamestate.board[check_tile_x][check_tile_y].unit_count += troops_to_move;
                    }
                    else {
                        println!("That's not your tile!");
                        self.reorganise_troops(cur_gamestate);
                    }
            }
            else {
                println!("That's not your tile!");
                self.reorganise_troops(cur_gamestate);
            } 
        return 
        }
    }
}


pub fn gen_player_list(player_count: usize) -> Vec<PlayerStats> {
    let mut players: Vec<PlayerStats> = Vec::new();
    match player_count {
        0 => {
            players.push(PlayerStats::new(PlayerTag::Empty));
        }
        1 => {
            players.push(PlayerStats::new(PlayerTag::Empty));
            players.push(PlayerStats::new(PlayerTag::P1));
        }
        2 => {
            //players.push(PlayerStats::new(PlayerTag::Empty));
            players.push(PlayerStats::new(PlayerTag::P1));
            players.push(PlayerStats::new(PlayerTag::P2));
        }
        3 => {
            //players.push(PlayerStats::new(PlayerTag::Empty));
            players.push(PlayerStats::new(PlayerTag::P1));
            players.push(PlayerStats::new(PlayerTag::P2));
            players.push(PlayerStats::new(PlayerTag::P3));
        }
        _ => {
            //players.push(PlayerStats::new(PlayerTag::Empty));
            players.push(PlayerStats::new(PlayerTag::P1));
            players.push(PlayerStats::new(PlayerTag::P2));
            players.push(PlayerStats::new(PlayerTag::P3));
            players.push(PlayerStats::new(PlayerTag::P4));
        }
    }
    return players;
}

pub fn gen_card() -> i32 {
    let mut rng = rand::thread_rng();
    let rando: f32 = rng.gen();
    if rando < 1.0 / 3.0 {
        1
    } else if 2.0 / 3.0 < rando {
        5
    } else {
        10
    }
}


pub fn how_many_troops_2place(new_troops: i32) -> i32{
    let mut player_move: String = String::new();
    println!("You can place {new_troops} troops");
    println!("Select how many troops to place :");
    io::stdin()
        .read_line(&mut player_move)
        .expect("failure to read line");
    let amount:i32 = player_move.trim().parse().expect("Invalid Input"); 
    if amount <= new_troops{
        return amount
    }
    else {
        println!("Invalid troop amount");
        how_many_troops_2place(new_troops)
    }
}



