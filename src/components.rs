// 组件模块，实现组件，用来编排实体
pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,   //  前景色和背景色
    pub glyph: FontCharType,   // 存储单个字符
}

#[derive(Debug,Clone, Copy,PartialEq)]
pub struct Player;