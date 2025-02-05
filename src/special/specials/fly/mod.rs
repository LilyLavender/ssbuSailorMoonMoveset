use super::*;

unsafe extern "C" fn palutena_tiara_fly_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
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

unsafe extern "C" fn palutena_tiara_fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // Change motion
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    
    // Set life
    WorkModule::set_int(weapon.module_accessor, 180, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, 180, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    
    // Set speed & accel
    let facing = PostureModule::lr(weapon.module_accessor);
    let speed_x: f32 = if facing == 1.0 { 2.8 } else { -2.8 };
    let facing = PostureModule::lr(weapon.module_accessor);
    let deccel_x: f32 = if facing == 1.0 { -0.05 } else { 0.05 };
    
    weapon.agent.clear_lua_stack();
    weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.agent.push_lua_stack(&mut L2CValue::new_num(speed_x));
    weapon.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);
    
    weapon.agent.clear_lua_stack();
    weapon.agent.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.agent.push_lua_stack(&mut L2CValue::new_num(deccel_x));
    weapon.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
    sv_kinetic_energy::set_accel(weapon.lua_state_agent);
    
    // Set scale
    PostureModule::set_scale(weapon.module_accessor, 0.8, false);
    
    // Assign substatus
    weapon.global_table[0x15].assign(&L2CValue::Ptr(palutena_tiara_fly_substatus as *const () as _));
    
    // Fastshift
    weapon.fastshift(L2CValue::Ptr(palutena_tiara_fly_main_loop as *const () as _))
}

unsafe extern "C" fn palutena_tiara_fly_substatus(weapon: &mut L2CWeaponCommon) {
    // Decrement life each frame
    let is_stop = StopModule::is_stop(weapon.module_accessor);
    if !is_stop {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
}

unsafe extern "C" fn palutena_tiara_fly_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // Transition into vanish if life expires
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 {
        weapon.change_status(WEAPON_LINK_SWORDBEAM_STATUS_KIND_VANISH.into(), false.into());
    }
    
    // Transition into hit if touching ground*
    let is_touch = GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32);
    if is_touch {
        weapon.change_status(WEAPON_LINK_SWORDBEAM_STATUS_KIND_HIT.into(), false.into());
    }
    
    0.into()
}

unsafe extern "C" fn palutena_tiara_fly_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("palutena_tiara")
        .status(Pre, *WEAPON_LINK_SWORDBEAM_STATUS_KIND_FLY, palutena_tiara_fly_pre)
        .status(Main, *WEAPON_LINK_SWORDBEAM_STATUS_KIND_FLY, palutena_tiara_fly_main)
        .status(End, *WEAPON_LINK_SWORDBEAM_STATUS_KIND_FLY, palutena_tiara_fly_end)
        .install();
}