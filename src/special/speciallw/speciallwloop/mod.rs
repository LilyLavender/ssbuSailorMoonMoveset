use super::*;

unsafe extern "C" fn palutena_speciallwloop_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Pre, fighter, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_REFLECT)(fighter);
    }
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_NONE),  
        *FIGHTER_KINETIC_TYPE_UNIQ,
        GROUND_CORRECT_KIND_KEEP.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true, 
        FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG.into(), 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, 
        FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT.into(), 
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 
        0, 
        FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW.into(), 
        0
    );
    0.into()
}

unsafe extern "C" fn palutena_speciallwloop_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn palutena_speciallwloop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Main, fighter, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_REFLECT)(fighter);
    }
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_CHARGE_FRAME_SPECIAL_LW);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    helper::FUN_7100024ec0(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(palutena_speciallwloop_main_loop as *const () as _))
}

unsafe extern "C" fn palutena_speciallwloop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let facing = PostureModule::lr(fighter.module_accessor);
    let is_off_button_special = ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
    if is_off_button_special {
        let charge_frame = WorkModule::get_int(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_CHARGE_FRAME_SPECIAL_LW);
        if charge_frame >= 60 {
            fighter.change_status(FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
        } else {
            fighter.change_status(FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
        }
        return 0.into();
    }

    let charge_frame = WorkModule::get_int(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_CHARGE_FRAME_SPECIAL_LW);
    if charge_frame >= 180 {
        fighter.change_status(FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_END.into(), false.into());
        return 0.into();
    }

    let is_changing = StatusModule::is_changing(fighter.module_accessor);
    if !is_changing {
        if fighter.global_table[0x17] != *SITUATION_KIND_GROUND {
            if fighter.global_table[0x16] == *SITUATION_KIND_GROUND {
                helper::FUN_7100024ec0(fighter);
            }
        } else {
            helper::FUN_7100024ec0(fighter);
        }
    }

    return 0.into();
}

unsafe extern "C" fn palutena_speciallwloop_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return 0.into();
    }
    WorkModule::inc_int(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_CHARGE_FRAME_SPECIAL_LW);
    0.into()
}

unsafe extern "C" fn palutena_speciallwloop_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(End, fighter, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_REFLECT)(fighter);
    }
    0.into()
}

unsafe extern "C" fn palutena_speciallwloop_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn palutena_game_speciallwloop(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 3.0);
}

unsafe extern "C" fn palutena_effect_speciallwloop(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
    macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("havel"), 0, 6, 0, 0, 0, 0, 1, 2, 2, 2, 0, 0, 0, true);
}

unsafe extern "C" fn palutena_sound_speciallwloop(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_smash_start_03"));
    }
}

unsafe extern "C" fn palutena_expression_speciallwloop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("palutena")
        .status(Pre, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_LOOP, palutena_speciallwloop_pre)
        .status(Init, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_LOOP, palutena_speciallwloop_init)
        .status(Main, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_LOOP, palutena_speciallwloop_main)
        .status(Exec, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_LOOP, palutena_speciallwloop_exec)
        .status(End, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_LOOP, palutena_speciallwloop_end)
        .status(Exit, FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_LOOP, palutena_speciallwloop_exit)
        .game_acmd("game_speciallwloop_moon", palutena_game_speciallwloop, Default)
        .effect_acmd("effect_speciallwloop_moon", palutena_effect_speciallwloop, Default)
        .sound_acmd("sound_speciallwloop_moon", palutena_sound_speciallwloop, Default)
        .expression_acmd("expression_speciallwloop_moon", palutena_expression_speciallwloop, Default)
        .install();
}
