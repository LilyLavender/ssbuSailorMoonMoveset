use super::*;

// STATUS
unsafe extern "C" fn palutena_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    }
    return 0.into();
}

unsafe extern "C" fn palutena_specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    }
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, 
        FIGHTER_STATUS_ATTR_START_TURN.into(), 
        FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S.into(), 
        0
    );
    0.into()
}

unsafe extern "C" fn palutena_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_moon(fighter.module_accessor) {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);
    }
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.sub_shift_status_main(L2CValue::Ptr(palutena_specials_main_loop as *const () as _))
}

unsafe extern "C" fn palutena_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Transing into fall/wait
    let is_changing = StatusModule::is_changing(fighter.module_accessor);
    if !is_changing {
        let is_end = MotionModule::is_end(fighter.module_accessor);
        if is_end {
            if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            return 1.into();
        }
    }
    
    // End if any conditions are true
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    let is_enable_cancel = CancelModule::is_enable_cancel(fighter.module_accessor);
    if is_enable_cancel {
        return 1.into();
    }
    if fighter.sub_wait_ground_check_common(false.into()).get_bool()
    || fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }

    // Motion & ground correct
    let motion_ground = "special_s";
    let motion_air = "special_air_s";

    if !is_changing {
        if fighter.global_table[0x17] != *SITUATION_KIND_GROUND
        && fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
            return 0.into();
        }
    }

    let mut change_mot = "";
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        change_mot = motion_air;
    } else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        change_mot = motion_ground;
    }

    if is_changing {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new(change_mot), 0.0, 1.0, false, 0.0, false, false);
    } else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(change_mot), -1.0, 1.0, 0.0, false, false);
    }
    //return 1.into();



    //if aLStack64 {
    //    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_S_FLAG_FALLED);
    //}
    //FUN_710001b690(fighter);
    return 0.into();
}

// ACMD
unsafe extern "C" fn palutena_game_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(agent, 1.2);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_PALUTENA_GENERATE_ARTICLE_TIARA, false, -1);
    }
}

unsafe extern "C" fn palutena_game_specialairs(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(agent, 1.2);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_PALUTENA_GENERATE_ARTICLE_TIARA, false, -1);
    }
}

unsafe extern "C" fn palutena_expression_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
}

pub fn install() {
    Agent::new("palutena")
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, palutena_specials_end)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, palutena_specials_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, palutena_specials_main)
        .game_acmd("game_specials_moon", palutena_game_specials, Default)
        .game_acmd("game_specialairs_moon", palutena_game_specialairs, Default)
        .expression_acmd("expression_specials_moon", palutena_expression_specials, Default)
        .expression_acmd("expression_specialairs_moon", palutena_expression_specials, Default)
        .install();
}

