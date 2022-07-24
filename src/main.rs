use std::collections::HashMap;
use std::fs;
use std::io::Read;

use chrono::offset::TimeZone;
use chrono::Local;
use iced::canvas::{Cursor, Frame, Geometry, Path, Program};
use iced::{button, Button, Color, Column, Rectangle, Sandbox, Settings, Text};

fn main() {
    println!("Hello, world! {}", Local.timestamp(1650519787, 0));
    let mut db: HashMap<i64, (f64, String)> = HashMap::new();

    ["1.dat", "2.dat", "3.dat", "4.dat", "5.dat"]
        .iter()
        .map(|fname| {
            let mut f = fs::File::open(format!("data/{}", fname)).expect("error opening file");
            let mut content = String::new();

            f.read_to_string(&mut content).unwrap();
            content
                .split('\n')
                .collect::<Vec<&str>>()
                .iter()
                .map(|line| {
                    // println!("line: {}", line);
                    let splitted_line = line.split('\t').collect::<Vec<&str>>();
                    if splitted_line.len() == 3 {
                        let timestamp: i64 = splitted_line[0].parse().unwrap();
                        let percentage: f64 = splitted_line[1].parse().unwrap();
                        db.insert(timestamp, (percentage, splitted_line[2].to_string()));
                    } else {
                        // eprintln!("not 3: {}", splitted_line.len());
                    }
                })
                .for_each(drop);
        })
        .for_each(drop);
    // println!("{:?}", db);
    println!("{:?}", db.keys().min());

    println!(
        "Hello, world! {}",
        Local.timestamp(*db.keys().min().unwrap(), 0)
    );
    println!(
        "Hello, world! {}",
        Local.timestamp(*db.keys().max().unwrap(), 0)
    );

    App::run(Settings::default()).unwrap();
    // let mut output = fs::OpenOptions::new()
    //     .create(true)
    //     .write(true)
    //     .open("output.csv")
    //     .unwrap();
    // for a in db {
    //     output
    //         .write(format!("{}, {}, {}\n", a.0, a.1 .0, a.1 .1).as_bytes())
    //         .unwrap();
    // }
}

// First, we define the data we need for drawing
#[derive(Debug)]
struct Circle {
    radius: f32,
}

// Then, we implement the `Program` trait
impl Program<()> for Circle {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        // We prepare a new `Frame`
        let mut frame = Frame::new(bounds.size());

        // We create a `Path` representing a simple circle
        let circle = Path::circle(frame.center(), self.radius);

        // And fill it with some color
        frame.fill(&circle, Color::BLACK);

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }
}

#[derive(Default)]
struct App {
    exit: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    Exit,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        App::default()
    }

    fn title(&self) -> String {
        "Battery stats".to_string()
    }

    fn update(&mut self, _message: Self::Message) {
        todo!()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        Column::new()
            .push(Text::new("hello"))
            .push(Button::new(&mut self.exit, Text::new("Exit")))
            .into()
    }
}

// Finally, we simply use our `Circle` to create the `Canvas`!
