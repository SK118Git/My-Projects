mod tile;
mod playerstats;
use dioxus::html::sub;
use tile::Tile;


pub use tile::PlayerTag;
pub use playerstats::gen_player_list;

use crate::libs;
use libs::battle_loop;


use std::fs::{self, File};
use std::io::prelude::*;

pub struct GameState {
    pub board: Vec<Vec<Tile>>,
}


impl GameState {
    pub fn initialize_board(board_size: usize, player_count: usize) -> GameState{
        let mut new= Vec::new();
        let mut id = 0;
        while id < board_size*board_size {
            let mut row = Vec::new();
            for _k in 0..board_size {
                row.push(Tile::new(id, player_count));
                id += 1;
            }
            new.push(row)
        }
       return GameState {board:new}
    }


    pub fn display_board(&self, choice: i32){
        for x in &self.board{
            for y in x{
                print!("|");
                y.display(choice)
            }
            print!("|");
            println!("");
        }
        println!("")
    }
    
    pub fn find_tile(&self, id: usize) -> (usize, usize){
        let mut x:usize = 0;
        let mut y:usize = 0;
        if id > 24 {
                panic!("Tile out of bounds")
        }
        for element in &self.board{
            y = 0;
            for sub_element in element {
                if sub_element.id == id {
                    return (x,y)
                }
                y += 1
            }
            x += 1
        }
        return (x, y)
    }   

    pub fn attack(&mut self, f_tile_id: usize, t_tile_id: usize) -> bool {
        let (x_f, y_f) = self.find_tile(f_tile_id);
        let (x_t, y_t) = self.find_tile(t_tile_id);
        let f_tile = &self.board[x_f][y_f].unit_count;
        let t_tile = &self.board[x_t][y_t].unit_count;
        if f_tile == &1 {
            return false
        }
        let (a, d) = battle_loop(*f_tile - 1, *t_tile);
        if d == 0 {
            self.board[x_t][y_t].owner = self.board[x_f][y_f].owner;
            self.board[x_t][y_t].unit_count = a;
            self.board[x_f][y_f].unit_count -= a; 
            return true 
        }
        self.board[x_f][y_f].unit_count = a;
        self.board[x_t][y_t].unit_count = d;
        return false 
    }

    pub fn save(&self) -> std::io::Result<()>{
        //let mut file = File::open("savefile.txt")?;
        let mut input = String::new();
        for element in &self.board {
            for sub_element in element {
                input.push_str(sub_element.to_string().as_str());
                input.push_str("\n");
                //println!("{input}");
            }
        }
        fs::write("savefile.txt", &input)?;
        Ok(())
    }  
}


