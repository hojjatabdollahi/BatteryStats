use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use bincode::serialize_into;
use byteorder::LE;
use chrono::Local;
use glob::glob;
use log::info;
use serde::{Deserialize, Serialize};
use zbus::{
    blocking::Connection,
    dbus_proxy,
    zvariant::{
        DeserializeDict, EncodingContext, ObjectPath, OwnedObjectPath, SerializeDict, Type,
    },
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
pub struct BatHistory {
    pub properties: UPowerProperties,
    pub data: Vec<(u32, f32, u32)>,
}

#[derive(Serialize, Deserialize)]
struct DataLayout {
    p: Vec<u8>,
    d: Vec<(u32, f32, u32)>,
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

        let ctx = EncodingContext::<LE>::new_dbus(0);

        for object_path in reply {
            let props = self.get_device_properties(object_path.as_ref())?;
            let proxy: zbus::blocking::Proxy =
                zbus::blocking::ProxyBuilder::new_bare(&self.connection)
                    .path(object_path)?
                    .interface("org.freedesktop.UPower.Device")?
                    .destination("org.freedesktop.UPower")?
                    .build()?;
            let m = proxy.call_method("GetHistory", &("charge", 0u32, 100u32))?;
            let m: Vec<(u32, f32, u32)> = m.body()?;
            // let n = BatHistory {
            //     properties: props,
            //     data: m,
            // };
            // let mut output = BufWriter::new(File::create("data.dat").unwrap());
            // serialize_into(&mut output, &n).unwrap();

            let datalayout = DataLayout {
                p: zbus::zvariant::to_bytes(ctx, &props)?,
                d: m,
            };

            let today = Local::now().to_string();
            let mut output =
                BufWriter::new(File::create(format!("{}-{}.dat", today, props.model)).unwrap());
            serialize_into(&mut output, &datalayout).unwrap();
        }

        // let s2 = S2 {
        //     f1: "3".to_string(),
        // };
        // println!("{s2:#?}");
        // let s3 = zbus::zvariant::to_bytes(ctx, &s2).unwrap();
        // println!("{:#?}", s3.len());
        // let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        // encoder.write(&s3).unwrap();
        // let s4 = encoder.finish().unwrap();
        // let s5 = bincode::serialize(&s4).unwrap();
        // let decode: Vec<u8> = bincode::deserialize(&s5).unwrap();
        // println!("decode: {:#?}", decode.len());
        // let mut gz = GzDecoder::new(&decode[..]);
        // let mut decodeddecoded: Vec<u8> = vec![];
        // let size = gz.read_to_end(&mut decodeddecoded).unwrap();
        // println!("size: {}", size);
        // println!("s3: {}, decodeddecoded: {}", s3.len(), decodeddecoded.len());
        // let x: S2 = zbus::zvariant::from_slice(&decodeddecoded, ctx).unwrap();
        // println!("{x:#?}");

        // drop(output);
        Ok(())

        // let input = BufReader::new(File::open("data.dat").unwrap());
        // let data: Vec<BatHistory> = deserialize_from(input).unwrap();
    }

    pub fn read_from_file(&self, path: impl AsRef<Path>) -> Result<BatHistory, Box<dyn Error>> {
        let ctx = EncodingContext::<LE>::new_dbus(0);
        let mut data2 = BatHistory {
            properties: Default::default(),
            data: Default::default(),
        };
        for f in glob("./*.dat")? {
            let mut input = BufReader::new(File::open(f.unwrap())?);
            let mut buf_reader = vec![];
            let size = input.read_to_end(&mut buf_reader)?;
            // info!("Read {size:?} bytes");
            // let data: BatHistory = bincode::deserialize(&buf_reader)?;
            let data: DataLayout = bincode::deserialize(&buf_reader)?;
            let xprop: UPowerProperties = zbus::zvariant::from_slice(&data.p, ctx).unwrap();
            // info!("properties:\n{:?}", data.properties);
            // info!("history:\n{:?}", data.data);
            data2 = BatHistory {
                properties: xprop,
                data: data.d,
            };
        }
        Ok(data2)
    }
}
