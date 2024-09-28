use std::fs;
use std::io::{Read, Write};
use ggez::*;
use rand::Rng;

const WINDOW_HEIGHT:f32 = 550.0;
const WINDOW_WIDTH:f32 = 750.0;

const WINDOW_HEIGHT_I16:i16 = 550;
const WINDOW_WIDTH_I16:i16 = 750;

const BOX_SIZE:f32 = 20.0;


// Everything we want to track
struct State{
    player_pos_x: f32,
    player_pos_y:f32,
    player_vel_x: f32,
    player_vel_y: f32,
    apple_pos_x: f32,
    apple_pos_y:f32,
    points: i16,
    prior_positions: Vec<(f32, f32)>,
    gameover: bool,
    game_over_screen_timer: i32,
    has_written_score:bool,
    scoreboard:String,
}

impl event::EventHandler<GameError> for State {
    // everything we need to update
    fn update(&mut self, ctx:&mut Context) -> GameResult {
        let keyspressed = &ctx.keyboard;


        let wall_collide = check_wall_collision(self.player_pos_x, self.player_pos_y);
        if wall_collide {
            self.gameover = true
        }

        if self.prior_positions.len() >2 {
            if self.prior_positions[0..self.prior_positions.len() - 2].contains(&(self.player_pos_x, self.player_pos_y))
            {
                self.gameover = true
            }
        }

        if self.prior_positions.len() > 0 {
            self.prior_positions.remove(0);
            self.prior_positions.push((self.player_pos_x, self.player_pos_y));
        }

        if keyspressed.is_key_pressed(input::keyboard::KeyCode::Left) {
            self.player_vel_x *= 0.0;
            self.player_vel_x += -3.0;
            self.player_vel_y *= 0.0
        }

        if keyspressed.is_key_pressed(input::keyboard::KeyCode::Right) {
            self.player_vel_x *= 0.0;
            self.player_vel_x += 3.0;
            self.player_vel_y *= 0.0
        }

        if keyspressed.is_key_pressed(input::keyboard::KeyCode::Up) {
            self.player_vel_y *= 0.0;
            self.player_vel_y += -3.0;
            self.player_vel_x *= 0.0
        }

        if keyspressed.is_key_pressed(input::keyboard::KeyCode::Down) {
            self.player_vel_y *= 0.0;
            self.player_vel_y += 3.0;
            self.player_vel_x *= 0.0
        }

        self.player_pos_x += self.player_vel_x;
        self.player_pos_y += self.player_vel_y;

        Ok(())
    }
    //everything we need to draw
    fn draw(&mut self, ctx:&mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::GREEN);
        let mut mesh = Vec::new();

        let mut score = graphics::Text::new("Points: ");
        score.add(self.points.to_string());
        score.set_scale(50.0);

        let mut game_over_text = graphics::Text::new("Game Over \n");

        if self.gameover {


            if !self.has_written_score{
                self.scoreboard = write_high_score(self.points);
                self.has_written_score = true;
            }
            self.game_over_screen_timer += 1;
            if self.game_over_screen_timer >= 200 {
                std::process::exit(0)
            };

            game_over_text.add(self.scoreboard.as_str());
            game_over_text.set_scale(70.0);

            canvas.draw(&game_over_text, graphics::DrawParam::default());

            canvas.finish(ctx)?;
            Ok(())
        }
        else {
            if self.apple_collision() {
                (self.apple_pos_x, self.apple_pos_y) = gen_random_spawn();
                self.points += 1;
                self.prior_positions.push((self.player_pos_x, self.player_pos_y));
            }

            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(self.player_pos_x, self.player_pos_y, BOX_SIZE, BOX_SIZE),
                graphics::Color::WHITE,
            )?;

            let apple = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(self.apple_pos_x, self.apple_pos_y, BOX_SIZE, BOX_SIZE),
                graphics::Color::RED,
            )?;

            for tail_parts in &self.prior_positions {
                let tail_segments = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(tail_parts.0, tail_parts.1, BOX_SIZE, BOX_SIZE),
                    graphics::Color::WHITE,
                )?;
                mesh.push(tail_segments);
            }

            mesh.push(rect);
            mesh.push(apple);

            for element in mesh {
                canvas.draw(&element, graphics::DrawParam::default());
            };
            canvas.draw(&score, graphics::DrawParam::default());
            canvas.finish(ctx)?;
            Ok(())
        }
    }


}

impl State {
    fn apple_collision(&self) -> bool {
        let mut collide = false;
        let dif_x = self.player_pos_x - self.apple_pos_x;
        let dif_y = self.player_pos_y - self.apple_pos_y;
        if (dif_x.abs() <= BOX_SIZE) && (dif_y.abs() <= BOX_SIZE) {
            collide = true
        }
        return collide
    }
}

pub fn main() {

    let state = State {player_pos_x:200.0, player_pos_y:200.0, player_vel_x:0.0, player_vel_y:0.0, apple_pos_x: 300.0, apple_pos_y:300.0, points:0, prior_positions:Vec::new(), gameover: false, game_over_screen_timer: 0, has_written_score: false, scoreboard: String::new()};
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello_world", "Sasha")
        .default_conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state);

}


pub fn check_wall_collision(x:f32, y:f32) -> bool{
    if y >= WINDOW_HEIGHT || y <= 0.0{
        return true
    };
    if x >= WINDOW_WIDTH || x <= 0.0 {
        return true
    };
    return false
}

fn gen_random_spawn() -> (f32, f32) {
    let x:f32 = rand::thread_rng().gen_range(0..WINDOW_WIDTH_I16) as f32;
    let y:f32 = rand::thread_rng().gen_range(0..WINDOW_HEIGHT_I16) as f32;
    return (x, y)

}


fn write_high_score(points:i16) -> String{
    let mut string_to_write = String::from(format!("{points}"));
    string_to_write.push_str("\n");
    let points_as_bytes = string_to_write.into_bytes();
    let mut file = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("scores.txt")
        .unwrap();
    file.write_all(&*points_as_bytes).unwrap();

    let mut file_3 = fs::File::create("highscores.txt").unwrap();
    file_3.write_all(find_top_three().as_bytes()).unwrap();

    let file2 = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("highscores.txt")
        .unwrap();
    let mut content = String::from("Highscore: \n");
    content.push_str(std::io::read_to_string(file2).unwrap().as_str());
    return content
}

fn find_top_three() -> String{
    let mut scores:Vec<i32> = Vec::new();
    let mut file = fs::File::open("scores.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines().into_iter().filter(|line| !line.is_empty() ) {
        scores.push(line.parse::<i32>().unwrap());
    }
    scores.sort();
    let mut top_3 = String::new();
    top_3.push_str(scores[scores.len()-1].to_string().as_str());
    top_3.push_str("\n");
    top_3.push_str(scores[scores.len()-2].to_string().as_str());
    top_3.push_str("\n" );
    top_3.push_str(scores[scores.len()-3].to_string().as_str());
    return top_3
}