use super::*;

unsafe extern "C" fn palutena_game_attackhi4(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 5.0);
	if macros::is_excute(agent) {
		WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(agent.lua_state_agent, 6.0);
	macros::FT_MOTION_RATE(agent, 2.4);
	frame(agent.lua_state_agent, 11.0);
	macros::FT_MOTION_RATE(agent, 1.0);
	frame(agent.lua_state_agent, 13.0);
	if macros::is_excute(agent) {
		macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 17.25, 89, 90, 0, 42, 5.8, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 17.25, 89, 90, 0, 42, 4.6, 0.0, 0.0, 7.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 17.25, 89, 90, 0, 42, 5.8, 0.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
		macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 125, 100, 155, 0, 5.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(-9.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
	}
	wait(agent.lua_state_agent, 2.0);
	if macros::is_excute(agent) {
		AttackModule::clear(agent.module_accessor, 0, false);
	}
	wait(agent.lua_state_agent, 3.0);
	if macros::is_excute(agent) {
		AttackModule::clear_all(agent.module_accessor);
	}
}

unsafe extern "C" fn palutena_effect_attackhi4(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 2.0);
	if macros::is_excute(agent) {
		macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -0.0, -0.0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	frame(agent.lua_state_agent, 6.0);
	if macros::is_excute(agent) {
		macros::EFFECT_FOLLOW(agent, Hash40::new("palutena_wand_light_smash_hi"), Hash40::new("stick"), 0, 8.65, 0, 0, 0, 0, 0.6, true);
	}
	frame(agent.lua_state_agent, 12.0);
	if macros::is_excute(agent) {
		macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	}
	frame(agent.lua_state_agent, 13.0);
	if macros::is_excute(agent) {
		macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 18, 0, -90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
		macros::LAST_PARTICLE_SET_COLOR(agent, 0.4, 0.6, 1);
		macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), -0.15, 0, 10, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
	}
	frame(agent.lua_state_agent, 18.0);
	if macros::is_excute(agent) {
		macros::AFTER_IMAGE_OFF(agent, 1);
	}
}

unsafe extern "C" fn palutena_effect_attackhi4charge(agent: &mut L2CAgentBase) {
	frame(agent.lua_state_agent, 5.0);
	if macros::is_excute(agent) {
		macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
	}
	wait(agent.lua_state_agent, 5.0);
	macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("haver"), 0, -0.0, 8, 0, 0, 0, 1, 2, 2, 4, 0, 0, 0, true);
}

unsafe extern "C" fn palutena_sound_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_03"));
        macros::PLAY_SE(agent, Hash40::new("vc_palutena_attack07"));
    }
}

pub fn install() {
    Agent::new("palutena")
        .game_acmd("game_attackhi4_moon", palutena_game_attackhi4, Default)
        .effect_acmd("effect_attackhi4_moon", palutena_effect_attackhi4, Default)
        .effect_acmd("effect_attackhi4charge_moon", palutena_effect_attackhi4charge, Default)
        .sound_acmd("sound_attackhi4_moon", palutena_sound_attackhi4, Default)
        .install();
}
