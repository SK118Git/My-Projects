mod libs;
mod gamestate;

use std::fs::File;
use std::io::Read;

use gamestate::GameState;
use gamestate::PlayerTag;

use crate::gamestate::gen_player_list;

const PLAYER_COUNT:usize = 3;

use dioxus::prelude::*;


fn main() {
    //println!("Hello, world!");
    game_loop(PLAYER_COUNT);
}


fn game_loop(player_count: usize){
    let mut cur_gamestate: GameState = GameState::initialize_board(5, player_count);
    let mut player_list = gen_player_list(player_count);
    cur_gamestate.save().unwrap();
    //dioxus_desktop::launch(App);
    loop {
        for player in &mut player_list {
            println!("Current player is: {:?}", player.player_name);
            cur_gamestate.display_board(3);
            cur_gamestate.display_board(2);
            player.player_turn(&mut cur_gamestate);
            cur_gamestate.display_board(3);
            cur_gamestate.display_board(2);
            println!("{:?}", player);
        }
    }
}

fn App(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    let mut current_save = File::open("savefile.txt").unwrap();
    let mut content = String::new();
    current_save.read_to_string(&mut content).unwrap();
    cx.render(rsx! {
        div {
            style { {include_str!("style.css")} }
            div {
                for line in content.lines(){
                    button {class: "button-key  {line.split(',').collect::<Vec<&str>>()[0]}", onclick: move |_| count +=1,  "{line.split(',').collect::<Vec<&str>>()[1]}"}
                }
            }
        }
    })
}
