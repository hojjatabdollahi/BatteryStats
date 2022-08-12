use std::collections::HashMap;

use byteorder::LE;
use log::error;
use serde::{Deserialize, Serialize};
use zbus::zvariant::{from_slice, to_bytes, EncodingContext, TypeDict};
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

#[derive(Type, DeserializeDict, SerializeDict, PartialEq, Debug)]
#[zvariant(signature = "dict")]
pub struct UPowerProperties {
    HasHistory: bool,
    #[zvariant(rename = "HasStatistics")]
    has_statistics: bool,
    #[zvariant(rename = "IsPresent")]
    is_present: bool,
    #[zvariant(rename = "IsRechargeable")]
    is_rechargeable: bool,
    #[zvariant(rename = "Online")]
    online: bool,
    #[zvariant(rename = "PowerSupply")]
    power_supply: bool,
    #[zvariant(rename = "Capacity")]
    capacity: f32,
    #[zvariant(rename = "Energy")]
    energy: f32,
    #[zvariant(rename = "EnergyEmpty")]
    energy_empty: f32,
    #[zvariant(rename = "EnergyFull")]
    energy_full: f32,
    #[zvariant(rename = "EnergyFullDesign")]
    energy_full_design: f32,
    #[zvariant(rename = "EnergyRate")]
    energy_rate: f32,
    #[zvariant(rename = "Luminosity")]
    luminosity: f32,
    #[zvariant(rename = "Percentage")]
    percentage: f32,
    #[zvariant(rename = "Temperature")]
    temperature: f32,
    #[zvariant(rename = "Voltage")]
    voltage: f32,
    #[zvariant(rename = "TimeToEmpty")]
    time_to_empty: i64,
    #[zvariant(rename = "TimeToFull")]
    time_to_full: i64,
    #[zvariant(rename = "IconName")]
    icon_name: String,
    #[zvariant(rename = "Model")]
    model: String,
    #[zvariant(rename = "NativePath")]
    native_path: String,
    #[zvariant(rename = "Serial")]
    serial: String,
    #[zvariant(rename = "Vendor")]
    vendor: String,
    #[zvariant(rename = "BatteryLevel")]
    battery_level: u32,
    #[zvariant(rename = "State")]
    state: u32,
    #[zvariant(rename = "Technology")]
    technology: u32,
    #[zvariant(rename = "Type")]
    r#type: u32,
    #[zvariant(rename = "WarningLevel")]
    warning_level: u32,
    #[zvariant(rename = "UpdateTime")]
    update_time: u64,
}

// #[derive(Default, Deserialize, Serialize, Type, PartialEq, Debug)]
// pub struct UPowerProperties {
//     #[serde(rename = "HasHistory")]
//     has_history: bool,
//     #[serde(rename = "HasStatistics")]
//     has_statistics: bool,
//     #[serde(rename = "IsPresent")]
//     is_present: bool,
//     #[serde(rename = "IsRechargeable")]
//     is_rechargeable: bool,
//     #[serde(rename = "Online")]
//     online: bool,
//     #[serde(rename = "PowerSupply")]
//     power_supply: bool,
//     #[serde(rename = "Capacity")]
//     capacity: f32,
//     #[serde(rename = "Energy")]
//     energy: f32,
//     #[serde(rename = "EnergyEmpty")]
//     energy_empty: f32,
//     #[serde(rename = "EnergyFull")]
//     energy_full: f32,
//     #[serde(rename = "EnergyFullDesign")]
//     energy_full_design: f32,
//     #[serde(rename = "EnergyRate")]
//     energy_rate: f32,
//     #[serde(rename = "Luminosity")]
//     luminosity: f32,
//     #[serde(rename = "Percentage")]
//     percentage: f32,
//     #[serde(rename = "Temperature")]
//     temperature: f32,
//     #[serde(rename = "Voltage")]
//     voltage: f32,
//     #[serde(rename = "TimeToEmpty")]
//     time_to_empty: i64,
//     #[serde(rename = "TimeToFull")]
//     time_to_full: i64,
//     #[serde(rename = "IconName")]
//     icon_name: String,
//     #[serde(rename = "Model")]
//     model: String,
//     #[serde(rename = "NativePath")]
//     native_path: String,
//     #[serde(rename = "Serial")]
//     serial: String,
//     #[serde(rename = "Vendor")]
//     vendor: String,
//     #[serde(rename = "BatteryLevel")]
//     battery_level: u32,
//     #[serde(rename = "State")]
//     state: u32,
//     #[serde(rename = "Technology")]
//     technology: u32,
//     #[serde(rename = "Type")]
//     r#type: u32,
//     #[serde(rename = "WarningLevel")]
//     warning_level: u32,
//     #[serde(rename = "UpdateTime")]
//     update_time: u64,
// }
//
// impl From<HashMap<String, zbus::zvariant::OwnedValue>> for UPowerProperties {
//     fn from(data: HashMap<String, zbus::zvariant::OwnedValue>) -> Self {
//         // let mut has_history = false;
//         // let mut has_statistics: bool = false;
//         // let mut is_present: bool = Default::default();
//         // let mut is_rechargeable: bool = Default::default();
//         // let mut online: bool = Default::default();
//         // let mut power_supply: bool = Default::default();
//         // let mut capacity: f32 = Default::default();
//         // let mut energy: f32 = Default::default();
//         // let mut energy_empty: f32 = Default::default();
//         // let mut energy_full: f32 = Default::default();
//         // let mut energy_full_design: f32 = Default::default();
//         // let mut energy_rate: f32 = Default::default();
//         // let mut luminosity: f32 = Default::default();
//         // let mut percentage: f32 = Default::default();
//         // let mut temperature: f32 = Default::default();
//         // let mut voltage: f32 = Default::default();
//         // let mut time_to_empty: i64 = Default::default();
//         // let mut time_to_full: i64 = Default::default();
//         // let mut icon_name: String = Default::default();
//         // let mut model: String = Default::default();
//         // let mut native_path: String = Default::default();
//         // let mut serial: String = Default::default();
//         // let mut vendor: String = Default::default();
//         // let mut battery_level: u32 = Default::default();
//         // let mut state: u32 = Default::default();
//         // let mut technology: u32 = Default::default();
//         // let mut r#type: u32 = Default::default();
//         // let mut warning_level: u32 = Default::default();
//         // let mut update_time: u64 = Default::default();
//         let mut res = Self::default();
//
//         for (k, v) in data.into_iter() {
//             match k.as_str() {
//                 "HasHistory" => {
//                     res.has_history = v.downcast::<bool>().unwrap();
//                 }
//                 "HasStatistics" => {
//                     res.has_statistics = v.downcast::<bool>().unwrap();
//                 }
//                 "IsPresent" => {
//                     res.is_present = v.downcast::<bool>().unwrap();
//                 }
//                 "IsRechargeable" => {
//                     res.is_rechargeable = v.downcast::<bool>().unwrap();
//                 }
//                 "Online" => {
//                     res.online = v.downcast::<bool>().unwrap();
//                 }
//                 "PowerSupply" => {
//                     res.power_supply = v.downcast::<bool>().unwrap();
//                 }
//                 key => error!("Found unknow key: {key:#?}"),
//             }
//         }
//         res
//
//         // #[serde(rename = "Capacity")]
//         // capacity: f32,
//         // #[serde(rename = "Energy")]
//         // energy: f32,
//         // #[serde(rename = "EnergyEmpty")]
//         // energy_empty: f32,
//         // #[serde(rename = "EnergyFull")]
//         // energy_full: f32,
//         // #[serde(rename = "EnergyFullDesign")]
//         // energy_full_design: f32,
//         // #[serde(rename = "EnergyRate")]
//         // energy_rate: f32,
//         // #[serde(rename = "Luminosity")]
//         // luminosity: f32,
//         // #[serde(rename = "Percentage")]
//         // percentage: f32,
//         // #[serde(rename = "Temperature")]
//         // temperature: f32,
//         // #[serde(rename = "Voltage")]
//         // voltage: f32,
//         // #[serde(rename = "TimeToEmpty")]
//         // time_to_empty: i64,
//         // #[serde(rename = "TimeToFull")]
//         // time_to_full: i64,
//         // #[serde(rename = "IconName")]
//         // icon_name: String,
//         // #[serde(rename = "Model")]
//         // model: String,
//         // #[serde(rename = "NativePath")]
//         // native_path: String,
//         // #[serde(rename = "Serial")]
//         // serial: String,
//         // #[serde(rename = "Vendor")]
//         // vendor: String,
//         // #[serde(rename = "BatteryLevel")]
//         // battery_level: u32,
//         // #[serde(rename = "State")]
//         // state: u32,
//         // #[serde(rename = "Technology")]
//         // technology: u32,
//         // #[serde(rename = "Type")]
//         // r#type: u32,
//         // #[serde(rename = "WarningLevel")]
//         // warning_level: u32,
//         // #[serde(rename = "UpdateTime")]
//         // update_time: u64,
//     }
// }
