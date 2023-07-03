use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
		hash40
    },
    smash_script::*,
    smashline::*,
	std::f32::consts::E
};

#[acmd_script( agent = "palutena", script = "effect_appealhil", category = ACMD_EFFECT )]
unsafe fn palutena_appealhil_fx(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "palutena", script = "effect_appealhir", category = ACMD_EFFECT )]
unsafe fn palutena_appealhir_fx(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "palutena", script = "effect_appeallwl", category = ACMD_EFFECT )]
unsafe fn palutena_appeallwl_fx(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "palutena", script = "effect_appeallwr", category = ACMD_EFFECT )]
unsafe fn palutena_appeallwr_fx(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "palutena", script = "effect_wait2", category = ACMD_EFFECT )]
unsafe fn palutena_wait2_fx(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "palutena", script = "effect_wait3", category = ACMD_EFFECT )]
unsafe fn palutena_wait3_fx(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "palutena", script = "effect_specials", category = ACMD_EFFECT )]
unsafe fn palutena_specials_fx(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "palutena", script = "effect_specialairs", category = ACMD_EFFECT )]
unsafe fn palutena_specialairs_fx(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "palutena", script = "effect_attacks4", category = ACMD_EFFECT )]
unsafe fn palutena_attacks4_fx(fighter: &mut L2CAgentBase) {}

#[acmd_script( agent = "palutena", script = "effect_specialhi", category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_specialhi_fx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_teleport_end"), Hash40::new("rot"), 0, 4, 0, 0, 0, 0, 1, false);
        macros::FLASH(fighter, 1, 1, 1, 1);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 0.25, 1, 1, 0.2);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 1, 1, 1, 0.15);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::COL_NORMAL(fighter);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "palutena", script = "effect_specialhistart", category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_specialhistart_fx(fighter: &mut L2CAgentBase) {
	wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FLW_UNSYNC_VIS(fighter, Hash40::new("palutena_teleport_start"), Hash40::new("rot"), 0, 1.5, 0, 0, 0, 0, 1, false);
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::FLASH(fighter, 1, 1, 1, 0.4);
	}
	wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::FLASH(fighter, 0.25, 1, 1, 0.2);
	}
	wait(fighter.lua_state_agent, 8.0);
	if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::FLASH(fighter, 1, 1, 1, 0.3);
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::FLASH(fighter, 1, 1, 1, 1);
	}
	wait(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::COL_NORMAL(fighter);
	}
}

#[acmd_script( agent = "palutena", script = "effect_specialairhi", category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_specialairhi_fx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_teleport_end"), Hash40::new("rot"), 0, 4, 0, 0, 0, 0, 1, false);
        macros::FLASH(fighter, 1, 1, 1, 1);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 0.25, 1, 1, 0.2);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 1, 1, 1, 0.15);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_teleport_feather"), Hash40::new("rot"), 0, 4, 0, 0, 0, 0, 1, true);
    }
}

#[acmd_script( agent = "palutena", script = "effect_specialairhistart", category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_specialairhistart_fx(fighter: &mut L2CAgentBase) {
	wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FLW_UNSYNC_VIS(fighter, Hash40::new("palutena_teleport_start"), Hash40::new("rot"), 0, 1.5, 0, 0, 0, 0, 1, false);
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::FLASH(fighter, 1, 1, 1, 0.4);
	}
	wait(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::FLASH(fighter, 0.25, 1, 1, 0.2);
	}
	wait(fighter.lua_state_agent, 9.0);
	if macros::is_excute(fighter) {
		macros::FLASH(fighter, 1, 1, 1, 0.3);
	}
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::FLASH(fighter, 1, 1, 1, 1);
	}
	wait(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::COL_NORMAL(fighter);
	}
}

#[acmd_script( agent = "palutena", script = "effect_final", category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_final_fx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_palutena_final"), false, false, false);
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FLW_POS(fighter, Hash40::new("palutena_final_ripple"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else{
        if macros::is_excute(fighter) {
            macros::EFFECT_FLW_POS(fighter, Hash40::new("palutena_final_ripple"), Hash40::new("footl"), 0, 0, 0, 0, 180, 0, 1, true);
        }
    }
}
if macros::is_excute(fighter) {
    macros::EFFECT_FLW_POS(fighter, Hash40::new("palutena_final_flash"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
}
if macros::is_excute(fighter) {
    macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_final_wand_light"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
}
frame(fighter.lua_state_agent, 135.0);
if macros::is_excute(fighter) {
    macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_final_gliderjump"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.5, false);
}
}

#[acmd_script( agent = "palutena", script = "effect_finalair", category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_finalair_fx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        EffectModule::req_screen(fighter.module_accessor, Hash40::new("bg_palutena_final"), false, false, false);
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FLW_POS(fighter, Hash40::new("palutena_final_ripple"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else{
        if macros::is_excute(fighter) {
            macros::EFFECT_FLW_POS(fighter, Hash40::new("palutena_final_ripple"), Hash40::new("footl"), 0, 0, 0, 0, 180, 0, 1, true);
        }
    }
}
if macros::is_excute(fighter) {
    macros::EFFECT_FLW_POS(fighter, Hash40::new("palutena_final_flash"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 1, true);
}
frame(fighter.lua_state_agent, 135.0);
if macros::is_excute(fighter) {
    macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_final_gliderjump"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.5, false);
}
}

#[acmd_script( agent = "palutena", script = "effect_win1a", category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_win1a_fx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_wand_light3"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_win_twinkle"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 120.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("palutena_win_twinkle"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("palutena_wand_light3"), false, false);
    }
}

#[acmd_script( agent = "palutena", script = "effect_win1b", category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_win1b_fx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_wand_light3"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("palutena_win_twinkle"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 120.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("palutena_win_twinkle"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("palutena_wand_light3"), false, false);
    }
}

#[acmd_script( agent = "palutena", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn palutena_specialhi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 0, 60, 60, 15.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
	}
}

#[acmd_script( agent = "palutena", script = "game_specialhistart", category = ACMD_GAME, low_priority )]
unsafe fn palutena_specialhistart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 0, 60, 60, 15.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
	}
	frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "palutena", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn palutena_specialairhi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_DIVE);
    }
	frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 0, 60, 60, 15.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
	}
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_CONTROL_ON);
    }
}

#[acmd_script( agent = "palutena", script = "game_specialairhistart", category = ACMD_GAME, low_priority )]
unsafe fn palutena_specialairhistart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
	if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 0, 60, 60, 15.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
	}
	frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "palutena", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn palutena_specialn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        shield!(fighter, MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 9.5, 0, 8.5, 8, 0, 8.5, 8, 1.8, 2.4, 80, false, 0.8, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    macros::FT_MOTION_RATE(fighter, 0.75);
    frame(fighter.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 160, 40, 0, 12, 8.5, 0.0, 7.0, -0.5, Some(0.0), Some(7.0), Some(0.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 160, 40, 0, 12, 4.0, 0.0, 8.0, -10.0, Some(0.0), Some(8.0), Some(10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 100, 0, 50, 7.0, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(4.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 100, 0, 40, 5.0, 0.0, 8.0, -11.0, Some(0.0), Some(8.0), Some(11.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        ShieldModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "palutena", script = "game_specialairn", category = ACMD_GAME, low_priority )]
unsafe fn palutena_specialairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        shield!(fighter, MA_MSC_CMD_REFLECTOR, COLLISION_KIND_REFLECTOR, 0, hash40("top"), 9.5, 0, 8.5, 8, 0, 8.5, 8, 1.8, 2.4, 80, false, 0.8, FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
	}
    macros::FT_MOTION_RATE(fighter, 0.75);
    frame(fighter.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 160, 40, 0, 12, 8.5, 0.0, 7.0, -0.5, Some(0.0), Some(7.0), Some(0.5), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 160, 40, 0, 12, 4.0, 0.0, 8.0, -10.0, Some(0.0), Some(8.0), Some(10.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 100, 0, 50, 7.0, 0.0, 8.0, -4.0, Some(0.0), Some(8.0), Some(4.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 100, 0, 40, 5.0, 0.0, 8.0, -11.0, Some(0.0), Some(8.0), Some(11.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        ShieldModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "palutena", script = "effect_specialn", category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_specialn_fx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
	for _ in 0..20 {
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_magic_s"), Hash40::new("top"), 0, 10.0, 0, 0.0, 0.0, 0.0, 0.7, true);
		macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
	}
	wait(fighter.lua_state_agent, 1.0);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
}

#[acmd_script( agent = "palutena", script = "effect_specialairn", category = ACMD_EFFECT, low_priority )]
unsafe fn palutena_specialairn_fx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
	for _ in 0..20 {
	wait(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_magic_s"), Hash40::new("top"), 0, 10.0, 0, 0.0, 0.0, 0.0, 0.7, true);
		macros::LAST_EFFECT_SET_RATE(fighter, 1.3);
	}
	wait(fighter.lua_state_agent, 1.0);
	}
	frame(fighter.lua_state_agent, 12.0);
	if macros::is_excute(fighter) {
		macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
}

#[acmd_script( agent = "palutena", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME )]
unsafe fn palutena_speciallw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        DamageModule::add_damage(fighter.module_accessor, 7.5/(1.0 + E.powf(-0.06 * (DamageModule::damage(fighter.module_accessor, 0) - 100.0))) * -1.0, 0);
		ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD, false, -1);
    }
}

#[acmd_script( agent = "palutena", scripts = ["effect_speciallw", "effect_specialairlw"], category = ACMD_EFFECT )]
unsafe fn palutena_speciallw_fx(fighter: &mut L2CAgentBase) {}


#[acmd_script( agent = "palutena_reflectionboard", script = "game_shoot", category = ACMD_GAME, low_priority )]
unsafe fn palutena_reflectionboard_shoot(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 30, 0, 0, 1.0, 0.0, 0.0, -10.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 20, 0, 0, 1.0, 0.0, 0.0, -10.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
		AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_ALL as u32);
		AttackModule::set_target_category(fighter.module_accessor, 1, *COLLISION_CATEGORY_MASK_ALL as u32);
		AttackModule::set_no_team(fighter.module_accessor, true);
    }
	wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
		palutena_appealhil_fx,
		palutena_appealhir_fx,
		palutena_appeallwl_fx,
		palutena_appeallwr_fx,
		palutena_attacks4_fx,
		palutena_final_fx,
		palutena_finalair_fx,
		palutena_reflectionboard_shoot,
		palutena_specialairhi,
		palutena_specialairhi_fx,
		palutena_specialairhistart,
		palutena_specialairhistart_fx,
		palutena_specialairn,
		palutena_specialairn_fx,
		palutena_specialairs_fx,
		palutena_specialhi,
		palutena_specialhi_fx,
		palutena_specialhistart,
		palutena_specialhistart_fx,
		palutena_speciallw,
		palutena_speciallw_fx,
		palutena_specialn,
		palutena_specialn_fx,
		palutena_specials_fx,
		palutena_wait2_fx,
		palutena_wait3_fx,
		palutena_win1a_fx,
		palutena_win1b_fx,
    );
}