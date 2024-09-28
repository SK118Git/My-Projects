use rand::Rng;
use std::io;

pub fn ask_player_move(request:&str) -> usize{
    let mut player_move = String::new();
    println!("{request}");
    io::stdin()
        .read_line(&mut player_move)
        .expect("failure to read line");
    player_move.trim().parse().expect("Invalid Input")
}


pub fn battle(mut attackers:i32, mut defenders: i32) -> (i32, i32){
    let mut attacker_rolls: Vec<i32> = Vec::new();
    let mut defender_rolls: Vec<i32> = Vec::new();
    for _k in 0..attackers {
        attacker_rolls.push(rand::thread_rng().gen_range(0..7))
    }
    for _i in 0..defenders {
        defender_rolls.push(rand::thread_rng().gen_range(0..7))
    }
    attacker_rolls.sort();       
    attacker_rolls.reverse();
    defender_rolls.sort();
    defender_rolls.reverse();

    let mut j = 0;
    for roll in defender_rolls {
        if roll >= attacker_rolls[j] {
            attackers -= 1;
        }
        else {
            defenders -= 1;
        }
        j = j + 1;
        if j == attacker_rolls.len() {
            break
        }
    }
    return (attackers, defenders)
}

pub fn battle_loop(attackers: i32, defenders: i32) -> (i32, i32){
    let mut x: i32 = attackers;
    let mut y: i32 = defenders;
    let mut continues = true;
    let mut message = String::new();
    while continues {
        (x, y) = battle(x, y); 
        println!("Attackers left: {x}, Defenders left: {y}");
        println!("Continue attack?");
        io::stdin()
            .read_line(&mut message)
            .expect("failure to read line");
        match message.trim().to_lowercase().as_str() {
            "yes" | "y" | "1" => continues = true,
            "no" | "n" | "0" => continues = false,
            _ => continues = true
        }
        if x == 0 || y == 0 {
            continues = false
        }
    }
    return (x, y)
}


pub fn choice(question: &str) -> bool {
    let doit:bool;
    let mut message = String::new();
    println!("{question}");
    io::stdin()
        .read_line(&mut message)
        .expect("failure to read line");
    match message.trim().to_lowercase().as_str() {
            "yes" | "y" | "1" => doit = true,
            "no" | "n" | "0" => doit = false,
            _ => doit = true
    }
    return doit
}