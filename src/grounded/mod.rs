use super::*;

unsafe extern "C" fn palutena_game_attack11(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 10.0);
	if macros::is_excute(agent) {
		MotionModule::set_rate(agent.module_accessor, 1.0);
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 37, 40, 0, 53, 3.5, 0.0, 11.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 37, 40, 0, 53, 3.5, 0.0, 10.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 37, 36, 0, 53, 4.0, 0.0, 10.0, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 37, 40, 0, 53, 3.5, 0.0, 11.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 37, 40, 0, 53, 3.5, 0.0, 11.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 37, 36, 0, 53, 4.0, 0.0, 11.0, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(agent.lua_state_agent, 5.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
}

unsafe extern "C" fn palutena_effect_attack11(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 11.0);
	if macros::is_excute(agent) {
		macros::EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 2, 13, 12, 0, 0, 0, 1, true, *EF_FLIP_YZ, 0.7);
		macros::LANDING_EFFECT_FLIP(agent, Hash40::new("sys_atk_smoke"), Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, -2, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
	}
}

unsafe extern "C" fn palutena_game_attacks3(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 2.0);
	macros::FT_MOTION_RATE(agent, 0.8);
	frame(agent.lua_state_agent, 14.0);
	if macros::is_excute(agent) {
		macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
		macros::ATTACK(agent, 0, 0, Hash40::new("stick"), 4.0, 361, 68, 0, 30, 4.5, 0.0, 6.0, 0.0, Some(0.0), Some(-6.5), Some(0.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
	}
	frame(agent.lua_state_agent, 20.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
		macros::ATTACK(agent, 0, 0, Hash40::new("stick"), 7.0, 106, 62, 0, 38, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(-6.0), Some(0.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
	}
	frame(agent.lua_state_agent, 25.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
		macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_NORMAL);
	}
}

unsafe extern "C" fn palutena_effect_attacks3(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
		macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
		macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_palutena_wand1"), Hash40::new("tex_palutena_wand2"), 8, Hash40::new("stick"), 0, -6.0, 0, Hash40::new("stick"), 0.0, 8.0, 0.0, true, Hash40::new("null"), Hash40::new("null"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
	}
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn palutena_sound_attacks3(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn palutena_game_attackhi3(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 10.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("stick"), 11.0, 95, 80, 0, 45, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(-6.0), Some(0.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
	}
	frame(agent.lua_state_agent, 13.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("stick"), 8.0, 92, 80, 0, 45, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(-6.0), Some(0.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
	}
	frame(agent.lua_state_agent, 19.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
}

unsafe extern "C" fn palutena_effect_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
		macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_palutena_wand1"), Hash40::new("tex_palutena_wand2"), 8, Hash40::new("stick"), 0, -8.8, 0, Hash40::new("stick"), 0.0, 8.0, 0.0, true, Hash40::new("null"), Hash40::new("null"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
	}
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn palutena_sound_attackhi3(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn palutena_game_attacklw3(agent: &mut L2CAgentBase) {
	macros::FT_MOTION_RATE(agent, 0.8);
	frame(agent.lua_state_agent, 14.0);
	macros::FT_MOTION_RATE(agent, 1.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("stick"), 9.6, 81, 100, 0, 38, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(-6.0), Some(0.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
		AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	frame(agent.lua_state_agent, 21.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
}

unsafe extern "C" fn palutena_effect_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
		macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_palutena_wand1"), Hash40::new("tex_palutena_wand2"), 8, Hash40::new("stick"), 0, -7.5, 0, Hash40::new("stick"), 0.0, 8.0, 0.0, true, Hash40::new("null"), Hash40::new("null"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
	}
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn palutena_game_attackdash(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 13.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 7.3, 45, 67, 0, 75, 3.5, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 7.3, 45, 67, 0, 75, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(agent, 2, 0, Hash40::new("colonells"), 7.3, 45, 67, 0, 75, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 7.3, 45, 67, 0, 75, 3.5, 1.0, 0.0, 6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 1.5);
	}
	frame(agent.lua_state_agent, 16.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 7.3, 45, 67, 0, 75, 3.5, 1.0, 0.0, 2.0, Some(8.0), Some(1.5), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 7.3, 45, 67, 0, 75, 3.5, 1.0, 0.0, 6.5, Some(12.0), Some(1.5), Some(-2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
	}
	frame(agent.lua_state_agent, 17.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
}

unsafe extern "C" fn palutena_effect_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_palutena_wand1"), Hash40::new("tex_palutena_wand2"), 8, Hash40::new("stick"), 0, -6.0, 0, Hash40::new("stick"), 0.0, 8.0, 0.0, true, Hash40::new("null"), Hash40::new("null"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

pub fn install() {
    Agent::new("palutena")
        .game_acmd("game_attack11_moon", palutena_game_attack11, Default)
        .effect_acmd("effect_attack11_moon", palutena_effect_attack11, Default)
        .game_acmd("game_attacks3_moon", palutena_game_attacks3, Default)
        .effect_acmd("effect_attacks3_moon", palutena_effect_attacks3, Default)
        .sound_acmd("sound_attacks3_moon", palutena_sound_attacks3, Default)
        .game_acmd("game_attackhi3_moon", palutena_game_attackhi3, Default)
        .effect_acmd("effect_attackhi3_moon", palutena_effect_attackhi3, Default)
        .sound_acmd("sound_attackhi3_moon", palutena_sound_attackhi3, Default)
        .game_acmd("game_attacklw3_moon", palutena_game_attacklw3, Default)
        .effect_acmd("effect_attacklw3_moon", palutena_effect_attacklw3, Default)
        .game_acmd("game_attackdash_moon", palutena_game_attackdash, Default)
        .effect_acmd("effect_attackdash_moon", palutena_effect_attackdash, Default)
        .install();
}
