use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use bincode::serialize_into;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use zbus::{
    blocking::Connection,
    dbus_proxy,
    zvariant::{DeserializeDict, ObjectPath, OwnedObjectPath, SerializeDict, Type},
};

use crate::model::UPowerProperties;

#[dbus_proxy(
    interface = "org.freedesktop.UPower",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower"
)]
trait UPower {
    fn enumerate_devices(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}

#[derive(Debug, Default, SerializeDict, DeserializeDict, PartialEq, Type)]
pub struct UPowerTest {
    pub has_history: bool,
    pub has_statistics: bool,
    pub is_present: bool,
    pub is_rechargeable: bool,
    pub online: String,
    pub power_supply: u32,
    pub capacity: f32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BatHistroy2 {
    pub properties: UPowerTest,
    pub data: Vec<(u32, f32, u32)>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BatHistroy {
    pub properties: UPowerProperties,
    pub data: Vec<(u32, f32, u32)>,
}

pub struct BusClient {
    connection: Connection,
}

impl BusClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            connection: Connection::system()?,
        })
    }

    pub fn get_device_properties(
        &self,
        path: ObjectPath,
    ) -> Result<UPowerProperties, Box<dyn Error>> {
        let proxy: zbus::blocking::Proxy = zbus::blocking::ProxyBuilder::new_bare(&self.connection)
            .path(path)?
            .interface("org.freedesktop.DBus.Properties")?
            .destination("org.freedesktop.UPower")?
            .build()?;
        let m = proxy.call_method("GetAll", &("org.freedesktop.UPower.Device",))?;
        let n = m.body::<UPowerProperties>()?;

        Ok(n)
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let proxy = UPowerProxyBlocking::new(&self.connection)?;
        let reply = proxy.enumerate_devices()?;
        for object_path in reply {
            // info!("{:?}", object_path.to_string());
            let props = self.get_device_properties(object_path.as_ref())?;
            // info!("{props:#?}");
            let proxy: zbus::blocking::Proxy =
                zbus::blocking::ProxyBuilder::new_bare(&self.connection)
                    .path(object_path)?
                    .interface("org.freedesktop.UPower.Device")?
                    .destination("org.freedesktop.UPower")?
                    .build()?;
            let m = proxy.call_method("GetHistory", &("charge", 0u32, 100u32))?;
            let m: Vec<(u32, f32, u32)> = m.body()?;

            // let n = BatHistroy {
            //     properties: props,
            //     data: m,
            // };

            let n2 = BatHistroy2 {
                properties: Default::default(),
                data: m,
            };

            let xx = json!(n2);
            info!("{xx:#?}");
            // let encoded: Vec<u8> = bincode::serialize(&n).unwrap();
            let mut output = BufWriter::new(File::create("data.dat").unwrap());
            // output.write_all(&encoded).unwrap();

            serialize_into(&mut output, &n2).unwrap();
        }
        // drop(output);
        Ok(())

        // let input = BufReader::new(File::open("data.dat").unwrap());
        // let data: Vec<BatHistroy> = deserialize_from(input).unwrap();
    }

    pub fn read_from_file(&self, path: impl AsRef<Path>) -> Result<BatHistroy2, Box<dyn Error>> {
        let mut input = BufReader::new(File::open(path)?);
        let mut buf_reader = vec![];
        let size = input.read_to_end(&mut buf_reader)?;
        // info!("Read {size:?} bytes");
        // let data: BatHistroy = bincode::deserialize(&buf_reader)?;
        let data: BatHistroy2 = bincode::deserialize(&buf_reader)?;
        // info!("properties:\n{:?}", data.properties);
        // info!("history:\n{:?}", data.data);
        Ok(data)
    }
}
