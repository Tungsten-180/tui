use crate::shapes;
use crate::terminal::{tui::Tui, Color, Cursor, Effect};
use crate::useful::{Coordinate, Dimension};

pub enum ItemTextEffects {
    Normal,
    Bold,
    Underline,
    Blink,
    Hidden,
}
impl ItemTextEffects {
    fn as_effect(&self) -> Effect {
        match self {
            Normal => Effect::Normal,
            Bold => Effect::Bold,
            Underline => Effect::Underline,
            Blink => Effect::Blink,
            Hidden => Effect::Hidden,
        }
    }
}

pub struct Item {
    name: String,
    drawing_function: fn(Coordinate, Dimension),
    text_color: Color,
    item_color: Color,
    text: String,
    location: Coordinate,
    dimension: Dimension,
}
impl Default for Item {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            drawing_function: shapes::rectangle,
            text_color: Color::Black,
            item_color: Color::Cyan,
            text: "Hello World".to_string(),
            location: Coordinate::default(),
            dimension: Dimension::default(),
        }
    }
}

impl Item {
    pub fn draw(&mut self) {}
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_drawing_function(&mut self, drawing_function: fn(Coordinate, Dimension)) {
        self.drawing_function = drawing_function;
    }
    pub fn set_text_color(&mut self, text_color: Color) {
        self.text_color = text_color;
    }
    pub fn set_item_color(&mut self, item_color: Color) {
        self.item_color = item_color;
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
    pub fn set_location(&mut self, location: Coordinate) {
        self.location = location;
    }
    pub fn set_dimension(&mut self, dimension: Dimension) {
        self.dimension = dimension;
    }
    pub fn name(&mut self) -> &mut String {
        &mut self.name
    }
    pub fn drawing_function(&mut self) -> fn(Coordinate, Dimension) {
        self.drawing_function
    }
    pub fn text_color(&mut self) -> Color {
        self.text_color
    }
    pub fn item_color(&mut self) -> Color {
        self.item_color
    }
    pub fn text(&mut self) -> &mut String {
        &mut self.text
    }
    pub fn location(&mut self) -> Coordinate {
        self.location
    }
    pub fn dimension(&mut self) -> Dimension {
        self.dimension
    }
}
