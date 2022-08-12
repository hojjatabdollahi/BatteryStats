use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read},
    path::Path,
};

use bincode::serialize_into;
use byteorder::LE;
use log::info;
use serde::{Deserialize, Serialize};
use zbus::{
    blocking::Connection,
    dbus_proxy,
    names::InterfaceName,
    zvariant::{
        from_slice, to_bytes, EncodingContext, ObjectPath, OwnedObjectPath, OwnedValue, Value,
    },
};

use crate::model::{self, UPowerProperties};

#[dbus_proxy(
    interface = "org.freedesktop.UPower",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower"
)]
trait UPower {
    fn enumerateDevices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "BatteryLevel")]
    pub battery_level: u32,
    #[serde(rename = "Capacity")]
    pub capacity: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatHistroy {
    pub properties: HashMap<String, OwnedValue>,
    pub data: Vec<(u32, f32, u32)>,
}

pub fn get_device_properties(
    connection: &Connection,
    path: ObjectPath,
) -> Result<HashMap<String, zbus::zvariant::OwnedValue>, Box<dyn Error>> {
    let proxy: zbus::blocking::Proxy = zbus::blocking::ProxyBuilder::new_bare(&connection)
        .path(path)?
        .interface("org.freedesktop.DBus.Properties")?
        .destination("org.freedesktop.UPower")?
        .build()?;
    let m = proxy.call_method("GetAll", &("org.freedesktop.UPower.Device",))?;
    let x = m.body_as_bytes().unwrap();

    let ctx = EncodingContext::<LE>::new_dbus(0);

    let decoded: model::UPowerProperties = from_slice(x, ctx).unwrap();

    info!("Decoded:");
    info!("{decoded:?}");

    info!("\n================\n");

    // let n: HashMap<String, zbus::zvariant::OwnedValue> = m.body()?;
    // let n = m.body::<UPowerProperties>()?;

    info!("{n:?}");
    Ok(HashMap::new())
}

pub fn get_device_properties2(
    connection: &Connection,
    path: ObjectPath,
) -> Result<(), Box<dyn Error>> {
    // let proxy: zbus::blocking::Proxy = zbus::blocking::ProxyBuilder::new_bare(&connection)
    //     .path(path)?
    //     .interface("org.freedesktop.DBus.Properties")?
    //     .destination("org.freedesktop.UPower")?
    //     .build()?;
    info!("The path is: {path:?}");
    let p = zbus::blocking::fdo::PropertiesProxy::builder(&connection)
        .path(path)?
        .destination("org.freedesktop.UPower")?
        .build()?;
    let o = p.get_all(InterfaceName::from_str_unchecked(
        "org.freedesktop.UPower.Device",
    ))?;

    info!("\n================\n");
    info!("{o:#?}");

    // let ctx = EncodingContext::<LE>::new_dbus(0);
    //
    // // let encoded = to_bytes(ctx, &o).unwrap();
    //
    // let decoded: model::UPowerProperties = from_slice(o, ctx).unwrap();

    // info!("Decoded:");
    // info!("{decoded:?}");

    // let m = proxy.call_method("GetAll", &("org.freedesktop.UPower.Device",))?;

    // let n: HashMap<String, zbus::zvariant::OwnedValue> = m.body()?;
    // let n: zbus::zvariant::Structure = m.body()?;
    // let n: HashMap<String, zbus::zvariant::OwnedValue> = m.body()?;

    info!("\n================\n");

    // for field in n {
    //     info!("{field:?}");
    // }

    Ok(())
}

pub fn save_to_file() -> Result<(), Box<dyn Error>> {
    let connection = Connection::system()?;
    let proxy = UPowerProxyBlocking::new(&connection)?;
    let reply = proxy.enumerateDevices()?;
    for object_path in reply {
        info!("{:?}", object_path.to_string());
        let props = get_device_properties(&connection, object_path.as_ref())?;
        get_device_properties2(&connection, object_path.as_ref())?;
        let proxy: zbus::blocking::Proxy = zbus::blocking::ProxyBuilder::new_bare(&connection)
            .path(object_path)?
            .interface("org.freedesktop.UPower.Device")?
            .destination("org.freedesktop.UPower")?
            .build()?;
        let m = proxy.call_method("GetHistory", &("charge", 0u32, 100u32))?;
        let m: Vec<(u32, f32, u32)> = m.body()?;

        let n = BatHistroy {
            properties: props,
            data: m,
        };
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
    info!("properties:\n{:?}", data.properties);
    info!("history:\n{:?}", data.data);
    Ok(data)
}
