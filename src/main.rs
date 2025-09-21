#![no_std]
#![no_main]

use crate::eadk::{Color, Rect, Point};
use crate::collection::Vec;


pub mod eadk;

#[used]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 12] = *b"ArrowTracer\0";

#[used]
#[link_section = ".rodata.eadk_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 4250] = *include_bytes!("../target/icon.nwi");

struct Image {
    width: u128,
    height: u128,
    image: Vec<Color>,
}
impl Image {
    fn new(width: u128, height: u128, image: Vec<Color>) -> Self {
        if (width * height == image.length) {
            
        } 
        Self {
            width,
            height,
            image
        }
    }
}


fn draw_top_bar() {
    eadk::display::push_rect_uniform(Rect { x: 0, y: 0, width: 320, height: 30 }, Color { rgb565: 0xed87 });
}

fn main_menu() {
    clear();
    draw_top_bar();
}

fn clear() {
    eadk::display::push_rect_uniform(Rect::SCREEN_RECT, Color::WHITE);
}

#[no_mangle]
pub fn main() {
    main_menu();    
    loop {}
}
