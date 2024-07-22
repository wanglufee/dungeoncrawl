// 处理玩家输入系统
use crate::prelude::*;

#[system]
#[write_component(Point)]   // 为一个组件申请写权限
#[read_component(Player)]   // 为一个组件申请读权限
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    // 读取键盘输入
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };
        if delta !=Point::zero() {
            // 通过列出组件，查询出该类型组件的实例，使用&mut 则会返回可变引用
            let mut players = <&mut Point>::query()
                                            .filter(component::<Player>());
            // 对世界中查询到的实体进行遍历
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            })
        }
    }
}