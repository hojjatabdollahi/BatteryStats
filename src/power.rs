use std::{collections::HashMap, fs, io::Read};

use chrono::{Local, TimeZone};
use plotters::prelude::*;

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

    println!(
        "Hello, world! {}",
        Local.timestamp(*db.keys().max().unwrap(), 0)
    );

    let mut dbvec: Vec<(i64, (f64, String))> = db.into_iter().collect();

    dbvec.sort_by(|a, b| a.0.cmp(&b.0));
    dbvec
}
