extern crate bitflags;
extern crate pixel_engine as px;
use px::traits::*;
use px::vector2::*;

const NES_SCREEN: Vu2d = Vu2d { x: 256, y: 240 };
const NES_OFFSET: Vu2d = Vu2d { x: 5, y: 5 };

mod bus;
mod cpu;
pub mod utils;

fn main() {
    px::launch(async {
        let mut engine = px::EngineWrapper::new(
            "Runes - NES emulator".to_string(),
            (NES_SCREEN.x + 250, NES_SCREEN.y + 150, 2),
        )
        .await;
        engine.clear(px::Color::DARK_BLUE);
        engine.draw_rect(
            NES_OFFSET.cast_i32() - (1, 1).into(),
            NES_SCREEN.cast_i32() + (1, 1).into(),
            px::Color::RED,
        );
        engine.run(|game: &mut px::Engine| {
            if game.get_key(px::inputs::Keycodes::Escape).any() {
                return Ok(false);
            }

            Ok(true)
        });
    })
}
