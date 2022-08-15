use log::{error, info};
use std::{collections::HashMap, error::Error, fs, io::Read};

use chrono::{Local, TimeZone};

use crate::{bus_client::BusClient, storage};

pub struct UPower {
    db: HashMap<String, HashMap<i64, (f64, String)>>,
}

impl UPower {
    pub fn init() -> Result<(), Box<dyn Error>> {
        Self::save()?;
        Self::load_saved_files();
        Ok(())
    }

    fn load_saved_files() {}

    fn save() -> Result<(), Box<dyn Error>> {
        let bc = BusClient::new()?;
        for device_path in bc.devices()? {
            let props = bc.get_device_properties(device_path.as_ref())?;
            let mut hists = vec![];
            if props.has_history {
                hists = bc.get_history(device_path.as_ref())?;
            }
            storage::save_to_file(device_path.to_string(), hists, props)?;
        }
        Ok(())
    }
}

pub fn create_chart() -> Vec<(i64, (f64, String))> {
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
                    let splitted_line = line.split('\t').collect::<Vec<&str>>();
                    if splitted_line.len() == 3 {
                        let timestamp: i64 = splitted_line[0].parse().unwrap();
                        let percentage: f64 = splitted_line[1].parse().unwrap();
                        db.insert(timestamp, (percentage, splitted_line[2].to_string()));
                    } else {
                        error!("Line is not formatted correctly! \n line: {}", line);
                    }
                })
                .for_each(drop);
        })
        .for_each(drop);

    info!(
        "Last sample {}",
        Local.timestamp(*db.keys().max().unwrap(), 0)
    );

    let mut dbvec: Vec<(i64, (f64, String))> = db.into_iter().collect();

    dbvec.sort_by(|a, b| a.0.cmp(&b.0));
    dbvec
}
