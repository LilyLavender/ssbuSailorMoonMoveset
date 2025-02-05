use super::*;

unsafe extern "C" fn palutena_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Pre, fighter, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_ATTACK)(fighter);
    }
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_NONE),  
        *FIGHTER_KINETIC_TYPE_UNIQ,
        GROUND_CORRECT_KIND_KEEP.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true, 
        0.into(), 
        0, 
        0.into(), 
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 
        FIGHTER_STATUS_ATTR_START_TURN.into(), 
        FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW.into(), 
        0
    );
    0.into()
}

unsafe extern "C" fn palutena_speciallw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Init, fighter, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_ATTACK)(fighter);
    }
    return 0.into();
}

unsafe extern "C" fn palutena_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Main, fighter, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_ATTACK)(fighter);
    }
    helper::FUN_71000260f01(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(palutena_speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn palutena_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_end = MotionModule::is_end(fighter.module_accessor);
    if is_end {
        let status = if fighter.global_table[0x16] == *SITUATION_KIND_GROUND { FIGHTER_STATUS_KIND_WAIT } else { FIGHTER_STATUS_KIND_FALL };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }

    let is_changing = StatusModule::is_changing(fighter.module_accessor);
    if !is_changing {
        if fighter.global_table[0x17] != *SITUATION_KIND_GROUND {
            if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
                helper::FUN_71000260f01(fighter);
            }
        } else {
            helper::FUN_71000260f01(fighter);
        }
    }

    return 0.into();
}

unsafe extern "C" fn palutena_speciallw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Exec, fighter, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_ATTACK)(fighter);
    }
    let situation_prev = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_WORK_INT_SITUATION_PREV);
    let situation = StatusModule::situation_kind(fighter.module_accessor);
    if situation != situation_prev {
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let special_n_brake_spd_x = 0.02;
        if situation != SITUATION_KIND_AIR {
            fighter.clear_lua_stack();
            fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_STOP as u64));
            fighter.push_lua_stack(&mut L2CValue::new_int(*ENERGY_STOP_RESET_TYPE_GROUND as u64));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

            fighter.clear_lua_stack();
            fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_STOP as u64));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
            
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_WORK_INT_SITUATION_PREV);
        } else {
            fighter.clear_lua_stack();
            fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_STOP as u64));
            fighter.push_lua_stack(&mut L2CValue::new_int(*ENERGY_STOP_RESET_TYPE_AIR as u64));
            fighter.push_lua_stack(&mut L2CValue::new_num(sum_speed_x));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            
            fighter.clear_lua_stack();
            fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_STOP as u64));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);

            fighter.clear_lua_stack();
            fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_STOP as u64));
            fighter.push_lua_stack(&mut L2CValue::new_num(special_n_brake_spd_x));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            sv_kinetic_energy::set_brake(fighter.lua_state_agent);

            fighter.clear_lua_stack();
            fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_STOP as u64));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);

            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            fighter.clear_lua_stack();
            fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY as u64));
            fighter.push_lua_stack(&mut L2CValue::new_int(*ENERGY_GRAVITY_RESET_TYPE_GRAVITY as u64));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            fighter.push_lua_stack(&mut L2CValue::new_num(sum_speed_y));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_WORK_INT_SITUATION_PREV);
        }
    }
    return 0.into();
}

unsafe extern "C" fn palutena_speciallw_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Exit, fighter, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_ATTACK)(fighter);
    }
    0.into()
}

unsafe extern "C" fn palutena_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(End, fighter, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_ATTACK)(fighter);
    }
    0.into()
}

pub fn install() {
    Agent::new("palutena")
        .status(Pre, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_END, palutena_speciallw_pre)
        .status(Init, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_END, palutena_speciallw_init)
        .status(Main, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_END, palutena_speciallw_main)
        .status(Exec, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_END, palutena_speciallw_exec)
        .status(Exit, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_END, palutena_speciallw_exit)
        .status(End, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_END, palutena_speciallw_end)
        .install();
}
