// 用于生成新实体的模块
use crate::prelude::*;

pub fn spawn_player(ecs: &mut World,pos: Point) {
    ecs.push((Player,pos,Render{
        color:ColorPair::new(WHITE, BLACK),
        glyph: to_cp437('@'),
    }));
}