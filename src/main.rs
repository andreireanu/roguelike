use rltk::{Rltk, BTermBuilder, GameState, embedded_resource};
use bracket_lib::prelude::*;
use std::fmt::Display;

embedded_resource!(WIDE_FONT, "resources/terminal_10x16.png");
embedded_resource!(VGA_FONT, "resources/vga8x16.png");

const WIDTH: i32 = 40;
const HEIGHT: i32 = 25;
pub enum RunState{
    WelcomeScreen,
    StartGame,
}


struct State { display: RunState}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();

        match self.display {
            RunState::WelcomeScreen => {
                self.center_at_row(ctx, 6, "Welcome to the Prog Rogue");
                self.center_at_row(ctx, 7, "A programable Roguelike");
                self.center_at_row(ctx, 8, "Press ENTER to Start ");
                if let Some(key) = ctx.key {
                    match key {
                    VirtualKeyCode::Return => self.display = RunState::StartGame,
                    _ => {}
                    }
                }
            }
            RunState::StartGame => {
                self.center_at_row(ctx, 12, "Game has begun");

            }
        }
    }
 }

impl State {
    fn center_at_row<D: Display>(&self, ctx: &mut BTerm, row: i32, message: D){
        let s = format!("{message}");
        let col = (WIDTH - s.len() as i32) / 2;  
        ctx.print(col, row, s);
    } 
}


fn main() -> BError { 
     link_resource!(WIDE_FONT, "resources/terminal_10x16.png");
     link_resource!(VGA_FONT, "resources/vga8x16.png");

    let context  = BTermBuilder::new()
        .with_simple_console(WIDTH, HEIGHT , "terminal_10x16.png")
        .with_title("Roguelike Tutorial")
        .with_font("terminal_10x16.png", 10, 16)
        .with_tile_dimensions(10, 16)
        .build()?;
    let gs = State{ display: RunState::WelcomeScreen};
    rltk::main_loop(context, gs)
}
 