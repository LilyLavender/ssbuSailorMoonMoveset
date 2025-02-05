use super::*;

pub mod acmd;
pub mod status;
pub mod fly;
pub mod hit;
pub mod vanish;

pub fn install() {
    acmd::install();
    status::install();
    fly::install();
    hit::install();
    vanish::install();
}