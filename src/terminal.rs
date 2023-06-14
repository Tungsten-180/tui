const ESC: &str = "\u{001b}";

#[derive(Clone, Copy, Debug)]
pub enum Alignment{
    Centered,
    None,
}
#[derive(Clone, Copy, Debug)]
struct Dimension {
    height: u32,
    width: u32,
}

#[derive(Clone, Copy)]
pub struct Coordinate(pub i32, pub i32);
impl Coordinate {
    pub fn as_array(&self) -> [i32; 2] {
        [self.0, self.1]
    }
}

#[derive(Clone)]
pub struct Text {
    pub color: Color,
    pub effect: Vec<Effect>,
}

#[derive(Clone)]
pub struct Background(pub Color);

#[derive(Clone)]
pub struct Cursor {
    pub written: bool,
    pub position: Coordinate,
    pub text: Text,
    pub background: Background,
}

#[derive(Clone, Copy)]
pub enum ClearCommands {
    ScreenCursorEnd,
    ScreenCursorBeginning,
    Screen,
    LineCursorEnd,
    LineCursorBeginning,
    Line,
    Reset,
}
#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BlackBright,
    RedBright,
    GreenBright,
    YellowBright,
    BlueBright,
    MagentaBright,
    CyanBright,
    WhiteBright,
}
#[derive(Clone, Copy)]
pub enum Effect {
    Normal,
    Bold,
    Underline,
    Reversed,
    Blink,
    Hidden,
    Dim,
}

pub trait Ansi {
    fn ansi_code(&self) -> String {
        match self {
            _ => "".to_string(),
        }
    }
}

pub trait Colored {
    fn color(&self) -> Color {
        Color::White
    }
    fn set_color(&mut self, color: Color) {}
}
pub trait Effectable {
    fn effect(&self) -> Effect {
        Effect::Normal
    }
    fn set_effect(&mut self, effect: Effect) {}
}

impl Colored for Text {
    fn color(&self) -> Color {
        self.color
    }
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Colored for Background {
    fn color(&self) -> Color {
        self.0
    }
    fn set_color(&mut self, color: Color) {
        self.0 = color;
    }
}

impl Ansi for Background {
    fn ansi_code(&self) -> String {
        String::from(ESC)
            + match self.color() {
                Color::Black => "[40m",
                Color::Red => "[41m",
                Color::Green => "[42m",
                Color::Yellow => "[43m",
                Color::Blue => "[44m",
                Color::Magenta => "[45m",
                Color::Cyan => "[46m",
                Color::White => "[47m",
                Color::BlackBright => "[40;1m",
                Color::RedBright => "[41;1m",
                Color::GreenBright => "[42;1m",
                Color::YellowBright => "[43;1m",
                Color::BlueBright => "[44;1m",
                Color::MagentaBright => "[45;1m",
                Color::CyanBright => "[46;1m",
                Color::WhiteBright => "[47;1m",
            }
    }
}

impl Ansi for Text {
    fn ansi_code(&self) -> String {
        format!(
            "{}{}{}",
            ESC,
            match self.color() {
                Color::Black => "[30m",
                Color::Red => "[31m",
                Color::Green => "[32m",
                Color::Yellow => "[33m",
                Color::Blue => "[34m",
                Color::Magenta => "[35m",
                Color::Cyan => "[36m",
                Color::White => "[37m",
                Color::BlackBright => "[30;1m",
                Color::RedBright => "[31;1m",
                Color::GreenBright => "[32;1m",
                Color::YellowBright => "[33;1m",
                Color::BlueBright => "[34;1m",
                Color::MagentaBright => "[35;1m",
                Color::CyanBright => "[36;1m",
                Color::WhiteBright => "[37;1m",
            },
            self.effect
                .iter()
                .map(|effect| ESC.to_owned()
                    + match effect {
                        Effect::Normal => "[0m",
                        Effect::Bold => "[1m",
                        Effect::Dim => "[2m",
                        Effect::Underline => "[4m",
                        Effect::Blink => "[5m",
                        Effect::Reversed => "[7m",
                        Effect::Hidden => "[8m",
                    })
                .collect::<String>()
        )
    }
}

pub mod cursor {
    use super::*;
    impl Default for Cursor {
        fn default() -> Self {
            new()
        }
    }
    pub fn new() -> Cursor {
        Cursor {
            written: false,
            position: Coordinate(0, 0),
            text: Text {
                color: Color::White,
                effect: vec![Effect::Normal],
            },
            background: Background(Color::Black),
        }
    }

    pub enum ClearCommands {
        ScreenCursorEnd,
        ScreenCursorBeginning,
        Screen,
        LineCursorEnd,
        LineCursorBeginning,
        Line,
        Reset,
    }
    pub enum RelativeCursorCommands {
        Up(i32),
        Down(i32),
        Right(i32),
        Left(i32),
        NextLine(i32),
        PrevLine(i32),
        SetColumn(i32),
    }
    pub enum AbsoluteCursorCommands {
        SetPosition([i32; 2]),
    }
    impl super::Ansi for RelativeCursorCommands {
        fn ansi_code(&self) -> String {
            format!(
                "{}{}",
                String::from(ESC),
                match self {
                    RelativeCursorCommands::Up(number) => format!("[{}A", number),
                    RelativeCursorCommands::Down(number) => format!("[{}B", number),
                    RelativeCursorCommands::Right(number) => format!("[{}C", number),
                    RelativeCursorCommands::Left(number) => format!("[{}D", number),
                    RelativeCursorCommands::NextLine(number) => format!("[{}E", number),
                    RelativeCursorCommands::PrevLine(number) => format!("[{}F", number),
                    RelativeCursorCommands::SetColumn(number) => format!("[{}G", number),
                }
            )
        }
    }
    impl super::Ansi for AbsoluteCursorCommands {
        fn ansi_code(&self) -> String {
            format!(
                "{}{}",
                String::from(ESC),
                match self {
                    AbsoluteCursorCommands::SetPosition(numbers) => {
                        format!("[{};{}H", numbers[0], numbers[1])
                    }
                }
            )
        }
    }
    impl super::Ansi for ClearCommands {
        fn ansi_code(&self) -> String {
            String::from(ESC)
                + match self {
                    ClearCommands::ScreenCursorEnd => "[0J",
                    ClearCommands::ScreenCursorBeginning => "[1J",
                    ClearCommands::Screen => "[2J",
                    ClearCommands::LineCursorEnd => "[0K",
                    ClearCommands::LineCursorBeginning => "[1K",
                    ClearCommands::Line => "[2K",
                    ClearCommands::Reset => "[0m",
                }
        }
    }
    impl super::Ansi for Coordinate {
        fn ansi_code(&self) -> String {
            AbsoluteCursorCommands::SetPosition([self.0, self.1]).ansi_code()
        }
    }

    impl Cursor {
        pub fn set_text(&mut self, text: &Text) {
            self.text = text.clone();
            print!("{}", self.text.ansi_code());
        }
        pub fn set_position(&mut self, destination: &Coordinate) {
            self.position = *destination;
            print!("{}", self.position.ansi_code());
        }
        pub fn get_position(&mut self) -> &Coordinate {
            &self.position
        }
        pub fn set_background(&mut self, background: &Background) {
            self.background = background.clone();
            print!("{}", self.background.ansi_code());
        }
        fn write(&mut self) {
            if self.written == false {
                print!("{}", ClearCommands::Screen.ansi_code());
                self.written = true;
            }
            // print!(
            //     "{}{}{}",
            //     self.background.ansi_code(),
            //     self.text.ansi_code(),
            //     self.position.ansi_code()
            // );
        }
        pub fn draw(
            &mut self,
            startposition: Coordinate,
            height: u32,
            width: u32,
            drawing: fn(origin: [i32; 2], height: u32, width: u32),
        ) {
            self.write();
            drawing(startposition.as_array(), height, width);
        }
    }
}

pub mod tui {
    use super::*;
    use std::process::{Command, Stdio};
    #[derive(Clone)]
    pub struct Tui {
        name: String,
        size: Dimension,
        cursor: Cursor,
    }

    pub fn new() -> Tui {
        Tui {
            name: "Window0".to_string(),
            size: get_size(),
            cursor: cursor::new(),
        }
    }

    pub fn new_named(name: String) -> Tui {
        Tui {
            name: name,
            size: get_size(),
            cursor: cursor::new(),
        }
    }
    pub enum S {
        Height,
        Width,
    }

    impl Tui {
        pub fn cursor(&mut self) -> &mut Cursor {
            &mut self.cursor
        }

        ///Rounds percent input to nearest number of spaces or rows
        ///
        ///(Input as percent and not ratio; ie. 50.0 == 50%)
        ///
        /// ```
        ///let mut tui = terminal::tui::new();
        ///tui.get_size();
        ///let mut test = new();
        ///test.set_text(&terminal::Text {
        ///    color: Color::BlueBright,
        ///    effect: vec![terminal::Effect::Reversed, terminal::Effect::Bold],
        ///});
        ///test.draw(
        ///    Coordinate(3, 3),
        ///tui.percent(50.0,S::Height),
        ///tui.percent(50.0,S::Width),
        ///    shapes::rectangle,
        ///);
        ///```
        pub fn percent(&mut self, percentage: f64, s: S) -> u32 {
            let output = match s {
                S::Height => ((Into::<f64>::into(self.size.height) * (percentage / 100.)).round()) as u32,
                S::Width => (((Into::<f64>::into(self.size.width) * (percentage / 100.))*0.5).round()) as u32,
            };
            println!("Percentage output:{:?}", &self.size);
            println!("Percentage output:{}", &output);
            output
        }

        pub fn get_size(&mut self) {
            self.size = get_size();
        }

    }

    fn get_size() -> Dimension {
        let terminalsizer = Command::new(r#"stty"#)
            .arg("size")
            .stdin(Stdio::inherit())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed5555");

        let nums: Vec<u32> =
            if let Ok(a) = String::from_utf8(terminalsizer.wait_with_output().unwrap().stdout) {
                print!("{}", &a);
                a
            } else {
                panic!()
            }
            .trim_end()
            .split(" ")
            .map(|num| {
                num.parse::<u32>()
                    .expect(r#"Unable to parse screen size from command "stty size""#)
            })
            .collect();
        Dimension {
            height: nums[0].clone(),
            width: nums[1].clone(),
        }
    }
}
