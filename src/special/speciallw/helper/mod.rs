use super::*;

pub unsafe extern "C" fn FUN_7100024ec0(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let is_continue_mot = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        if !is_continue_mot {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_loop"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_loop"), -1.0, 1.0, 0.0, false, false);
        }
    } else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let is_continue_mot = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        if !is_continue_mot {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_loop"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        } else {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_lw_loop"), -1.0, 1.0, 0.0);
        }
    }
}

pub unsafe extern "C" fn FUN_71000260f0(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let is_continue_mot = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        if !is_continue_mot {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
        }
    } else {
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let is_continue_mot = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        if !is_continue_mot {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
        }
    }
}

pub unsafe extern "C" fn FUN_71000260f01(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let is_continue_mot = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        if !is_continue_mot {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 8.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
        }
    } else {
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let is_continue_mot = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        if !is_continue_mot {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 8.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
        }
    }
}
