use super::*;

pub mod speciallw;
pub mod speciallwloop;
pub mod speciallwend;
pub mod helper;

pub fn install() {
    speciallw::install();
    speciallwloop::install();
    speciallwend::install();
}