use super::*;

unsafe extern "C" fn palutena_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
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
        return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
    }
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_n_start_spd_x_mul = 0.8;
    let final_speed_x = sum_speed_x * special_n_start_spd_x_mul;
    let special_n_brake_spd_x = 0.02;
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if situation_kind != *SITUATION_KIND_AIR {
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_STOP as u64));
        fighter.push_lua_stack(&mut L2CValue::new_int(*ENERGY_STOP_RESET_TYPE_GROUND as u64));
        fighter.push_lua_stack(&mut L2CValue::new_num(final_speed_x));
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
        WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_WORK_INT_SITUATION_PREV);
    } else {
        fighter.clear_lua_stack();
        fighter.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_STOP as u64));
        fighter.push_lua_stack(&mut L2CValue::new_int(*ENERGY_STOP_RESET_TYPE_AIR as u64));
        fighter.push_lua_stack(&mut L2CValue::new_num(final_speed_x));
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
        if sum_speed_y < 0.0 {
            sum_speed_y = 0.0;
        }
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
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    return 0.into();
}

unsafe extern "C" fn palutena_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_TRANS_LOOP);
    helper::FUN_71000260f0(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(palutena_speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn palutena_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_trans_loop = WorkModule::is_flag(fighter.module_accessor, FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_TRANS_LOOP);
    if is_trans_loop {
        let check_button_off = ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
        if !check_button_off {
            fighter.change_status(FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_LOOP.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_END.into(), true.into());
        }
        return 0.into();
    }
    
    let is_changing = StatusModule::is_changing(fighter.module_accessor);
    if !is_changing {
        if fighter.global_table[0x17] != *SITUATION_KIND_GROUND {
            if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
                helper::FUN_71000260f0(fighter);
            }
        } else {
            helper::FUN_71000260f0(fighter);
        }
    }

    return 0.into();
}

unsafe extern "C" fn palutena_speciallw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
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
        return smashline::original_status(Exit, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
    }
    0.into()
}

unsafe extern "C" fn palutena_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
    }
    0.into()
}

unsafe extern "C" fn palutena_game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_TRANS_LOOP);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        if 60 > WorkModule::get_int(agent.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_CHARGE_FRAME_SPECIAL_LW) {
            macros::ATTACK(agent, 0, 0, Hash40::new("havel"), 10.0, 361, 55, 0, 40, 2.7, 0.0, -0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("havel"), 10.0, 361, 55, 0, 40, 2.7, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("havel"), 10.0, 361, 58, 0, 45, 3.8, 0.0, 8.5, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);    
        } else {
            macros::ATTACK(agent, 0, 0, Hash40::new("havel"), 17.0, 361, 75, 0, 40, 3.7, 0.0, -0.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("havel"), 17.0, 361, 75, 0, 40, 3.7, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 2, 0, Hash40::new("havel"), 18.0, 361, 78, 0, 45, 5.3, 0.0, 9.5, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);    
        }
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 2, 3.8);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn palutena_effect_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 6, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_palutena_wand1"), Hash40::new("tex_palutena_wand2"), 8, Hash40::new("stick"), 0, -6.0, 0, Hash40::new("stick"), 0.0, 8.0, 0.0, true, Hash40::new("null"), Hash40::new("null"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn palutena_sound_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_03"));
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_peach_attack05")); //TODO
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_peach_smash_s01"));
    }
}

unsafe extern "C" fn palutena_expression_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("smash_item") as i64, hash40("smash_item_none") as i64);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

pub fn install() {
    Agent::new("palutena")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, palutena_speciallw_pre)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, palutena_speciallw_init)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, palutena_speciallw_main)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, palutena_speciallw_exec)
        .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, palutena_speciallw_exit)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, palutena_speciallw_end)
        .game_acmd("game_speciallw_moon", palutena_game_speciallw, Default)
        .effect_acmd("effect_speciallw_moon", palutena_effect_speciallw, Default)
        .sound_acmd("sound_speciallw_moon", palutena_sound_speciallw, Default)
        .expression_acmd("expression_speciallw_moon", palutena_expression_speciallw, Default)
        .install();
}