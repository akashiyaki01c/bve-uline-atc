use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Settings {
    pub vehicle: VehicleSettings,
    pub atc: ATCSettings,
    pub ato: ATOSettings,
    pub tasc: TASCSettings,
    pub tims: TIMSSettings,
    pub sound: SoundSettings,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct VehicleSettings {
    /// 入力する力行の段数
    pub input_power_notches: i32,
    /// 入力するブレーキの段数
    pub input_brake_notches: i32,
    /// 出力する力行の段数
    pub output_power_notches: i32,
    /// 出力するブレーキの段数
    pub output_brake_notches: i32,
    /// 定速制御を開始する速度 [km/h]
    pub constant_start_speed: f32,
    /// 抑速制御を開始する速度 [km/h]
    pub yokusoku_start_speed: f32,
}
impl Default for VehicleSettings {
    fn default() -> Self {
        Self { 
            input_power_notches: 4, 
            input_brake_notches: 7, 
            output_power_notches: 31, 
            output_brake_notches: 31, 
            constant_start_speed: 25.0, 
            yokusoku_start_speed: 5.0
        }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct ATCSettings {
    /// ATC速度照査のマージン
    pub check_speed_margin: f32,
    /// 緩和ブレーキの長さ [ms]
    pub half_brake_time: i32,
    /// 確認運転時の照査速度 [km/h]
    pub kakunin_limit_speed: f32,
    /// 非常運転時の照査速度 [km/h]
    pub hijo_limit_speed: f32,
}
impl Default for ATCSettings {
    fn default() -> Self {
        Self {
            check_speed_margin: 1.5,
            half_brake_time: 900,
            kakunin_limit_speed: 15.0,
            hijo_limit_speed: 15.0,
        }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct ATOSettings {
    /// ATC速度とATO目標速度との差 [km/h]
    target_speed: f32,
    /// TASC第2パターン発生時の照査速度 [km/h]
    p2_check_speed: f32,
    /// 過速防止の照査速度 [km/h]
    p3_check_speed: f32,
    /// 減速制御時の最大減速時間 [ms]
    p4_brake_time: i32,
    /// 力行OFF制御時の最低条件速度 [km/h]
    p5_lower_limit_speed: f32,
}
impl Default for ATOSettings {
    fn default() -> Self {
        Self {
            target_speed: 3.0,
            p2_check_speed: 25.0,
            p3_check_speed: 5.0,
            p4_brake_time: 8000,
            p5_lower_limit_speed: 35.0
        }
    }
}

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct TASCSettings {
    /// 在来車のTASCパターンか
    is_old_pattern: bool,
}

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct TIMSSettings {
    /// TIMS画面の描画速度
    display_draw_time: i32,
}

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct SoundSettings {

}