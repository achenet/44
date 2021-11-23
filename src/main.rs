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
    bad_guess: bool,
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
            bad_guess: false, 
        }
    }

    fn win(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Win;
        ctx.cls();
        ctx.print_centered(5, "You win!");
        ctx.print_centered(6, "Press M to go back to the menu. :)");
        // It doesn't actually seem to pause here, it just goes straight back to main
        // menu.
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::M => self.menu(ctx),
                _ => {},
            }
        }
    }

    // TODO: Make this an actual game
    fn play(&mut self, ctx: &mut BTerm) {
       self.mode = GameMode::Playing;
       ctx.cls_bg(NAVY);
       ctx.print_centered(5, "You are currently playing.");
       ctx.print_centered(6, &format!("The word is {}", self.word));
       if self.bad_guess {
        ctx.print_centered(7, "Incorrect guess, try again.");
       }
       self.show_grid(ctx);

       if let Some(key) = ctx.key {
            let good_key = check_key(key);
            if good_key {
                self.win(ctx);
            } else {
                self.bad_guess = true;
            }
       }
    }


    fn show_grid(&mut self, ctx: &mut BTerm) {
        // Make the grid using self word
        let mut grid = String::new();
        for _value in vec!["wander", "weg"] {
            grid += " _";
        }
        // Print it
        ctx.print_centered(8, grid);
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

fn check_key(key: VirtualKeyCode) -> bool {
    match key {
        VirtualKeyCode::W => return true,
        VirtualKeyCode::A => return true,
        VirtualKeyCode::N => return true,
        VirtualKeyCode::D => return true,
        VirtualKeyCode::E => return true,
        _ => return false,
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Guess the word!")
        .build()?;

    main_loop(context, State::new("wanderweg!".to_string()))
}