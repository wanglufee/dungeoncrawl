use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

// 图块类型
#[derive(Copy,Clone,PartialEq)]
pub enum TileType {
    Wall,    // 墙体
    Floor,   // 地面
}


pub struct Map {
    pub tiles: Vec<TileType>,
}

// 计算图块所在的索引
pub fn map_idx(x: i32, y: i32) -> usize {
    (y*SCREEN_WIDTH + x) as usize
}

impl Map {
    // 创建地图
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    // 渲染地图
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, WHEAT, BLACK, to_cp437('.'));
                    },
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    },
                }
            }
        }
    }
}