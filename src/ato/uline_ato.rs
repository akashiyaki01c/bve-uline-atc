
use bveats_rs::{AtsConstantSpeed, AtsHandles, AtsKey, AtsVehicleState, BveAts};

use crate::atc::atc_signal::AtcSignal;

/// ATOの状態を表す
#[derive(Debug, PartialEq)]
enum ATOStatus {
    /// 停止状態
    Stop,
    /// 出発制御
    Departure,
    /// 定速運転制御
    ConstantSpeed,
    /// 力行OFF制御
    PowerOff(AtcSignal),
    /// 減速制御
    Braking(i32, AtcSignal),
    /// 定位置停止制御(90パターン)
    TASC90(i32, f32, f32),
    /// 定位置停止制御(75パターン)
    TASC1(i32, f32, f32),
    /// 定位置停止制御(20パターン)
    TASC2(i32, f32, f32),
    /// 過速防止制御(P3)
    P3(i32, f32, f32),
}
impl Default for ATOStatus {
    fn default() -> Self {
        Self::Stop
    }
}

/// ATOを表す
#[derive(Debug, Default)]
pub struct ULineATO {
    pub status: ATOStatus,
    before_ato_notch: AtsHandles,
    signal: AtcSignal,
    now_power: i32,
    now_brake: i32,
    before_time: i32,
    before_speed: f32,
    before_acceleration: f32,
    recent_operation_time: i32,
    recent_tasc_operation_time: i32,
    recent_tasc_calced_deceleration: f32,
    recent_tasc_deceleration: f32,
    recent_tasc_difference: f32,
    is_not_one_time_braking: bool,
}
impl BveAts for ULineATO {
    fn load(&mut self) {
    }

    fn dispose(&mut self) {
    }

    fn set_vehicle_spec(&mut self, _spec: bveats_rs::AtsVehicleSpec) {
    }

    fn initialize(&mut self, _handle: bveats_rs::AtsInit) {
    }

    fn elapse(&mut self, state: bveats_rs::AtsVehicleState, _panel: &mut [i32], _sound: &mut [i32]) -> bveats_rs::AtsHandles {
        let delta = state.time - self.before_time;
        let acceleration_km_h_s = (state.speed - self.before_speed) / (delta as f32 / 1000.0);

        let atc_brake =  state.speed > self.signal.getSpeed() as f32;
        let target_speed = self.signal.getSpeed() - 5; // ATO目標速度

        // ATCブレーキチェック
        if atc_brake {
            self.now_power = 0;
        }

        let result = match self.status {
            ATOStatus::Departure => {
                if target_speed as f32 <= state.speed {
                    println!("[ATO] Departure->ConstantSpeed");
                    self.status = ATOStatus::ConstantSpeed;
                }

                let operatable100 = || {
                    let result = (state.time - self.recent_operation_time) > 100;
                    result
                };
                if self.now_brake != 0 {
                    if operatable100() {
                        self.now_brake = 0;
                        self.recent_operation_time = state.time;
                    }
                } else {
                    if operatable100() {
                        self.now_power += 1;
                        self.recent_operation_time = state.time;
                    }
                }

                self.now_power = self.now_power.clamp(0, 31);
                self.now_brake = self.now_brake.clamp(0, 31);


                self.before_time = state.time;
                self.before_speed = state.speed;
                self.before_acceleration = acceleration_km_h_s;

                AtsHandles {
                    power: self.now_power,
                    brake: 0,
                    reverser: 1,
                    constant_speed: 0
                }
            }
            ATOStatus::ConstantSpeed => {
                let result = self.ato_constant_speed(state);
                self.before_time = state.time;
                self.before_speed = state.speed;
                self.before_acceleration = acceleration_km_h_s;
                result
            }
            ATOStatus::TASC1(_pattern_start_time, beacon_location, target_distance) => {
                if beacon_location.is_nan() {
                    if let ATOStatus::TASC1(_, location, _) = &mut self.status {
                        *location = state.location as f32;
                    }
                }
                // let result = self.ato_tasc(state, pattern_start_time, 80.0);
                let result = &self.ato_tasc_with_distance(state, (beacon_location + target_distance) - state.location as f32);
                self.before_time = state.time;
                self.before_speed = state.speed;
                self.before_acceleration = acceleration_km_h_s;
                result.clone()
            }
            ATOStatus::TASC2(pattern_start_time, beacon_location, target_distance) => {
                if state.speed < 1.0 {
                    let status = ATOStatus::P3(pattern_start_time, beacon_location, target_distance);
                    println!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }
                if state.speed > 25.0 {
                    return AtsHandles {
                        brake: 8,
                        power: 0,
                        reverser: 1,
                        constant_speed: AtsConstantSpeed::Disable as i32,
                    }
                }
                if beacon_location.is_nan() {
                    if let ATOStatus::TASC2(_, location, _) = &mut self.status {
                        *location = state.location as f32;
                    }
                }
                // let result = self.ato_tasc(state, pattern_start_time, 20.0);
                let result = &self.ato_tasc_with_distance(state, (beacon_location + target_distance) - state.location as f32);
                self.before_time = state.time;
                self.before_speed = state.speed;
                self.before_acceleration = acceleration_km_h_s;
                result.clone()
            }
            ATOStatus::TASC90(_pattern_start_time, beacon_location, target_distance) => {
                // let result = self.ato_tasc(state, pattern_start_time, 95.0);
                if beacon_location.is_nan() {
                    if let ATOStatus::TASC90(_, location, _) = &mut self.status {
                        *location = state.location as f32;
                    }
                }
                let result = &self.ato_tasc_with_distance(state, (beacon_location + target_distance) - state.location as f32);
                self.before_time = state.time;
                self.before_speed = state.speed;
                self.before_acceleration = acceleration_km_h_s;
                result.clone()
            }
            ATOStatus::P3(_pattern_start_time, beacon_location, target_distance) => {
                let result = &self.ato_tasc_with_distance(state, (beacon_location + target_distance) - state.location as f32);
                
                if state.speed == 0.0 {
                    let status = ATOStatus::Stop;
                    println!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }

                self.before_time = state.time;
                self.before_speed = state.speed;
                self.before_acceleration = acceleration_km_h_s;

                /* AtsHandles {
                    power: self.now_power,
                    brake: self.now_brake,
                    reverser: 1,
                    constant_speed: 0
                } */
                result.clone()
            }
            ATOStatus::Braking(mut time, signal) => {
                if time == -1 {
                    if let ATOStatus::Braking(time, _) = &mut self.status {
                        *time = state.time;
                    }
                    time = state.time;
                    // 一個下現示速度以下
                    let lower_signal_speed = self.signal.getLower().getSpeed();
                    if state.speed < lower_signal_speed as f32 {
                        let status = ATOStatus::ConstantSpeed;
                        println!("[ATO] {:?}→{:?}", self.status, status);
                        self.status = status;
                    }
                }
                // 8秒以上
                if time + 8000 < state.time {
                    let status = ATOStatus::ConstantSpeed;
                    println!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }
                // 現示が変位
                if signal != self.signal {
                    let status = ATOStatus::ConstantSpeed;
                    println!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }

                let operatable200 = || {
                    let result = (state.time - self.recent_operation_time) > 200;
                    result
                };
                if operatable200() {
                    self.now_brake += 1;
                    self.recent_operation_time = state.time;
                }

                self.now_power = self.now_power.clamp(0, 31);
                self.now_brake = self.now_brake.clamp(0, 23);

                AtsHandles {
                    power: self.now_power,
                    brake: self.now_brake,
                    reverser: 1,
                    constant_speed: AtsConstantSpeed::Disable as i32
                }
            }
            ATOStatus::PowerOff(signal) => {
                // 35km/h以下
                if state.speed < 35.0 {
                    let status = ATOStatus::ConstantSpeed;
                    println!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }
                // 上位現示に変化
                if signal.getSpeed() < self.signal.getSpeed() {
                    let status = ATOStatus::ConstantSpeed;
                    println!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }
                // 目標速度-5km/h
                if target_speed as f32 - 5.0 < state.speed {
                    let status = ATOStatus::ConstantSpeed;
                    println!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                }

                AtsHandles {
                    power: 0,
                    brake: 0,
                    reverser: 1,
                    constant_speed: AtsConstantSpeed::Disable as i32
                }
            }
            ATOStatus::Stop => {
                AtsHandles {
                    power: 0,
                    brake: 7,
                    reverser: 0,
                    constant_speed: AtsConstantSpeed::Disable as i32
                }
            }
        };
        self.before_ato_notch = result;
        result
    }

    fn set_power(&mut self, _notch: i32) {
        
    }

    fn set_brake(&mut self, _notch: i32) {
        
    }

    fn set_reverser(&mut self, _notch: i32) {
        
    }

    fn key_down(&mut self, key: bveats_rs::AtsKey) {
        match key {
            AtsKey::S => {
                if self.before_speed != 0.0 {
                    return;
                }
                let status = ATOStatus::Departure;
                println!("[ATO] {:?}→{:?}", self.status, status);
                self.status = status;
            }
            _ => {}
        }
    }

    fn key_up(&mut self, _key: bveats_rs::AtsKey) {
        
    }

    fn horn_blow(&mut self, _horn_type: bveats_rs::AtsHorn) {
        
    }

    fn door_open(&mut self) {
        
    }

    fn door_close(&mut self) {
        
    }

    fn set_signal(&mut self, signal: i32) {
        self.signal = unsafe { std::mem::transmute(signal as u8) };
    }

    fn set_beacon_data(&mut self, data: bveats_rs::AtsBeaconData) {
        match data.beacon_type {
            1 => { // 第1パターン
                let status = ATOStatus::TASC1(self.before_time, f32::NAN , 350.5);
                println!("[ATO] {:?}→{:?}", self.status, status);
                self.status = status;
            }
            2 => { // 第2パターン
                let status = ATOStatus::TASC2(self.before_time, f32::NAN , 25.5);
                println!("[ATO] {:?}→{:?}", self.status, status);
                self.status = status;
            }
            3 => { // 過速防止
                if let ATOStatus::TASC2(pattern_start_time, beacon_location, target_distance) = self.status {
                    let status = ATOStatus::P3(pattern_start_time, beacon_location, target_distance);
                    println!("[ATO] {:?}→{:?}", self.status, status);
                    self.status = status;
                    self.recent_tasc_operation_time = 0;
                    self.is_not_one_time_braking = false;
                };
            }
            4 => { // 減速制御
                let status = ATOStatus::Braking(-1, self.signal);
                println!("[ATO] {:?}→{:?}", self.status, status);
                self.status = status;
            }
            5 => { // 力行OFF
                if self.before_speed < 35.0 {
                    return;
                }
                let status = ATOStatus::PowerOff(self.signal);
                println!("[ATO] {:?}→{:?}", self.status, status);
                self.status = status;
            }
            6 => { // 90パターン
                let status = ATOStatus::TASC90(self.before_time, f32::NAN , 600.5);
                println!("[ATO] {:?}→{:?}", self.status, status);
                self.status = status;
            }
            _ => {}
        }
    }
}

impl ULineATO {
    fn ato_constant_speed(&mut self, state: AtsVehicleState) -> AtsHandles {
        let delta = state.time - self.before_time;
        let acceleration_km_h_s = (state.speed - self.before_speed) / (delta as f32 / 1000.0);
        // let acceleration_acceleration = (acceleration_km_h_s - self.before_acceleration) / (delta as f32 / 1000.0);

        let atc_brake =  state.speed > self.signal.getSpeed() as f32;
        let target_speed = self.signal.getSpeed() - 3; // ATO目標速度
        let target_relative_speed = state.speed - target_speed as f32; // ATO目標速度からの相対速度

        // ATCブレーキチェック
        if atc_brake {
            self.now_power = 0;
        }

        if atc_brake {
            return AtsHandles {
                power: 0,
                brake: 0,
                reverser: 1,
                constant_speed: AtsConstantSpeed::Disable as i32
            }
        }
        
        const POWER: i32 = 1;
        const COASTING: i32 = 0;
        const BRAKE: i32 = -1;

        let operatable250 = || {
            let result = (state.time - self.recent_operation_time) > 250;
            result
        };

        let plus_power = |power: &mut i32, brake: &mut i32| {
            if *brake == 0 {
                *power += 1;
            } else {
                *brake -= 1;
            }
        };
        let plus_power_weak = |_power: &mut i32, brake: &mut i32| {
            if *brake == 0 {
                // *power += 1;
            } else {
                *brake -= 1;
            }
        };
        let minus_power = |power: &mut i32, brake: &mut i32| {
            if *power == 0 {
                *brake += 1;
            } else {
                *power -= 1;
            }
        };
        let minus_power_weak = |power: &mut i32, _brake: &mut i32| {
            if *power == 0 {
                // *brake += 1;
            } else {
                *power -= 1;
            }
        };
        /* match (target_relative_speed, acceleration_km_h_s, now_notch) {
            // ~ -5km/h
            (speed, acceleration, notch) if speed < -8.0 && notch == POWER => { // ~ -5km/h 力行中
                if operatable500() { println!("~ -5km/h POWER"); plus_power(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            (speed, acceleration, notch) if speed < -8.0 && notch == COASTING => { // ~ -5km/h 惰行中
                if operatable500() { println!("~ -5km/h COASTING"); plus_power(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            (speed, acceleration, notch) if speed < -8.0 && notch == BRAKE => { // ~ -5km/h 制動中
                if operatable200() { println!("~ -5km/h BRAKE"); plus_power(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }

            // -5km/h ~ -3km/h
            (speed, acceleration, notch) if -8.0 <= speed && speed < -4.0 && acceleration < 0.0 && notch == POWER => { // -5km/h ~ -3km/h 力行中 加速度=不足
                if operatable500() { println!("-5km/h ~ -3km/h POWER 不足"); plus_power_weak(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            (speed, acceleration, notch) if -8.0 <= speed && speed < -4.0 && acceleration < 0.0 && notch == COASTING => { // -5km/h ~ -3km/h 惰行中 加速度=不足
                if operatable500() { println!("-5km/h ~ -3km/h COASTING 不足"); plus_power_weak(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            (speed, acceleration, notch) if -8.0 <= speed && speed < -4.0 && acceleration < 0.0 && notch == BRAKE => { // -5km/h ~ -3km/h 制動中 加速度=不足
                if operatable500() { println!("-5km/h ~ -3km/h BRAKE 不足"); plus_power_weak(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            
            (speed, acceleration, notch) if -8.0 <= speed && speed < -4.0 && 0.0 <= acceleration && acceleration < 0.75 => { // ~ -5km/h ~ -3km/h 加速度=適正
                println!("-5km/h ~ -3km/h 適正");
            }

            (speed, acceleration, notch) if -8.0 <= speed && speed < -4.0 && 0.75 <= acceleration && notch == POWER => { // -5km/h ~ -3km/h 力行中 加速度=過剰
                if operatable500() { println!("-5km/h ~ -3km/h POWER 過剰"); minus_power(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            (speed, acceleration, notch) if -8.0 <= speed && speed < -4.0 && 0.75 <= acceleration && notch == COASTING => { // -5km/h ~ -3km/h 惰行中 加速度=過剰
                if operatable500() { println!("-5km/h ~ -3km/h COASTING 過剰"); minus_power(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            (speed, acceleration, notch) if -8.0 <= speed && speed < -4.0 && 0.75 <= acceleration && notch == BRAKE => { // -5km/h ~ -3km/h 制動中 加速度=過剰
                if operatable500() { println!("-5km/h ~ -3km/h BRAKE 過剰"); minus_power(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            
            // -3km/h ~ 0km/h
            (speed, acceleration, notch) if -4.0 <= speed && speed < 1.0 && acceleration < 0.5 => { // ~ -5km/h ~ -3km/h 加速度=不足
                if operatable1000() { println!("-3km/h ~ 0km/h 不足"); plus_power_weak(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            (speed, acceleration, notch) if -4.0 <= speed && speed < 1.0 && acceleration > 0.5 => { // ~ -5km/h ~ -3km/h 加速度=過剰
                if operatable1000() { println!("-3km/h ~ 0km/h 過剰"); minus_power(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }

            // 0km/h ~ 2km/h
            (speed, acceleration, notch) if 1.0 <= speed && speed < 3.0 && acceleration < -0.75 && notch == POWER => { // 0km/h ~ 2km/h 力行中 加速度=不足
                if operatable500() { println!("0km/h ~ 2km/h POWER 不足"); plus_power_weak(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            (speed, acceleration, notch) if 1.0 <= speed && speed < 3.0 && acceleration < -0.75 && notch == COASTING => { // 0km/h ~ 2km/h 惰行中 加速度=不足
                if operatable500() { println!("0km/h ~ 2km/h COASTING 不足"); plus_power_weak(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            (speed, acceleration, notch) if 1.0 <= speed && speed < 3.0 && acceleration < -0.75 && notch == BRAKE => { // 0km/h ~ 2km/h 制動中 加速度=不足
                if operatable500() { println!("0km/h ~ 2km/h BRAKE 不足"); plus_power_weak(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            
            (speed, acceleration, notch) if 1.0 <= speed && speed < 3.0 && -0.75 <= acceleration && acceleration < 0.0 => { // ~ 0km/h ~ 2km/h 加速度=適正
                println!("0km/h ~ 2km/h 適正");
            }

            (speed, acceleration, notch) if 1.0 <= speed && speed < 3.0 && 0.0 <= acceleration && notch == POWER => { // 0km/h ~ 2km/h 力行中 加速度=過剰
                if operatable500() { println!("0km/h ~ 2km/h POWER 過剰"); minus_power(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            (speed, acceleration, notch) if 1.0 <= speed && speed < 3.0 && 0.0 <= acceleration && notch == COASTING => { // 0km/h ~ 2km/h 惰行中 加速度=過剰
                if operatable500() { println!("0km/h ~ 2km/h COASTING 過剰"); minus_power(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            (speed, acceleration, notch) if 1.0 <= speed && speed < 3.0 && 0.0 <= acceleration && notch == BRAKE => { // 0km/h ~ 2km/h 制動中 加速度=過剰
                if operatable500() { println!("0km/h ~ 2km/h BRAKE 過剰"); minus_power(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            
            // 2km/h ~
            (speed, acceleration, notch) if 3.0 <= speed && notch == POWER => { // 2km/h ~ 力行中
                if operatable250() { println!("2km/h ~ POWER"); minus_power(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            (speed, acceleration, notch) if 3.0 <= speed && notch == COASTING => { // 2km/h ~ 惰行中
                if operatable500() { println!("2km/h ~ COASTING"); minus_power(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }
            (speed, acceleration, notch) if 3.0 <= speed && notch == BRAKE => { // 2km/h ~ 制動中
                if operatable500() { println!("2km/h ~ BRAKE"); minus_power(&mut self.now_power, &mut self.now_brake); self.recent_operation_time = state.time; } }

            _ => {
                println!("何故か通らない")
            }
        } */

        let constant = match target_relative_speed + acceleration_km_h_s * 0.5 {
            speed if speed < -5.0 => {
                if operatable250() {
                    // println!("< -8.0");
                    plus_power(&mut self.now_power, &mut self.now_brake);
                    self.recent_operation_time = state.time;
                }
                false
            }
            speed if -5.0 <= speed && speed < -3.0 => {
                if operatable250() {
                    // println!("-8.0 < < -5.0");
                    if acceleration_km_h_s <= -1.0 {
                        plus_power_weak(&mut self.now_power, &mut self.now_brake);
                    } else if acceleration_km_h_s >= 1.0 {
                        minus_power(&mut self.now_power, &mut self.now_brake);
                    }
                    self.recent_operation_time = state.time;
                }
                false
            }
            speed if -3.0 <= speed && speed < 1.5 => {
                // println!("-3.0 < < 1.0");
                if operatable250() {
                    self.recent_operation_time = state.time;
                }
                true
            }
            speed if 1.5 <= speed && speed < 3.0 => {
                // println!("1.0 < < 3.0");
                if operatable250() {
                    if acceleration_km_h_s <= -1.0 {
                        // plus_power(&mut self.now_power, &mut self.now_brake);
                    } else if acceleration_km_h_s >= 1.0 {
                        minus_power_weak(&mut self.now_power, &mut self.now_brake);
                    }
                    self.recent_operation_time = state.time;
                }
                false
            }
            speed if 3.0 <= speed => {
                // println!("3.0 <");
                if operatable250() {
                    minus_power(&mut self.now_power, &mut self.now_brake);
                    self.recent_operation_time = state.time;
                }
                false
            }
            _ => { false }
        };

        self.now_power = self.now_power.clamp(0, 31);
        self.now_brake = self.now_brake.clamp(0, 31);
        if constant {
            self.now_brake = 0;
            self.now_power = 0;
        }

        AtsHandles {
            power: self.now_power,
            brake: self.now_brake,
            reverser: 1,
            constant_speed: if constant { AtsConstantSpeed::Enable as i32 } else { AtsConstantSpeed::Disable as i32 }
        }
    }
    fn ato_tasc(&mut self, state: AtsVehicleState, pattern_start_time: i32, pattern_start_speed: f32) -> AtsHandles {
        let passage_time = state.time - pattern_start_time;
        let target_speed = pattern_start_speed - (passage_time as f32 / 1000.0) * 2.5;

        println!("{target_speed}");

        let output_deceleration = 2.5 - (target_speed - state.speed) / 2.0;
        let output_brake = output_deceleration / 3.5 * 7.0;

        self.now_power = 0;
        self.now_brake = (output_brake.floor() as i32).clamp(0, 31);

        AtsHandles {
            power: self.now_power,
            brake: self.now_brake,
            reverser: 1,
            constant_speed: AtsConstantSpeed::Disable as i32
        }
    }
    fn ato_tasc_with_distance(&mut self, state: AtsVehicleState, remaining_distance: f32) -> AtsHandles {
        const BRAKE_NOTCH_COUNT: i32 = 31;
        const MAX_DECELERATION: f32 = 3.50;

        /* let target_speed = 
            (7.2 * 2.4 * remaining_distance).sqrt(); */
        let target_speed = self.ato_tasc_target_speed(remaining_distance);
        
        if target_speed.is_nan() {
            return self.before_ato_notch;
        }
        if target_speed > state.speed + 5.0 {
            return self.ato_constant_speed(state);
        }
        self.is_not_one_time_braking = true;

        let mut output_deceleration = (state.speed.powi(2)/(2.0*remaining_distance)) * (1000.0/3600.0);
        if state.speed > 5.0 {
            output_deceleration = (1.0 / 7.2) * ((state.speed.powi(2) - self.ato_tasc_target_speed(remaining_distance / 2.0).powi(2)) / (remaining_distance / 2.0));
        }
        let mut output_brake = output_deceleration / MAX_DECELERATION * BRAKE_NOTCH_COUNT as f32; 

        { // 速度超過時のブレーキ補填
            output_brake -= ((target_speed - state.speed) / 1.0).clamp(-10.0, 10.0);
        }

        self.now_power = 0;
        self.now_brake = (output_brake.round() as i32).clamp(0, 31);

        AtsHandles {
            power: self.now_power,
            brake: self.now_brake,
            reverser: 1,
            constant_speed: AtsConstantSpeed::Disable as i32
        }
    }
    
    /// 残距離とTASCパターンから目標速度を求める関数
    fn ato_tasc_target_speed(&self, remaining_distance: f32) -> f32 {
        return (7.2 * 2.30 * remaining_distance).sqrt();

        match self.status {
            ATOStatus::TASC90(_, _, _) => {
                let deceleration = 2.00;
                (7.2 * deceleration * (33.625 + remaining_distance)).sqrt()
            }
            ATOStatus::TASC1(_, _, _) => {
                let deceleration = 2.25;
                (7.2 * deceleration * (-8.500 + remaining_distance)).sqrt()
            }
            ATOStatus::TASC2(_, _, _) | 
            ATOStatus::P3(_, _, _) => {
                let deceleration = 1.50;
                (7.2 * deceleration * remaining_distance).sqrt()
            }
            _ => 0.0
        }
    }
}