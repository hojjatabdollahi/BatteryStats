use std::error::Error;

use zbus::{
    blocking::Connection,
    dbus_proxy,
    zvariant::{ObjectPath, OwnedObjectPath},
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

pub struct BusClient<'a> {
    connection: Connection,
    proxy: UPowerProxyBlocking<'a>,
}

impl BusClient<'_> {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let connection = Connection::system()?;
        let proxy = UPowerProxyBlocking::new(&connection)?;
        Ok(Self { connection, proxy })
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

    pub fn devices(&self) -> Result<Vec<OwnedObjectPath>, Box<dyn Error>> {
        Ok(self.proxy.enumerate_devices().unwrap())
    }

    pub fn get_history(
        &self,
        object_path: ObjectPath,
    ) -> Result<Vec<(u32, f32, u32)>, Box<dyn Error>> {
        let proxy: zbus::blocking::Proxy = zbus::blocking::ProxyBuilder::new_bare(&self.connection)
            .path(&object_path)?
            .interface("org.freedesktop.UPower.Device")?
            .destination("org.freedesktop.UPower")?
            .build()?;
        let mut m: Vec<(u32, f32, u32)> = vec![];
        let m1 = proxy.call_method("GetHistory", &("charge", 0u32, 100u32))?;
        let m2: Vec<(u32, f32, u32)> = m1.body()?;
        Ok(m2)
    }
}
