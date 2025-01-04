use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Settings {
    vehicle: VehicleSettings,
    atc: ATCSettings,
    ato: ATOSettings,
    tasc: TASCSettings,
    tims: TIMSSettings,
    sound: SoundSettings,
}

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct VehicleSettings {
    /// 出力する力行の段数
    power_notches: i32,
    /// 出力するブレーキの段数
    brake_notches: i32,
    /// 定速制御を開始する速度
    constant_start_speed: f32,
    /// 抑速制御を開始する速度
    yokusoku_start_speed: f32,
}

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct ATCSettings {
    
}

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct ATOSettings {

}

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct TASCSettings {

}

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct TIMSSettings {

}

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct SoundSettings {

}