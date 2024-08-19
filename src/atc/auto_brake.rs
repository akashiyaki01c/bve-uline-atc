use bveats_rs::{AtsHandles, AtsSound, AtsVehicleState};

use super::{atc_signal::AtcSignal, uline_atc::{AtcBrakeStatus, ULineATC}};

const ATS_SOUND_BELL: usize = 2;
const ATS_SOUND_BUZZER: usize = 3;

pub fn elapse_atc_brake(atc: &mut ULineATC, state: AtsVehicleState, sound: &mut [i32]) -> AtsHandles {
	// ATCブレーキなし状態
	let atc_none_brake_handle = AtsHandles { 
		brake: atc.man_brake,
		power: atc.man_power, 
		reverser: atc.man_reverser, 
		constant_speed: 0 
	};
	// ATC緩和ブレーキ状態
	let atc_half_brake_handle = AtsHandles {
		brake: atc.vehicle_spec.brake_notches / 2,
		power: 0,
		reverser: atc.man_reverser,
		constant_speed: 0
	};
	// ATC常用ブレーキ状態
	let atc_full_brake_handle = AtsHandles {
		brake: atc.vehicle_spec.brake_notches,
		power: 0,
		reverser: atc.man_reverser,
		constant_speed: 0
	};
	// ATC非常ブレーキ状態
	let atc_emg_brake_handle = AtsHandles {
		brake: atc.vehicle_spec.brake_notches + 1,
		power: 0,
		reverser: atc.man_reverser,
		constant_speed: 0
	};

	let enable_auto_brake = state.speed as i32 > atc.now_signal.getSpeed();
	// ブレーキが掛かった瞬間
	if atc.atc_brake_status == AtcBrakeStatus::Passing && enable_auto_brake {
		println!("[Brake] Passing -> StartHalfBraking");
		atc.atc_brake_status = AtcBrakeStatus::StartHalfBraking(state.time);
	}
	if enable_auto_brake {
		println!("[Brake] EndHalfBraking -> StartHalfBraking");
		if let AtcBrakeStatus::EndHalfBraking(_) = atc.atc_brake_status {
			atc.atc_brake_status = AtcBrakeStatus::StartHalfBraking(state.time);
		}
	}
	// 緩和ブレーキからフルブレーキになる瞬間
	if let AtcBrakeStatus::StartHalfBraking(time) = atc.atc_brake_status {
		println!("[Brake] StartHalfBraking -> FullBraking");
		if time + 1000 < state.time {
			atc.atc_brake_status = AtcBrakeStatus::FullBraking;
		}
	}
	// ブレーキが解除された瞬間
	if atc.atc_brake_status == AtcBrakeStatus::FullBraking && !enable_auto_brake {
		println!("[Brake] FullBraking -> EndHalfBraking");
		atc.atc_brake_status = AtcBrakeStatus::EndHalfBraking(state.time);
	}
	if !enable_auto_brake {
		if let AtcBrakeStatus::StartHalfBraking(_) = atc.atc_brake_status {
			println!("[Brake] StartHalfBraking -> EndHalfBraking");
			atc.atc_brake_status = AtcBrakeStatus::EndHalfBraking(state.time);
		}
	}
	// 緩和ブレーキから緩解になる瞬間
	if let AtcBrakeStatus::EndHalfBraking(time) = atc.atc_brake_status {
		println!("[Brake] EndHalfBraking -> Passing");
		if time + 1000 < state.time {
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
		AtcBrakeStatus::StartHalfBraking(_) => atc_half_brake_handle,
		AtcBrakeStatus::EndHalfBraking(_) => atc_half_brake_handle,
		AtcBrakeStatus::FullBraking => atc_full_brake_handle,
		AtcBrakeStatus::Passing => atc_none_brake_handle,
	}
}