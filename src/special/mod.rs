use super::*;

pub mod specialn;
pub mod specials;
pub mod specialhi;
pub mod speciallw;

pub fn install() {
    specialn::install();
    specials::install();
    specialhi::install();
    speciallw::install();
}