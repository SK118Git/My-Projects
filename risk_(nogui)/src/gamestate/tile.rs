use rand::Rng;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum PlayerTag {
    Empty,
    P1,
    P2,
    P3,
    P4
}

pub fn random_player(player_count:usize) -> PlayerTag{
    match player_count {
       1 => { 
        let rando = rand::thread_rng().gen_range(0..2);
        match rando {
             0 => PlayerTag::Empty,
             1 => PlayerTag::P1,
             _ => {println!("shouldn't happen"); PlayerTag::Empty}
        }
       }
       2 => {
        let rando = rand::thread_rng().gen_range(1..3);
        match rando {
             //0 => PlayerTag::Empty,
             1 => PlayerTag::P1,
             2 => PlayerTag::P2,
             _ => {println!("shouldn't happen"); PlayerTag::Empty}
        }
       }
       3 => {
        let rando = rand::thread_rng().gen_range(1..4);
        match rando {
             //0 => PlayerTag::Empty,
             1 => PlayerTag::P1,
             2 => PlayerTag::P2,
             3 => PlayerTag::P3,
             _ => {println!("shouldn't happen"); PlayerTag::Empty}
        }
       }
       _ => {
        let rando = rand::thread_rng().gen_range(1..5);
        match rando {
             //0 => PlayerTag::Empty,
             1 => PlayerTag::P1,
             2 => PlayerTag::P2,
             3 => PlayerTag::P3,
             4 => PlayerTag::P4,
             _ => {println!("shouldn't happen"); PlayerTag::Empty}
        }
       }
    }
}

#[derive(Clone, Copy)]
pub struct Tile{
    pub id: usize,
    pub owner: PlayerTag,
    pub unit_count: i32,
}


impl Tile {
    pub fn new(id: usize, player_count:usize) -> Tile {
        Tile {id: id, owner: random_player(player_count), unit_count: gen_first_unit()}
    }

    pub fn display(&self, choice: i32) {
        match choice {
            1 => {print!("{:?}", self.id)},
            2 => {print!("{:?}", self.owner)},
            3 => {print!("{:?}", self.unit_count)},
            _ => {print!("({:?}, {:?}, {:?})", self.id, self.owner, self.unit_count)},
        }
    }

    pub fn to_string(&self) -> String{
        let mut conversion = String::new();
        //conversion.push_str(&self.id.to_string());
        //conversion.push('{');
        let ptag = match self.owner {
            PlayerTag::P1 => "P1",
            PlayerTag::P2 => "P2",
            PlayerTag::P3 => "P3",
            PlayerTag::P4 => "P4",
            PlayerTag::Empty => "Empty"
        };
        conversion.push_str(ptag);
        conversion.push(',');
        conversion.push_str(&self.unit_count.to_string());
        //conversion.push('}');
        return conversion
    }
}

pub fn gen_first_unit() -> i32 {
    let mut rng= rand::thread_rng(); 
    let rando:f32 = rng.gen();
    if rando < 1.0/3.0 {
        1
    }
    else if 2.0/3.0 < rando {
        2
    }
    else {
        3
    }

}
