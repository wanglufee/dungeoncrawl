mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod system;

// 顶层模块，全局可见
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH/2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT/2;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::system::*;
}

use map::Map;
use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        // 
        let mut ecs = World::default();
        // 资源，地图等属于资源，摄像机也属于资源
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);   // 通过组件构建玩家，并添加到世界
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        Self{
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        // TODO: Execute Systems
        self.resources.insert(ctx.key);   // 将键盘输入插入资源列表
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");    // 批量绘制，执行被添加到buffer中的绘制命令。
        // TODO: Render Draw Buffer
    }
}

fn main() -> BError{
    let context = BTermBuilder::new()
                .with_title("Dungeon Crawler")
                .with_fps_cap(30.0)
                .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
                .with_tile_dimensions(32, 32)
                .with_resource_path("resources/")
                .with_font("dungeonfont.png", 32, 32)
                .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
                .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
                .build()?;
    main_loop(context, State::new())
}
