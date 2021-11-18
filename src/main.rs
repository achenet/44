use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    Win,
}

struct State {
    mode: GameMode,
    word: String,
    grid: Vec<char>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::Win => self.win(ctx),
        }
    }
}

impl State {
    fn new(word: String) -> Self {
        State {
            mode: GameMode::Menu,
            word: word,
            grid: vec![],
        }
    }

    fn win(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Win;
        ctx.cls();
        ctx.print_centered(5, "You win!");
        if let Some(key) = ctx.key {
            self.menu(ctx);
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
       self.mode = GameMode::Playing;
       ctx.cls_bg(NAVY);
       ctx.print_centered(5, "You are currently playing.");
       ctx.print_centered(6, &format!("The word is {}", self.word));
       if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::A => self.win(ctx),
                _ => {
                    ctx.print_centered(5, "Press A");
                }
            }
       }
    }

    fn menu(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Menu;
        ctx.cls();
        ctx.print_centered(5, "You in the menu now, son.");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.play(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Guess the word!")
        .build()?;

    main_loop(context, State::new("wanderweg!".to_string()))
}
