use bracket_lib::prelude::*;

struct State {
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        //TODO: fill in this stub later
        self.mode = GameMode::End;
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You died!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
       Player {
        x,
        y,
        velocity: 0.0,
       } 
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@')
        )
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?;
    main_loop(context, State::new())
}

//worked on rust all day.
//worked on algorithms in Rust
//worked on rust today.
//worked on rust today at work.
//read the rust book today.
//did lets get rusty
//worked on rust at work
//Worked on rust all day at work.
//watched http crash course with brad traversy