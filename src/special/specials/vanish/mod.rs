use super::*;

unsafe extern "C" fn palutena_tiara_vanish_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR),  
        *WEAPON_KINETIC_TYPE_NORMAL,
        GROUND_CORRECT_KIND_AIR.into(), 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    0.into()
}

unsafe extern "C" fn palutena_tiara_vanish_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("vanish"), 0.0, 1.0, false, 0.0, false, false);
    EffectModule::detach_all(weapon.module_accessor, 5);
    // Fastshift
    weapon.fastshift(L2CValue::Ptr(palutena_tiara_vanish_main_loop as *const () as _))
}

unsafe extern "C" fn palutena_tiara_vanish_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // If at least one frame into vanish status, kill projectile
    let status_frame = weapon.global_table[0xe].get_f32();
    if status_frame > 1.0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn palutena_tiara_vanish_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("palutena_tiara")
        .status(Pre, *WEAPON_LINK_SWORDBEAM_STATUS_KIND_VANISH, palutena_tiara_vanish_pre)
        .status(Main, *WEAPON_LINK_SWORDBEAM_STATUS_KIND_VANISH, palutena_tiara_vanish_main)
        .status(End, *WEAPON_LINK_SWORDBEAM_STATUS_KIND_VANISH, palutena_tiara_vanish_end)
        .install();
}