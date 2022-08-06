use std::{collections::HashMap, time::Duration};

use chrono::{Local, TimeZone};
use iced::{
    canvas::Cache, executor, Application, Column, Command, Container, Element, Length, Settings,
    Subscription, Text,
};

use plotters::prelude::*;
use plotters_iced::{Chart, ChartWidget};
mod power;

struct State {
    chart: BatteryChart,
}

#[derive(Debug)]
enum Message {
    Tick,
}

impl Application for State {
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
        match message {
            Message::Tick => self.chart.update(),
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let content = Column::new()
            .push(Text::new("Iced test chart").size(20))
            .push(self.chart.view());
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}

struct BatteryChart {
    counter: i32,
    should_update: bool,
    all_time_chart: AllTimeChart,
}

impl Default for BatteryChart {
    fn default() -> Self {
        Self {
            counter: 0,
            should_update: true,
            all_time_chart: AllTimeChart::default(),
        }
    }
}

impl BatteryChart {
    fn update(&mut self) {
        self.counter += 1;
    }

    fn view(&mut self) -> Element<Message> {
        if self.should_update {
            self.all_time_chart = AllTimeChart::new(power::create_chart());
            self.should_update = false;
        }
        Column::new()
            .push(Text::new(format!("{}", self.counter)))
            .push(self.all_time_chart.view())
            .into()
    }
}

#[derive(Default)]
struct AllTimeChart {
    cache: Cache,
    db: HashMap<i64, (f64, String)>,
}

impl AllTimeChart {
    fn new(db: HashMap<i64, (f64, String)>) -> Self {
        Self {
            cache: Cache::new(),
            db,
        }
    }

    fn view(&mut self) -> Element<Message> {
        Container::new(
            Column::new()
                .width(Length::Fill)
                .height(Length::Fill)
                .push(ChartWidget::new(self).height(Length::Fill)),
        )
        .into()
    }
}

impl Chart<Message> for AllTimeChart {
    fn build_chart<DB: plotters_iced::DrawingBackend>(
        &self,
        mut builder: plotters_iced::ChartBuilder<DB>,
    ) {
        let min = self.db.keys().min().unwrap();
        let max = self.db.keys().max().unwrap();
        let duration = max - min;

        let min_date = Local.timestamp(*min, 0);
        let max_date = Local.timestamp(*max, 0);

        let mut chart = builder
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption("Charge/Discharge Graph", ("sans-serif", 40))
            .build_cartesian_2d(min_date..max_date, 0..100)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(self.db.iter().map(|(k, v)| {
                Circle::new(
                    (Local.timestamp(*k, 0), v.0 as i32),
                    5,
                    if v.1 == "charging" { &BLUE } else { &RED },
                )
            }))
            .unwrap();
    }
}

fn main() {
    State::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
    .unwrap();
}
