use terminal::{cursor::new, Background};

mod shapes;
mod terminal;
use terminal::{Ansi, Color, Coordinate,Alignment};

use crate::terminal::tui::S;

fn main() {
    let mut tui = terminal::tui::new();
    let mut test = new();
    // test.set_background(&Background(Color::RedBright));
    test.set_text(&terminal::Text {
        color: Color::BlueBright,
        effect: vec![terminal::Effect::Reversed, terminal::Effect::Bold],
    });

    test.draw(
        Coordinate(0, 0),
        // 5,
        tui.percent(20., S::Height),
        tui.percent(20., S::Width),
        shapes::rectangle,
    );
    finish();
}

fn finish() {
    let mut r#final = new();
    r#final.set_text(&terminal::Text {
        color: Color::White,
        effect: vec![terminal::Effect::Normal],
    })
}
