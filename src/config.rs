#![allow(unused_variables)]
#![allow(dead_code)]

use mipidsi::{models::ILI9225Rgb565, options::Rotation};
// DisplaySetting
pub const ROTATION: Rotation = Rotation::Deg270;
pub const MODEL: ILI9225Rgb565 = ILI9225Rgb565;

pub const LCD_WIDTH: usize = 220;
pub const LCD_HEIGHT: usize = 176;
pub const LCD_BUFFER_SIZE: usize = LCD_WIDTH * LCD_HEIGHT;