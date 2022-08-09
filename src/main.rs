use std::error::Error;

use chrono::{Date, Local, TimeZone};
use clap::Parser;
use iced::{
    canvas::Cache, executor, Application, Button, Column, Command, Container, Element, Length, Row,
    Settings, Space, Text,
};

use plotters::prelude::*;
use plotters_iced::{Chart, ChartWidget};

use log::{info, trace};
use zbus::blocking::Connection;

mod power;

struct BatteryStatApp {
    chart: BatteryChartComponents,
    today: Option<String>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    NextDay,
    PreviousDay,
    // DayChanged(chrono::Date<Local>),
}

impl Application for BatteryStatApp {
    type Executor = executor::Default;

    type Message = self::Message;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                chart: Default::default(),
                today: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Battery Stats".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        // let msg2 = self.chart.update(message);
        // if let Message::DayChanged(newDay) = msg2 {
        //     self.today = Some(newDay.format("%y/%m/%d").to_string());
        // }
        self.chart.update(message);
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let mut content = Column::new()
            .spacing(20)
            .align_items(iced::Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill);
        if self.today.is_some() {
            content = content.push(Text::new(self.today.clone().unwrap()));
        }
        content = content.push(self.chart.view());

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
}

impl Default for BatteryChartComponents {
    fn default() -> Self {
        Self {
            should_update: true,
            initialized: false,
            battery_chart: Default::default(),
            next_state: Default::default(),
            prev_state: Default::default(),
        }
    }
}

impl BatteryChartComponents {
    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::NextDay => {
                trace!("update::next");
                let newDay = self.battery_chart.next();
                self.should_update = true;
                // Message::DayChanged(newDay);
                Command::none()
            }
            Message::PreviousDay => {
                trace!("update::prev");
                let newDay = self.battery_chart.prev();
                self.should_update = true;
                // Message::DayChanged(newDay);
                Command::none()
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

        Column::new()
            .push(self.battery_chart.view())
            .push(
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
            )
            .into()
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
        let start_day = db.last().unwrap().0;
        let start_day = Local.timestamp(start_day, 0);
        let start_day = start_day.date().and_hms(0, 0, 0);
        let start_day = start_day.timestamp();
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
                .height(Length::Fill)
                .push(ChartWidget::new(self).height(Length::Fill))
                .height(Length::Fill),
        )
        .height(Length::Fill)
        .into()
    }

    fn next(&mut self) -> Date<Local> {
        self.start_day += 24 * 60 * 60;
        if self.start_day > self.db.last().unwrap().0 {
            self.start_day = self.db.last().unwrap().0 - 24 * 60 * 60;
        }
        self.cache.clear();
        trace!("Next: {}", self.start_day);
        Local.timestamp(self.start_day, 0).date()
    }

    fn prev(&mut self) -> Date<Local> {
        self.start_day -= 24 * 60 * 60;
        if self.start_day < self.db[0].0 {
            self.start_day = self.db[0].0;
        }
        self.cache.clear();
        trace!("Prev: {}", self.start_day);
        Local.timestamp(self.start_day, 0).date()
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
            .margin(15)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption("Charge/Discharge Graph", ("sans-serif", 40))
            .build_cartesian_2d(min_date..max_date, 0..100)
            .unwrap();

        chart
            .configure_mesh()
            .x_label_formatter(&|x| format!("{}", x.time().format("%H:%M")))
            .draw()
            .unwrap();

        chart
            .draw_series(
                self.db
                    .iter()
                    .filter(|x| x.0 > self.start_day && x.0 < end_day)
                    .map(|(k, v)| {
                        Circle::new(
                            (Local.timestamp(*k, 0), v.0 as i32),
                            3,
                            if v.1 == "charging" {
                                ShapeStyle::from(&BLUE).filled()
                            } else {
                                ShapeStyle::from(&RED).filled()
                            },
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

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about=None)]
struct Args {
    // Use this to just store the latest stats (hourly)
    #[clap(short, long, value_parser)]
    no_gui: bool,
}

// #[dbus_proxy(
//     interface = "org.freedesktop.UPower.Device",
//     default_service = "org.freedesktop.UPower",
//     default_path = "/org/freedesktop/UPower/devices/mouse_hidpp_battery_0"
// )]
// trait UPower {
//     fn GetHistory(
//         &self,
//         r#type: &str,
//         timespan: u32,
//         resolution: u32,
//     ) -> zbus::Result<Vec<(u32, f32, u32)>>;
// }

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter(Some("battery"), log::LevelFilter::Trace)
        .init();

    let args = Args::parse();
    if args.no_gui {
        // let connection = Connection::system()?;
        // let proxy = UPowerProxyBlocking::new(&connection)?;
        // let reply = proxy.GetHistory("charge", 0, 100);
        // info!("{reply:?}");

        let connection = Connection::system()?;
        let proxy: zbus::blocking::Proxy = zbus::blocking::ProxyBuilder::new_bare(&connection)
            .path("/org/freedesktop/UPower/devices/mouse_hidpp_battery_0")?
            .interface("org.freedesktop.UPower.Device")?
            .destination("org.freedesktop.UPower")?
            .build()?;
        let m = proxy.call_method("GetHistory", &("charge", 0u32, 100u32))?;
        let m: Vec<(u32, f32, u32)> = m.body()?;
        info!("{m:?}");

        return Ok(());
    }

    BatteryStatApp::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
    .unwrap();
    Ok(())
}
