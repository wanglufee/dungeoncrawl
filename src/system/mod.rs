mod player_input;

use player_input::player_input;

use crate::prelude::*;

// 返回执行器的桩函数
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .build()
}