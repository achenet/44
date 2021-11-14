use bracket_lib::prelude::*;
use std::fs;

const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
const FRAME_DURATION : f32 = 75.0;
const DictionaryFilePath : &str = "dictionary/dictionary.json";

enum GameMode {
    Menu,
    Play,
}

struct State {
    mode: GameMode,
    dico: Vec<Entry>,
    current_word: String,
    grid: Vec<char>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Play => self.play(ctx)
        }
    }
}

impl State {
    fn new(dico: &Vec<Entry>) -> Self {
        State {
            mode: GameMode::Menu,
            current_word: "Hi".to_string(),
            grid:  vec!['-','i'],
            dico: dico.to_vec(),
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "'sup, dawg?");
        let out = pretty_print_dico(&self.dico);
        ctx.print_centered(7, out);

        if let Some(key) = ctx.key {
            ctx.quitting = true;
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
            ctx.cls();
            ctx.print_centered(5, "Press Q to quit.");
            ctx.print_centered(6, "Press A to win.");

            if let Some(key) = ctx.key {
                match key {
                    VirtualKeyCode::Q => ctx.quitting = true,
                    VirtualKeyCode::A => self.good_input(ctx),
                    _ => self.bad_input(ctx),
                }
            }
    }
    fn good_input(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You win! Press 'a' to play again.");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::A=> self.play(ctx),
                _ => ctx.quitting = true,
            }
        }
    }
    fn bad_input(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "nope.");
        if let Some(key) = ctx.key {
            self.play(ctx);
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Vidya Gaems!")
        .build()?;

    let mut d = ReadDictionary();
    main_loop(context, State::new(&d))
}


#[derive(Copy, Clone)]
struct Entry {
    key: String,
    val: String,
}

fn ReadDictionary() -> Vec<Entry>{
    let contents = fs::read_to_string(DictionaryFilePath)
        .expect("Error reading file");

    // Input into adequate data structure.
    // Let's try an array of a certain length.

    let lines = contents.split("\n");
    // Create an array of size len(lines)
    let mut dico = Vec::new();
    for (idx, line) in lines.enumerate() {
        dico.push( Entry { key: "smoke weed".to_string(), val: line.to_string()})
    }
    dico
}

// Take a dico and 'flatten' it into a string.
fn pretty_print_dico(dico: &Vec<Entry>) -> String {
    let mut out = "".to_string();
    for entry in dico {
        out = out + &entry.key;
        out = out + &entry.val;
    }
    out
}
