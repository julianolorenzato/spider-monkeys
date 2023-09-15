use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, Mesh, Rect, Text};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};

struct GameState {
    spaceship: Mesh,
    is_active_engine: bool,
    height: f32,               // m
    speed: f32,                // m/s
    gravity_acceleration: f32, // m/s²
    engine_acceleration: f32,  // m/s²
    fuel: f32,                 // l
}

const FPS: u32 = 60;

impl GameState {
    pub fn new(ctx: &mut Context) -> GameState {
        let spaceship = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(360.0, 0.0, 80.0, 150.0),
            Color::WHITE,
        )
        .unwrap();

        GameState {
            spaceship,
            is_active_engine: false,
            height: 0.0, // increase from top to bottom
            speed: 0.0,
            gravity_acceleration: 10.0,
            engine_acceleration: 40.0,
            fuel: 100.0,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(FPS) {
            // Divide all values per FPS to keep the values per second correct
            let speed = self.speed / FPS as f32;
            let gravity_acceleration = self.gravity_acceleration / FPS as f32;
            let engine_acceleration = self.engine_acceleration / FPS as f32;

            // Increase height based on speed
            self.height += speed;

            // Slow down if engine is active
            if self.is_active_engine && self.fuel > 0.0 {
                self.speed += gravity_acceleration - engine_acceleration;
                self.fuel -= 0.3;
            } else {
                self.speed += gravity_acceleration;
            }

            if self.height < 0.0 {
                self.speed = 0.0;
                self.height = 0.0;
            }

            // Each second do this:
            // if ctx.time.ticks() as u32 % FPS == 0 {
            //     println!("height: {}", self.height);
            //     println!("speed: {}", self.speed);
            //     println!("motor active: {}", self.is_active_engine);
            //     println!("gravity acceleration: {}", self.gravity_acceleration);
            // }

            // if self.fuel <= 0.0 {
            //     println!("GAMEOVER, NO FUEL");
            //     ctx.request_quit()
            // }

            if self.height >= 450.0 {
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

        canvas.draw(&self.spaceship, Vec2::new(0.0, self.height));

        canvas.draw(
            &Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(20.0, 20.0, self.fuel * 2.0, 20.00),
                Color::BLUE,
            )
            .unwrap(),
            Vec2::new(0.0, 0.0),
        );

        let fuel_str = format!(
            "Combustível: {} L",
            if self.fuel < 0.0 { 0.0 } else { self.fuel }
        );
        canvas.draw(&Text::new(fuel_str), Vec2::new(40.0, 22.0));

        let speed_str = format!("Velocidade: {} m/s", self.speed);
        canvas.draw(&Text::new(speed_str), Vec2::new(550.0, 20.0));

        let height_str = format!("Altura: {} m", self.height * -1.0 + 450.0);
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
            Some(KeyCode::NumpadEnter) => self.is_active_engine = true,
            _ => (),
        }

        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> GameResult {
        match input.keycode {
            Some(KeyCode::NumpadEnter) => self.is_active_engine = false,
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
