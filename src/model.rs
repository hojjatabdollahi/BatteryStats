use serde::{Deserialize, Serialize};
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

#[derive(Type, Default, DeserializeDict, SerializeDict, PartialEq, Debug)]
#[zvariant(signature = "dict")]
pub struct UPowerProperties {
    #[zvariant(rename = "HasHistory")]
    pub has_history: bool,
    #[zvariant(rename = "HasStatistics")]
    pub has_statistics: bool,
    #[zvariant(rename = "IsPresent")]
    pub is_present: bool,
    #[zvariant(rename = "IsRechargeable")]
    pub is_rechargeable: bool,
    #[zvariant(rename = "Online")]
    pub online: bool,
    #[zvariant(rename = "PowerSupply")]
    pub power_supply: bool,
    #[zvariant(rename = "Capacity")]
    pub capacity: f32,
    #[zvariant(rename = "Energy")]
    pub energy: f32,
    #[zvariant(rename = "EnergyEmpty")]
    pub energy_empty: f32,
    #[zvariant(rename = "EnergyFull")]
    pub energy_full: f32,
    #[zvariant(rename = "EnergyFullDesign")]
    pub energy_full_design: f32,
    #[zvariant(rename = "EnergyRate")]
    pub energy_rate: f32,
    #[zvariant(rename = "Luminosity")]
    pub luminosity: f32,
    #[zvariant(rename = "Percentage")]
    pub percentage: f32,
    #[zvariant(rename = "Temperature")]
    pub temperature: f32,
    #[zvariant(rename = "Voltage")]
    pub voltage: f32,
    #[zvariant(rename = "TimeToEmpty")]
    pub time_to_empty: i64,
    #[zvariant(rename = "TimeToFull")]
    pub time_to_full: i64,
    #[zvariant(rename = "IconName")]
    pub icon_name: String,
    #[zvariant(rename = "Model")]
    pub model: String,
    #[zvariant(rename = "NativePath")]
    pub native_path: String,
    #[zvariant(rename = "Serial")]
    pub serial: String,
    #[zvariant(rename = "Vendor")]
    pub vendor: String,
    #[zvariant(rename = "BatteryLevel")]
    pub battery_level: u32,
    #[zvariant(rename = "State")]
    pub state: u32,
    #[zvariant(rename = "Technology")]
    pub technology: u32,
    #[zvariant(rename = "Type")]
    pub r#type: u32,
    #[zvariant(rename = "WarningLevel")]
    pub warning_level: u32,
    #[zvariant(rename = "UpdateTime")]
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BatHistory {
    pub properties: UPowerProperties,
    pub data: Vec<(u32, f32, u32)>,
}

#[derive(Serialize, Deserialize)]
pub struct DataLayout {
    pub p: Vec<u8>,
    pub d: Vec<(u32, f32, u32)>,
}
