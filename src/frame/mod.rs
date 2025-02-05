use super::*;

unsafe extern "C" fn palutena_frame(fighter: &mut L2CFighterCommon) {
    if is_moon(fighter.module_accessor) {
        let situation_kind = fighter.global_table[0x16].get_i32();
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&situation_kind)
        || [*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING].contains(&status_kind) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
        }
    
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) {
            WorkModule::enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        } else {
            WorkModule::unable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        }
    }
}

pub fn install() {
    Agent::new("palutena")
        .on_line(Main, palutena_frame)
        .install();
}
