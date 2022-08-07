use chrono::{Local, TimeZone};
use iced::{
    canvas::Cache, executor, Application, Button, Column, Command, Container, Element, Length, Row,
    Scrollable, Settings, Space, Text,
};

use plotters::prelude::*;
use plotters_iced::{Chart, ChartWidget};
mod power;

struct BatteryStatApp {
    chart: BatteryChartComponents,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    NextDay,
    PreviousDay,
}

impl Application for BatteryStatApp {
    type Executor = executor::Default;

    type Message = self::Message;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                chart: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Battery Stats".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        self.chart.update(message);
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let content = Column::new()
            .spacing(20)
            .align_items(iced::Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(self.chart.view());
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()
            .center_y()
            .into()
    }
}

struct BatteryChartComponents {
    should_update: bool,
    initialized: bool,
    battery_chart: BatteryChart,
    next_state: iced::button::State,
    prev_state: iced::button::State,
    scroll_state: iced::scrollable::State,
}

impl Default for BatteryChartComponents {
    fn default() -> Self {
        Self {
            should_update: true,
            initialized: false,
            battery_chart: Default::default(),
            next_state: Default::default(),
            prev_state: Default::default(),
            scroll_state: Default::default(),
        }
    }
}

impl BatteryChartComponents {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::NextDay => {
                println!("update::next");
                self.battery_chart.next();
                self.should_update = true;
            }
            Message::PreviousDay => {
                println!("update::prev");
                self.battery_chart.prev();
                self.should_update = true;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        if !self.initialized {
            self.battery_chart = BatteryChart::new(power::create_chart());
            self.initialized = true;
        }

        if self.should_update {
            self.should_update = false;
        }
        let mut scroll = Scrollable::new(&mut self.scroll_state)
            .width(Length::Fill)
            .height(Length::Fill);

        let col = Column::new().push(self.battery_chart.view()).push(
            Row::new()
                .push(
                    Button::new(
                        &mut self.prev_state,
                        Text::new("Previous")
                            .width(Length::Fill)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                    )
                    .width(Length::Units(100))
                    .on_press(Message::PreviousDay),
                )
                .push(Space::new(Length::Fill, Length::Fill))
                .push(
                    Button::new(
                        &mut self.next_state,
                        Text::new("Next")
                            .width(Length::Fill)
                            .horizontal_alignment(iced::alignment::Horizontal::Center),
                    )
                    .width(Length::Units(100))
                    .on_press(Message::NextDay),
                )
                .height(Length::Units(40)),
        );
        scroll = scroll.push(col);
        scroll.into()
    }
}

#[derive(Default)]
struct BatteryChart {
    cache: Cache,
    db: Vec<(i64, (f64, String))>,
    start_day: i64,
}

impl BatteryChart {
    fn new(db: Vec<(i64, (f64, String))>) -> Self {
        let start_day = db[0].0;
        Self {
            cache: Cache::new(),
            db,
            start_day,
        }
    }

    fn view(&mut self) -> Element<Message> {
        Container::new(
            Column::new()
                .width(Length::Fill)
                .height(Length::Shrink)
                .push(ChartWidget::new(self).height(Length::Fill)),
        )
        .height(Length::Units(500))
        .into()
    }

    fn next(&mut self) {
        self.start_day += 24 * 60 * 60;
        if self.start_day > self.db.last().unwrap().0 {
            self.start_day = self.db.last().unwrap().0 - 24 * 60 * 60;
        }
        self.cache.clear();
        println!("Next: {}", self.start_day);
    }

    fn prev(&mut self) {
        self.start_day -= 24 * 60 * 60;
        if self.start_day < self.db[0].0 {
            self.start_day = self.db[0].0;
        }
        self.cache.clear();
        println!("Prev: {}", self.start_day);
    }
}

impl Chart<Message> for BatteryChart {
    fn build_chart<DB: plotters_iced::DrawingBackend>(
        &self,
        mut builder: plotters_iced::ChartBuilder<DB>,
    ) {
        // let min = self.db.keys().min().unwrap();
        let max = self.db.last().unwrap().0;
        let mut end_day = self.start_day + 24 * 60 * 60;
        if end_day > max {
            end_day = max;
        }
        // let duration = max - min;

        let min_date = Local.timestamp(self.start_day, 0);
        let max_date = Local.timestamp(end_day, 0);

        let mut chart = builder
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption("Charge/Discharge Graph", ("sans-serif", 40))
            .build_cartesian_2d(min_date..max_date, 0..100)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(
                self.db
                    .iter()
                    .filter(|x| x.0 > self.start_day && x.0 < end_day)
                    .map(|(k, v)| {
                        Circle::new(
                            (Local.timestamp(*k, 0), v.0 as i32),
                            5,
                            if v.1 == "charging" { &BLUE } else { &RED },
                        )
                    }),
            )
            .unwrap();
    }

    fn draw_chart<DB: DrawingBackend>(&self, root: DrawingArea<DB, plotters::coord::Shift>) {
        let builder = ChartBuilder::on(&root);
        self.build_chart(builder);
    }

    fn draw<F: Fn(&mut iced::canvas::Frame)>(
        &self,
        size: iced::Size,
        f: F,
    ) -> iced::canvas::Geometry {
        self.cache.draw(size, f)
    }
}

fn main() {
    BatteryStatApp::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
    .unwrap();
}
