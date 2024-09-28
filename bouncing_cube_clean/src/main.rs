use ggez::*;

const WINDOW_WIDTH:f32 = 750.0;
const WINDOW_HEIGHT:f32 = 550.0;



// Everything we want to track
struct State{
    player_pos_x: f32,
    player_pos_y:f32,
    player_vel_x: f32,
    player_vel_y: f32
}

impl ggez::event::EventHandler<GameError> for State {
    // everything we need to update
    fn update(&mut self, ctx:&mut Context) -> GameResult {
        let check = check_wall_collision(self.player_pos_x, self.player_pos_y);
        match check.0 {
            false => {},
            true => {
                match check.1 {
                    true => self.player_vel_y *= -1.0,
                    false => self.player_vel_x *= -1.0
                }
            }
        }

        self.player_pos_x += self.player_vel_x;
        self.player_pos_y += self.player_vel_y;

        Ok(())
    }
    //everything we need to draw
    fn draw(&mut self, ctx:&mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);


        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.player_pos_x, self.player_pos_y, 50.0, 50.0),
            graphics::Color::WHITE,
        )?;

        let mut mesh = Vec::new();
        mesh.push(rect);


        for element in mesh {
            canvas.draw(&element, graphics::DrawParam::default());
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}


pub fn main() {
    let state = State {player_pos_x:200.0, player_pos_y:200.0, player_vel_x:5.0, player_vel_y:5.0};
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello_world", "Sasha")
        .default_conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state);
}


pub fn check_wall_collision(x:f32, y:f32) -> (bool, bool){
    if y >= WINDOW_HEIGHT || y <= 0.0{
        return (true, true)
    };
    if x >= WINDOW_WIDTH || x <= 0.0 {
        return (true, false)
    };
    return (false, false)
}