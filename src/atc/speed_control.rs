//! 定速制御/抑速制御を制御する関数群

use bveats_rs::{AtsConstantSpeed, AtsHandles, AtsVehicleState};
use log::info;

use super::uline_atc::ULineATC;

/// 定速制御の条件を満たしているかを判断する関数
/// (速度が15km/h以上で、P4→P3に戻された時に条件を満たす)
pub fn is_constant_speed(atc: &mut ULineATC, speed: f32, beforeNotch: i32, afterNotch: i32) -> bool {
	info!("[CONSTANT] Target: {:?}", atc.constant_target_speed);
	if speed >= atc.settings.vehicle.constant_start_speed && beforeNotch == 4 && afterNotch == 3 {
		if atc.constant_target_speed == 0.0 {
			atc.constant_target_speed = atc.speed;
		}
		true
	} else {
		atc.constant_target_speed = 0.0;
		false
	}
}

/// 抑速制御の条件を満たしているかを判断する関数
/// (速度が25km/h以上で、B2→B1に戻された時に条件を満たす)
pub fn is_holding_speed(atc: &ULineATC, speed: f32, beforeNotch: i32, afterNotch: i32) -> bool {
	speed >= atc.settings.vehicle.yokusoku_start_speed && beforeNotch == -2 && afterNotch == -1
}

/// 空制の抑速制御の条件を満たしているかを判断する関数
/// (速度が25km/h以下で、抑速ノッチが投入されている時に条件を満たす)
pub fn is_air_holding_speed(atc: &ULineATC, speed: f32, notch: i32) -> bool {
	speed < atc.settings.vehicle.yokusoku_start_speed && notch < 0
}

/// 定速制御を適用する関数
fn constant_speed(_atc: &ULineATC, handles: AtsHandles) -> AtsHandles {
    /* handles.constant_speed = AtsConstantSpeed::Enable as i32;
	handles */
	atc_constant_speed(_atc)
}

/// 抑速制御を適用する関数
fn holding_speed(_atc: &ULineATC, mut handles: AtsHandles) -> AtsHandles {
    handles.constant_speed = AtsConstantSpeed::Enable as i32;
	handles.power = 0;
	handles
}

/// 空制の抑速制御を適用する関数
fn air_holding_speed(atc: &ULineATC, mut handles: AtsHandles) -> AtsHandles {
    handles.brake = atc.convert_output_notch(handles.power.abs()).max(handles.brake);
	handles.power = 0;
	handles.reverser = 0;
	handles
}

/// 定速制御/抑速制御の判定を満たした上でAtsHandlesを返す関数
pub fn constant_and_holding_speed(atc: &ULineATC, mut handles: AtsHandles, is_constant_speed: bool, is_holding_speed: bool, is_air_holding_speed: bool) -> AtsHandles {
	if handles.power < 0 {
		handles.brake = atc.convert_output_notch(-handles.power);
		handles.power = 0;
	}
	if is_constant_speed {
		handles = constant_speed(atc, handles);
	} else if is_holding_speed {
		handles = holding_speed(atc, handles);
	} else {
		handles.constant_speed = AtsConstantSpeed::Disable as i32;
	}
	if is_air_holding_speed {
		handles = air_holding_speed(atc, handles);
	}

	handles
}

pub fn atc_constant_speed(atc: &ULineATC) -> AtsHandles {
	let delta = atc.time - atc.before_time;
	let acceleration_km_h_s = (atc.speed - atc.before_speed) / (delta as f32 / 1000.0);

	let speed_2second = atc.speed + (acceleration_km_h_s * 1.0);
	let target_speed = atc.constant_target_speed; // ATO目標速度
	let speed_diff = target_speed as f32 - speed_2second;
	let mut power_notch: i32 = (speed_diff / 0.4) as i32;
	let mut brake_notch = (speed_diff / 0.3) as i32;

	power_notch += ((speed_diff) / 0.50).clamp(-10.0, 10.0) as i32;
	brake_notch += ((speed_diff) / 0.50).clamp(-10.0, 10.0) as i32;

	AtsHandles {
		power: power_notch.clamp(0, 31),
		brake: -brake_notch.clamp(-31, 0),
		reverser: 1,
		constant_speed: AtsConstantSpeed::Disable as i32
	}
}