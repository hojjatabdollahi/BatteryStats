use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read},
};

use bincode::serialize_into;
use byteorder::LE;
use chrono::Local;
use glob::glob;
use zbus::zvariant::EncodingContext;

use crate::model::{BatHistory, DataLayout, UPowerProperties};

pub fn save_to_file(
    object_path: String,
    hists: Vec<(u32, f32, u32)>,
    props: UPowerProperties,
) -> Result<(), Box<dyn Error>> {
    let ctx = EncodingContext::<LE>::new_dbus(0);
    let today = Local::now().to_string();
    let path_name = object_path.split('/').last().unwrap();
    let props_bytes = zbus::zvariant::to_bytes(ctx, &props)?;
    let dl = DataLayout {
        p: props_bytes,
        d: hists,
    };
    let mut output = BufWriter::new(File::create(format!(
        "{}-{}-{}.dat",
        today, path_name, props.model
    ))?);
    serialize_into(&mut output, &dl)?;
    Ok(())
}

pub fn read_from_file() -> Result<BatHistory, Box<dyn Error>> {
    let ctx = EncodingContext::<LE>::new_dbus(0);
    let mut data2 = BatHistory {
        properties: Default::default(),
        data: Default::default(),
    };
    for f in glob("./*.dat")? {
        let mut input = BufReader::new(File::open(f.unwrap())?);
        let mut buf_reader = vec![];
        let _size = input.read_to_end(&mut buf_reader)?;
        let data: DataLayout = bincode::deserialize(&buf_reader)?;
        let xprop: UPowerProperties = zbus::zvariant::from_slice(&data.p, ctx).unwrap();
        data2 = BatHistory {
            properties: xprop,
            data: data.d,
        };
    }
    Ok(data2)
}
