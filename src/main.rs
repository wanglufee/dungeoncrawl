pub mod map;
pub mod map_builder;
pub mod player;

// 顶层模块，全局可见
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
}

use map::Map;
use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self{
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);     // 获取玩家位置
        self.map.render(ctx);       // 渲染地图
        self.player.render(ctx);        // 渲染玩家
    }
}

fn main() -> BError{
    let context = BTermBuilder::simple80x50()
                .with_title("Dungeon Crawler")
                .with_fps_cap(30.0)
                .build()?;
    main_loop(context, State::new())
}
