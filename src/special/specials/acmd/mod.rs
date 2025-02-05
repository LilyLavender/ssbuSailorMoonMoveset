use super::*;

unsafe extern "C" fn palutena_tiara_game_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.8, 361, 15, 0, 45, 1.8, 0.0, 0.5, -7.0, Some(0.0), Some(0.5), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 24, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn palutena_tiara_effect_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            macros::EFFECT_FOLLOW(agent, Hash40::new("link_swordbeam"), Hash40::new("top"), 0, 0, 0, 0, 0, -10, 1, true);
        } else {
            macros::EFFECT_FOLLOW(agent, Hash40::new("link_swordbeam"), Hash40::new("top"), 0, 0, 0, 0, 0, 10, 1, true);
        }
    }
}

unsafe extern "C" fn palutena_tiara_sound_fly(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn palutena_tiara_effect_hit(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("link_swordbeam_hit"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}

unsafe extern "C" fn palutena_tiara_sound_hit(agent: &mut L2CAgentBase) {}

unsafe extern "C" fn palutena_tiara_sound_vanish(agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("palutena_tiara")
        .game_acmd("game_fly", palutena_tiara_game_fly, Default)
        .effect_acmd("effect_fly", palutena_tiara_effect_fly, Default)
        .sound_acmd("sound_fly", palutena_tiara_sound_fly, Default)
        .effect_acmd("effect_hit", palutena_tiara_effect_hit, Default)
        .sound_acmd("sound_hit", palutena_tiara_sound_hit, Default)
        .sound_acmd("sound_vanish", palutena_tiara_sound_vanish, Default)
        .install();
}
