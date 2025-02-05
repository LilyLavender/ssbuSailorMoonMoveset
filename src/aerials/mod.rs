use super::*;

unsafe extern "C" fn palutena_game_attackairn(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 4.0);
	if macros::is_excute(agent) {
		WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(agent.lua_state_agent, 10.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 30, 0, 4.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 90, 100, 50, 0, 4.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(agent.lua_state_agent, 2.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
	frame(agent.lua_state_agent, 18.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 30, 0, 4.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 90, 100, 50, 0, 4.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(agent.lua_state_agent, 2.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
	frame(agent.lua_state_agent, 26.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 367, 100, 30, 0, 4.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 90, 100, 50, 0, 4.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(agent.lua_state_agent, 2.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
	frame(agent.lua_state_agent, 36.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 90, 0, 25, 7.5, 0.0, 11.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(agent.lua_state_agent, 3.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
	frame(agent.lua_state_agent, 50.0);
	if macros::is_excute(agent) {
		WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn palutena_effect_attackairn(agent: &mut L2CAgentBase) {
	if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
		frame(agent.lua_state_agent, 8.0);
		if macros::is_excute(agent) {
			macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 5, 8.7, 3.5, 125, -85, -90, 1, true);
			macros::LAST_EFFECT_SET_RATE(agent, 1.3);
		}
	}
	frame(agent.lua_state_agent, 17.0);
	if macros::is_excute(agent) {
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 5, 8.7, 3.5, 140, -110, -95, 1, true);
		macros::LAST_EFFECT_SET_RATE(agent, 1.3);
	}
	frame(agent.lua_state_agent, 24.0);
	if macros::is_excute(agent) {
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 5, 8.6, 3.1, 85, -110, -70, 1, true);
		macros::LAST_EFFECT_SET_RATE(agent, 1.3);
	}
	frame(agent.lua_state_agent, 17.0);
	if macros::is_excute(agent) {
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 5, 8.7, 3.5, 140, -110, -95, 1, true);
		macros::LAST_EFFECT_SET_RATE(agent, 1.3);
	}
	frame(agent.lua_state_agent, 24.0);
	if macros::is_excute(agent) {
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 5, 8.4, 3.1, 100, -120, -90, 1, true);
		macros::LAST_EFFECT_SET_RATE(agent, 1.3);
	}
}

unsafe extern "C" fn palutena_sound_attackairn(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn palutena_game_attackairf(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 4.0);
	if macros::is_excute(agent) {
		WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(agent.lua_state_agent, 9.0);
	if macros::is_excute(agent) {
		let deg = smash::app::sv_math::rand(hash40("agent"), 50) as u64 + 20;
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, deg, 80, 0, 30, 6.5, 0.0, 6.9, 9.0, Some(0.0), Some(6.9), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
	}
	frame(agent.lua_state_agent, 12.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
	frame(agent.lua_state_agent, 24.0);
	if macros::is_excute(agent) {
		WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn palutena_game_attackairb(agent: &mut L2CAgentBase) {
	if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GODWING, Hash40::new("attack_air_b"), false, -1.0);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::SET_SPEED_EX(agent, 1.2, 0.25, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 361, 100, 0, 40, 7.5, 0.0, 10.0, -12.5, Some(0.0), Some(11.0), Some(-17.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 361, 100, 0, 40, 5.7, 0.0, 10.7, -12.5, Some(0.0), Some(12.2), Some(-21.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn palutena_effect_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
		agent.clear_lua_stack();
		lua_args!(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8.5, 0, -10, 180, 0, 1.5, true, 0.658, 0.631, 0.578);
		sv_animcmd::EFFECT_FOLLOW_COLOR(agent.lua_state_agent);
		macros::LAST_EFFECT_SET_RATE(agent, 1.3);
		macros::EFFECT(agent, Hash40::new("palutena_feather"), Hash40::new("top"), 0, 6, -3, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12.3, -21, 0, 0, 0, 1.1, true);
    }
}

unsafe extern "C" fn palutena_sound_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_palutena_rnd_smash_s"));
        macros::PLAY_SE(agent, Hash40::new("se_palutena_smash_s01"));
    }
}

unsafe extern "C" fn palutena_expression_attackairb(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_wing"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
		macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn palutena_game_attackairhi(agent: &mut L2CAgentBase) {
	if macros::is_excute(agent) {
		FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 2.0, 3.0, 3.0, 5.0);
	}
	frame(agent.lua_state_agent, 2.0);
	if macros::is_excute(agent) {
		WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(agent.lua_state_agent, 6.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 7.0, 73, 106, 0, 49, 4.0, 1.4, -3.0, -3.0, Some(1.4), Some(3.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 7.0, 73, 106, 0, 49, 5.0, 3.4, -3.0, -3.0, Some(3.4), Some(3.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	wait(agent.lua_state_agent, 5.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
	frame(agent.lua_state_agent, 30.0);
	if macros::is_excute(agent) {
		WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
}

unsafe extern "C" fn palutena_effect_attackairhi(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 5.0);
	if macros::is_excute(agent) {
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), -1, -3, 1, -90, 0, 0, 1.5, true);
		macros::LAST_EFFECT_SET_RATE(agent, 0.6);
	}
}

unsafe extern "C" fn palutena_sound_attackairhi(agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("palutena")
        .game_acmd("game_attackairn_moon", palutena_game_attackairn, Default)
        .effect_acmd("effect_attackairn_moon", palutena_effect_attackairn, Default)
        .sound_acmd("sound_attackairn_moon", palutena_sound_attackairn, Default)
        .game_acmd("game_attackairf_moon", palutena_game_attackairf, Default)
        .game_acmd("game_attackairb_moon", palutena_game_attackairb, Default)
        .effect_acmd("effect_attackairb_moon", palutena_effect_attackairb, Default)
        .sound_acmd("sound_attackairb_moon", palutena_sound_attackairb, Default)
        .expression_acmd("expression_attackairb_moon", palutena_expression_attackairb, Default)
        .game_acmd("game_attackairhi_moon", palutena_game_attackairhi, Default)
        .effect_acmd("effect_attackairhi_moon", palutena_effect_attackairhi, Default)
        .sound_acmd("sound_attackairhi_moon", palutena_sound_attackairhi, Default)
        .install();
}
