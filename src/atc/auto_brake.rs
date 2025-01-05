use bveats_rs::{AtsConstantSpeed, AtsHandles, AtsSound, AtsVehicleState};

use super::{atc_signal::AtcSignal, uline_atc::{AtcBrakeStatus, ULineATC}};

const ATS_SOUND_BUZZER: usize = 3;

/// ATCブレーキなし状態のAtsHandlesを取得
fn get_none_brake_handle(_atc: &ULineATC, handles: AtsHandles) -> AtsHandles {
	handles
}
/// ATC緩和ブレーキ状態のAtsHandlesを取得
fn get_half_brake_handle(_atc: &ULineATC, mut handles: AtsHandles) -> AtsHandles {
	handles.brake = _atc.settings.vehicle.output_brake_notches / 2;
	handles.constant_speed = AtsConstantSpeed::Disable as i32;
	handles
}
/// ATC常用ブレーキ状態のAtsHandlesを取得
fn get_full_brake_handle(_atc: &ULineATC, mut handles: AtsHandles) -> AtsHandles {
	handles.brake = _atc.settings.vehicle.output_brake_notches;
	handles.constant_speed = AtsConstantSpeed::Disable as i32;
	handles
}
/// ATC非常ブレーキ状態のAtsHandlesを取得
fn get_emg_brake_handle(_atc: &ULineATC, mut handles: AtsHandles) -> AtsHandles {
	handles.brake = _atc.settings.vehicle.output_brake_notches + 1;
	handles.constant_speed = AtsConstantSpeed::Disable as i32;
	handles
}

/// ATCブレーキが有効かを判断する関数
fn enable_atc_brake(signal_speed: f32, vehicle_speed: f32) -> bool {
	signal_speed <= vehicle_speed
}

/// ATC有効時にElapse内のATCブレーキ判定を行う関数
pub fn elapse_atc_brake<'a>(atc: &'a mut ULineATC, handles: AtsHandles, state: AtsVehicleState, sound: &'a mut [i32]) -> AtsHandles {

	let enable_auto_brake = enable_atc_brake(atc.now_signal.getSpeed() as f32 + atc.settings.atc.check_speed_margin, state.speed);
	// ブレーキが掛かった瞬間
	if atc.atc_brake_status == AtcBrakeStatus::Passing && enable_auto_brake {
		atc.atc_brake_status = AtcBrakeStatus::HalfBraking(state.time);
	}
	// 緩和ブレーキからフルブレーキになる瞬間
	if let AtcBrakeStatus::HalfBraking(time) = atc.atc_brake_status {
		if time + atc.settings.atc.half_brake_time < state.time {
			atc.atc_brake_status = AtcBrakeStatus::FullBraking;
		}
	}
	// ブレーキが解除された瞬間
	if atc.atc_brake_status == AtcBrakeStatus::EmergencyBraking && !enable_auto_brake {
		atc.atc_brake_status = AtcBrakeStatus::Passing;
	}
	if atc.atc_brake_status == AtcBrakeStatus::FullBraking && !enable_auto_brake {
		atc.atc_brake_status = AtcBrakeStatus::Passing;
	}
	if !enable_auto_brake {
		if let AtcBrakeStatus::HalfBraking(_) = atc.atc_brake_status {
			atc.atc_brake_status = AtcBrakeStatus::Passing;
		}
	}

	// 02信号なら非常ブレーキ
	if atc.now_signal == AtcSignal::Signal02 {
		atc.atc_brake_status = AtcBrakeStatus::EmergencyBraking;
	}

	// 非常運転の場合
	if atc.enable_02hijo_unten {
		if atc.now_signal == AtcSignal::Signal02 {
			if state.speed <= (atc.settings.atc.hijo_limit_speed + atc.settings.atc.check_speed_margin) {
				atc.atc_brake_status = AtcBrakeStatus::Passing;
			} else {
				atc.atc_brake_status = AtcBrakeStatus::FullBraking;
			}
		} else {
			atc.enable_02hijo_unten = false;
		}
	}
	// 確認運転の場合
	if atc.enable_01kakunin_unten {
		if atc.now_signal == AtcSignal::Signal01 {
			if state.speed <= (atc.settings.atc.kakunin_limit_speed + atc.settings.atc.check_speed_margin) {
				atc.atc_brake_status = AtcBrakeStatus::Passing;
			} else {
				atc.atc_brake_status = AtcBrakeStatus::FullBraking;
			}
		} else {
			atc.enable_01kakunin_unten = false;
		}
	}


	match atc.atc_brake_status {
		AtcBrakeStatus::Passing => {
			sound[ATS_SOUND_BUZZER] = AtsSound::Stop as i32;
		},
		_ => {
			sound[ATS_SOUND_BUZZER] = AtsSound::PlayLooping as i32;
		}
	}
	match atc.atc_brake_status {
		AtcBrakeStatus::EmergencyBraking => get_emg_brake_handle(atc, handles),
		AtcBrakeStatus::HalfBraking(_) => get_half_brake_handle(atc, handles),
		AtcBrakeStatus::FullBraking => get_full_brake_handle(atc, handles),
		AtcBrakeStatus::Passing => get_none_brake_handle(atc, handles),
	}
}

/// ATC非設時にElapse内のATCブレーキ判定を行う関数
pub fn elapse_hisetsu_brake(_atc: &mut ULineATC, handles: AtsHandles) -> AtsHandles {
	handles
}