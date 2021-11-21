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
        ctx.print_centered(6, "Press any key to go back to the menu. :)");
        // It doesn't actually seem to pause here, it just goes straight back to main
        // menu.
        if let Some(_key) = ctx.key {
            self.menu(ctx);
        }
    }

    // TODO: Make this an actual game
    fn play(&mut self, ctx: &mut BTerm) {
       self.mode = GameMode::Playing;
       ctx.cls_bg(NAVY);
       ctx.print_centered(5, "You are currently playing.");
       ctx.print_centered(6, &format!("The word is {}", self.word));

       self.show_grid(ctx);

       if let Some(key) = ctx.key {
            let good_key = check_key(key);
            ctx.print_centered(7, format!("good key: {}", good_key));
            // How could I insert a short pause here? 
            if good_key {
                self.win(ctx);
            } else {
       //         self.bad_guess(ctx);
            }
       }
    }

    fn bad_guess(&mut self, ctx: &mut BTerm) {        
        ctx.print_centered(7, "bad guess, press any key to continue");
        if let Some(_key) = ctx.key {
            self.play(ctx)
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