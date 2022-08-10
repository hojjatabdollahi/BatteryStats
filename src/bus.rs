use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read},
    path::Path,
};

use bincode::serialize_into;
use log::info;
use serde::{Deserialize, Serialize};
use zbus::{blocking::Connection, dbus_proxy, zvariant::OwnedObjectPath};

#[dbus_proxy(
    interface = "org.freedesktop.UPower",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower"
)]
trait UPower {
    fn EnumerateDevices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatHistroy {
    pub data: Vec<(u32, f32, u32)>,
}

pub fn save_to_file() -> Result<(), Box<dyn Error>> {
    let connection = Connection::system()?;
    let proxy = UPowerProxyBlocking::new(&connection)?;
    let reply = proxy.EnumerateDevices()?;
    for object_path in reply {
        info!("{:?}", object_path.to_string());
        let proxy: zbus::blocking::Proxy = zbus::blocking::ProxyBuilder::new_bare(&connection)
            .path(object_path)?
            .interface("org.freedesktop.UPower.Device")?
            .destination("org.freedesktop.UPower")?
            .build()?;
        let m = proxy.call_method("GetHistory", &("charge", 0u32, 100u32))?;
        let m: Vec<(u32, f32, u32)> = m.body()?;
        let n = BatHistroy { data: m };
        // let encoded: Vec<u8> = bincode::serialize(&n).unwrap();
        let mut output = BufWriter::new(File::create("data.dat").unwrap());
        serialize_into(&mut output, &n).unwrap();
    }
    // drop(output);
    Ok(())

    // let input = BufReader::new(File::open("data.dat").unwrap());
    // let data: Vec<BatHistroy> = deserialize_from(input).unwrap();
}

pub fn read_from_file(path: impl AsRef<Path>) -> Result<BatHistroy, Box<dyn Error>> {
    let mut input = BufReader::new(File::open(path)?);
    let mut buf_reader = vec![];
    let size = input.read_to_end(&mut buf_reader)?;
    info!("Read {size:?} bytes");
    let data: BatHistroy = bincode::deserialize(&buf_reader)?;
    info!("{data:?}");
    Ok(data)
}
