use rltk::{Rltk, BTermBuilder, GameState, embedded_resource};
use bracket_lib::prelude::*;

embedded_resource!(WIDE_FONT, "resources/terminal_10x16.png");
embedded_resource!(VGA_FONT, "resources/vga8x16.png");

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> BError {
     link_resource!(WIDE_FONT, "resources/terminal_10x16.png");
     link_resource!(VGA_FONT, "resources/vga8x16.png");

    let context  = BTermBuilder::new()
        .with_simple_console(80, 50, "vga8x16.png")
        .with_title("Roguelike Tutorial")
        .with_font("vga8x16.png", 8, 16)
        .with_tile_dimensions(8, 16)
        .build()?;
    let gs = State{ };
    rltk::main_loop(context, gs)
}
 