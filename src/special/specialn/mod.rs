use super::*;

// ACMD
unsafe extern "C" fn palutena_game_specialn(agent: &mut L2CAgentBase) {
	macros::FT_MOTION_RATE(agent, 0.75);
	frame(agent.lua_state_agent, 13.0);
	macros::FT_MOTION_RATE(agent, 1.0);
	if macros::is_excute(agent) {
		shield!(agent, MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 9.5, 0, 8.5, 8, 0, 8.5, 8, 1.8, 2.4, 80, false, 0.8, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 160, 40, 0, 12, 8.5, 0.0, 7.0, -0.5, Some(0.0), Some(7.0), Some(0.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 160, 40, 0, 12, 4.0, 0.0, 8.0, -10.0, Some(0.0), Some(8.0), Some(10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
	}
	frame(agent.lua_state_agent, 25.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
	frame(agent.lua_state_agent, 28.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 100, 0, 50, 7.0, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(4.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
		macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 361, 100, 0, 40, 5.0, 0.0, 8.0, -11.0, Some(0.0), Some(8.0), Some(11.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
		shield!(agent, MA_MSC_CMD_SHIELD_OFF,*COLLISION_KIND_REFLECTOR, 0, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
	}
}

unsafe extern "C" fn palutena_effect_specialn(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 12.0);
	if macros::is_excute(agent) {
		if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
	    	macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		} else {
			macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		}
	}
	frame(agent.lua_state_agent, 13.0);
	if macros::is_excute(agent) {
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.0, 0, 0, 0, 0, 1.0, true);
	}
}

unsafe extern "C" fn palutena_effect_specialairn(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 13.0);
	if macros::is_excute(agent) {
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 10.0, 0, 0, 0, 0, 1.0, true);
	}
}

// STATUS
unsafe extern "C" fn palutena_specialn_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
    FUN_7100009d40(fighter);

    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
    fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
    fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
    sv_kinetic_energy::controller_set_accel_x_add(fighter.lua_state_agent);

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_MOTION as u64));
    fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
    fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    return 0.into();
}

unsafe extern "C" fn FUN_7100009d40(fighter: &mut L2CFighterCommon) {
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
      
    //let special_n_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, Hash40::new("param_special_n"), Hash40::new("special_n_speed_x_mul"));
    //sum_speed_x = sum_speed_x * special_n_speed_x_mul;

    let energy_reset_type = if fighter.global_table[0x16] == *SITUATION_KIND_AIR { *ENERGY_STOP_RESET_TYPE_AIR } else { *ENERGY_STOP_RESET_TYPE_GROUND };

    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_STOP as u64));
    fighter.push_lua_stack(&mut L2CValue::new_int(energy_reset_type as u64));
    fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
    fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
    fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
    fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
    fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_STOP as u64));
    fighter.push_lua_stack(&mut L2CValue::new_num(sum_speed_x));
    fighter.push_lua_stack(&mut L2CValue::new_num(0.0));
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
}

unsafe extern "C" fn palutena_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
	fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_NONE),  
        *FIGHTER_KINETIC_TYPE_UNIQ,
        GROUND_CORRECT_KIND_KEEP.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true, 
        FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG.into(), 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, 
        FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT.into(), 
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 
        FIGHTER_STATUS_ATTR_START_TURN.into(), 
        FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N.into(), 
        0
    );
    0.into()
}

pub fn install() {
    Agent::new("palutena")
        .game_acmd("game_specialn_moon", palutena_game_specialn, Default)
        .effect_acmd("effect_specialn_moon", palutena_effect_specialn, Default)
        .game_acmd("game_specialairn_moon", palutena_game_specialn, Default)
        .effect_acmd("effect_specialairn_moon", palutena_effect_specialairn, Default)
		.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, palutena_specialn_init)
		.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, palutena_specialn_pre)
        .install();
}
