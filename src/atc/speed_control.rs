//! 定速制御/抑速制御を制御する関数群

use bveats_rs::{AtsConstantSpeed, AtsHandles};

/// 定速制御の条件を満たしているかを判断する関数
/// (速度が15km/h以上で、P4→P3に戻された時に条件を満たす)
pub fn is_constant_speed(speed: f32, beforeNotch: i32, afterNotch: i32) -> bool {
	speed >= 15.0 && beforeNotch == 4 && afterNotch == 3
}

/// 抑速制御の条件を満たしているかを判断する関数
/// (速度が25km/h以上で、B2→B1に戻された時に条件を満たす)
pub fn is_holding_speed(speed: f32, beforeNotch: i32, afterNotch: i32) -> bool {
	speed >= 25.0 && beforeNotch == -2 && afterNotch == -1
}

/// 空制の抑速制御の条件を満たしているかを判断する関数
/// (速度が25km/h以下で、抑速ノッチが投入されている時に条件を満たす)
pub fn is_air_holding_speed(speed: f32, notch: i32) -> bool {
	speed < 25.0 && notch < 0
}

/// 定速制御を適用する関数
fn constant_speed(mut handles: AtsHandles) -> AtsHandles {
    handles.constant_speed = AtsConstantSpeed::Enable as i32;
	handles
}

/// 抑速制御を適用する関数
fn holding_speed(mut handles: AtsHandles) -> AtsHandles {
    handles.constant_speed = AtsConstantSpeed::Enable as i32;
	handles.power = 0;
	handles
}

/// 空制の抑速制御を適用する関数
fn air_holding_speed(mut handles: AtsHandles) -> AtsHandles {
    handles.brake = handles.power.abs().max(handles.brake);
	handles.power = 0;
	handles.reverser = 0;
	handles
}

/// 定速制御/抑速制御の判定を満たした上でAtsHandlesを返す関数
pub fn constant_and_holding_speed(mut handles: AtsHandles, is_constant_speed: bool, is_holding_speed: bool, is_air_holding_speed: bool) -> AtsHandles {
	if is_constant_speed {
		handles = constant_speed(handles);
	} else if is_holding_speed {
		handles = holding_speed(handles);
	} else {
		handles.constant_speed = AtsConstantSpeed::Disable as i32;
	}
	if is_air_holding_speed {
		handles = air_holding_speed(handles);
	}

	handles
}