
/// 一定間隔で処理を行うタイマーを表す
#[derive(Debug, Default)]
pub struct Timer {
	/// 発火間隔
	interval: i32,
	/// 最後に発火した時刻
	last_elapsed: i32,
}

impl Timer {
	pub fn new(interval: i32) -> Self {
		Self {
			interval,
			last_elapsed: 0,
		}
	}

	pub fn is_ready(&mut self, now_time: i32) -> bool {
		if (self.last_elapsed + self.interval) <= now_time {
			self.last_elapsed = now_time;
			true
		} else {
			false
		}
	}
}