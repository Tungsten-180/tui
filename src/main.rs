mod useful;
use useful::Coordinate;
mod shapes;
use terminal::{Cursor, tui::{S,Tui}, Color, Background};
mod terminal;
mod items;

fn main() {
    let mut tui = Tui::new();
    let mut test = Cursor::new();
    test.set_background(&Background(Color::RedBright));
    test.set_text(&terminal::Text {
        color: Color::BlueBright,
        effect: vec![terminal::Effect::Reversed, terminal::Effect::Bold],
    });

    test.draw(
        Coordinate(0, 0),
        // 5,
        tui.percent(100., S::Height),
        tui.percent(100., S::Width),
        shapes::rectangle,
    );
    // println!("hello world!");
    finish();
}

fn finish() {
    let mut r#final = new();
    r#final.set_text(&terminal::Text {
        color: Color::White,
        effect: vec![terminal::Effect::Normal],
    })
}
