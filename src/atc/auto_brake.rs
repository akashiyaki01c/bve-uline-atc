use bveats_rs::{AtsHandles, AtsSound, AtsVehicleState};

use super::{atc_signal::AtcSignal, uline_atc::{AtcBrakeStatus, ULineATC}};

const ATS_SOUND_BELL: usize = 2;
const ATS_SOUND_BUZZER: usize = 3;

/// ATCブレーキなし状態のAtsHandlesを取得
fn get_none_brake_handle(atc: &ULineATC) -> AtsHandles {
	AtsHandles { 
		brake: atc.man_brake,
		power: atc.man_power, 
		reverser: atc.man_reverser, 
		constant_speed: 0 
	}
}
/// ATC緩和ブレーキ状態のAtsHandlesを取得
fn get_half_brake_handle(atc: &ULineATC) -> AtsHandles {
	AtsHandles {
		brake: 4,
		power: 0,
		reverser: atc.man_reverser,
		constant_speed: 0
	}
}
/// ATC常用ブレーキ状態のAtsHandlesを取得
fn get_full_brake_handle(atc: &ULineATC) -> AtsHandles {
	AtsHandles {
		brake: atc.vehicle_spec.brake_notches,
		power: 0,
		reverser: atc.man_reverser,
		constant_speed: 0
	}
}
/// ATC非常ブレーキ状態のAtsHandlesを取得
fn get_emg_brake_handle(atc: &ULineATC) -> AtsHandles {
	AtsHandles {
		brake: atc.vehicle_spec.brake_notches + 1,
		power: 0,
		reverser: atc.man_reverser,
		constant_speed: 0
	}
}

pub fn elapse_atc_brake(atc: &mut ULineATC, state: AtsVehicleState, sound: &mut [i32]) -> AtsHandles {
	let atc_none_brake_handle = get_none_brake_handle(atc);
	let atc_half_brake_handle =  get_half_brake_handle(atc);
	let atc_full_brake_handle = get_full_brake_handle(atc);
	let atc_emg_brake_handle = get_emg_brake_handle(atc);

	let enable_auto_brake = state.speed as i32 > atc.now_signal.getSpeed();
	// ブレーキが掛かった瞬間
	if atc.atc_brake_status == AtcBrakeStatus::Passing && enable_auto_brake {
		println!("[Brake] Passing -> StartHalfBraking");
		atc.atc_brake_status = AtcBrakeStatus::HalfBraking(state.time);
	}
	// 緩和ブレーキからフルブレーキになる瞬間
	if let AtcBrakeStatus::HalfBraking(time) = atc.atc_brake_status {
		println!("[Brake] StartHalfBraking -> FullBraking");
		if time + 700 < state.time {
			atc.atc_brake_status = AtcBrakeStatus::FullBraking;
		}
	}
	// ブレーキが解除された瞬間
	if atc.atc_brake_status == AtcBrakeStatus::EmergencyBraking && !enable_auto_brake {
		println!("[Brake] EmgBraking -> Passing");
		atc.atc_brake_status = AtcBrakeStatus::Passing;
	}
	if atc.atc_brake_status == AtcBrakeStatus::FullBraking && !enable_auto_brake {
		println!("[Brake] FullBraking -> Passing");
		atc.atc_brake_status = AtcBrakeStatus::Passing;
	}
	if !enable_auto_brake {
		if let AtcBrakeStatus::HalfBraking(_) = atc.atc_brake_status {
			println!("[Brake] StartHalfBraking -> Passing");
			atc.atc_brake_status = AtcBrakeStatus::Passing;
		}
	}

	// 02信号なら非常ブレーキ
	if atc.now_signal == AtcSignal::Signal02 {
		println!("[Brake] EmergencyBraking!!!");
		atc.atc_brake_status = AtcBrakeStatus::EmergencyBraking;
	}

	// ATC音関連
	if atc.is_changing_signal {
		sound[ATS_SOUND_BELL] = AtsSound::Play as i32;
		atc.is_changing_signal = false;
	} else {
		sound[ATS_SOUND_BELL] = AtsSound::Continue as i32;
	}

	// 非常運転の場合
	if atc.enable_02hijo_unten {
		if atc.now_signal == AtcSignal::Signal02 {
			if state.speed as i32 <= 15 {
				atc.atc_brake_status = AtcBrakeStatus::Passing;
			} else {
				atc.atc_brake_status = AtcBrakeStatus::FullBraking;
			}
		} else {
			atc.atc_brake_status = AtcBrakeStatus::EmergencyBraking;
			if state.speed as i32 == 0 {
				atc.enable_02hijo_unten = false;
			}
		}
	}
	// 確認運転の場合
	if atc.enable_01kakunin_unten {
		if atc.now_signal == AtcSignal::Signal01 {
			if state.speed as i32 <= 15 {
				atc.atc_brake_status = AtcBrakeStatus::Passing;
			} else {
				atc.atc_brake_status = AtcBrakeStatus::FullBraking;
			}
		} else {
			atc.atc_brake_status = AtcBrakeStatus::EmergencyBraking;
			if state.speed as i32 == 0 {
				atc.enable_01kakunin_unten = false;
			}
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
		AtcBrakeStatus::EmergencyBraking => atc_emg_brake_handle,
		AtcBrakeStatus::HalfBraking(_) => atc_half_brake_handle,
		AtcBrakeStatus::FullBraking => atc_full_brake_handle,
		AtcBrakeStatus::Passing => atc_none_brake_handle,
	}
}

pub fn elapse_irekae_brake(atc: &mut ULineATC, state: AtsVehicleState, _sound: &mut [i32]) -> AtsHandles {
	let atc_none_brake_handle = get_none_brake_handle(atc);
	let atc_full_brake_handle = get_full_brake_handle(atc);

	let atc_signal_speed = atc.now_signal.getSpeed();
	if atc_signal_speed < (state.speed as i32).min(25) {
		atc_full_brake_handle
	} else {
		atc_none_brake_handle
	}
}

pub fn elapse_hisetsu_brake(atc: &mut ULineATC, _state: AtsVehicleState, _sound: &mut [i32]) -> AtsHandles {
	let atc_none_brake_handle = get_none_brake_handle(atc);
	atc_none_brake_handle
}