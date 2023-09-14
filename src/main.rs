use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, Mesh, Rect, Text};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};

#[derive(Debug)]
struct GameState {
    spaceship: Mesh,
    landing_base: Mesh,
    fuel_indicator: Mesh,
    height: f32,              // m
    speed: f32,               // m/s
    gravity: f32,             // m/s²
    fuel: f32,                // l
    engine_acceleration: f32, // m/s²
}

const FPS: u32 = 60;
const DIMENSION_SCALE: f32 = 0.5;

impl GameState {
    pub fn new(ctx: &mut Context) -> GameState {
        let spaceship = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(360.0, 0.0, 80.0, 150.0),
            Color::WHITE,
        )
        .unwrap();

        let landing_base = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(5.0, 5.0, 50.0, 200.0),
            Color::WHITE,
        )
        .unwrap();

        let fuel_indicator = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(20.0, 20.0, 200.0, 100.00),
            Color::WHITE,
        )
        .unwrap();

        GameState {
            spaceship,
            landing_base,
            fuel_indicator,
            height: 0.0, // increase from top to bottom
            speed: 0.0,
            gravity: 10.0,
            fuel: 100.0,
            engine_acceleration: 10.0,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(FPS) {
            // Increase 10m/s to speed each second
            self.speed = self.speed + (self.gravity / FPS as f32);

            // Increase height based on speed modified by a scale factor
            self.height = self.height + self.speed;

            if self.height <= 0.0 {
                self.speed = 0.0;
                self.height = 0.0;
            }

            // Each second do this:
            if ctx.time.ticks() as u32 % FPS == 0 {
                println!("height: {}", self.height);
                println!("speed: {}", self.speed);
            }

            if self.height >= 900.0 {
                if self.speed > 5.0 {
                    println!("GAMEOVER")
                } else {
                    println!("WINNER")
                }

                ctx.request_quit()
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(
            &self.spaceship,
            Vec2::new(0.0, self.height * DIMENSION_SCALE),
        );

        let speed_str = format!("Velocidade: {} m/s", self.speed);
        let height_str = format!("Altura: {} m", self.height * -1.0 + 900.0);
        canvas.draw(&Text::new(speed_str), Vec2::new(550.0, 20.0));
        // canvas.draw(&self.fuel_indicator);
        canvas.draw(&Text::new(height_str), Vec2::new(550.0, 60.0));
        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult {
        match input.keycode {
            Some(KeyCode::NumpadEnter) => self.speed = self.speed - self.engine_acceleration,
            _ => (),
        }

        Ok(())
    }
}

fn main() {
    let cb = ContextBuilder::new("spider-monkeys", "julianolorenzato");
    let (mut ctx, event_loop) = cb.build().unwrap();
    let game = GameState::new(&mut ctx);
    event::run(ctx, event_loop, game)
}
