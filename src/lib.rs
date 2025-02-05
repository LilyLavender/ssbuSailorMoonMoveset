#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const
)]

use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

pub static mut FIGHTER_PALUTENA_GENERATE_ARTICLE_TIARA : i32 = 0x9;
pub const FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI : i32 = 0x200000EB;
pub const FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_LOOP : i32 = 0x1E4;
pub const FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_LW_END : i32 = 0x1E3;
pub const FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_CHARGE_FRAME_SPECIAL_LW : i32 = 0x100000C2;
pub const FIGHTER_PALUTENA_STATUS_SPECIAL_LW_FLAG_TRANS_LOOP : i32 = 0x21000014;

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

pub fn check_deps() -> bool {
    let mut passed = true;

    for dep in [
        "rom:/skyline/plugins/libparam_config.nro",
        "rom:/skyline/plugins/libthe_csk_collection.nro",
        "rom:/skyline/plugins/libarcropolis.nro",
        "rom:/skyline/plugins/libnro_hook.nro",
        "rom:/skyline/plugins/libsmashline_plugin.nro",
    ] {
        if !std::path::Path::new(dep).is_file() {
            println!("{} not found! This installation is incomplete. Please download all dependencies listed in the README file.", dep);
            passed = false;
        }
    }

    passed
}

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    const FIGHTER_NAME: &str = "palutena";
    const MARKER_FILE: &str = "moon.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/model/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _ ;
                }
            }
        }
    }

	param_config::disable_kirby_copy(*FIGHTER_KIND_PALUTENA, marked_slots.clone());
	param_config::disable_villager_pocket(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), *WEAPON_KIND_LINK_SWORD_BEAM);
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("walk_accel_max"), 0, 1.138));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("ground_brake"), 0, 0.1));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("run_speed_max"), 0, 1.75));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("run_accel_mul"), 0, 0.10978));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("dash_speed"), 0, 1.865));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("air_accel_y"), 0, 0.1));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("air_accel_y_stable"), 0, 1.2));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("air_accel_x_stable"), 0, 1.1));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("damage_fly_top_air_accel_y"), 0, 0.069552));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("dive_speed_y"), 0, 1.78));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("landing_attack_air_frame_n"), 0, 14.0));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("landing_attack_air_frame_b"), 0, 12.0));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("landing_attack_air_frame_hi"), 0, 8.0));
	param_config::update_float_2(*FIGHTER_KIND_PALUTENA, marked_slots.clone(), (hash40("weight"), 0, 89.0));

    if lowest_color == -1 {
        return;
    }

    let color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            index - lowest_color
        }
    };

    the_csk_collection_api::add_chara_db_entry_info(the_csk_collection_api::CharacterDatabaseEntry {
		ui_chara_id: smash::hash40("ui_chara_moon"), 
		fighter_kind: the_csk_collection_api::Hash40Type::Overwrite(0x15D6FA9492 /* fighter_kind_palutena */), 
		fighter_kind_corps: the_csk_collection_api::Hash40Type::Overwrite(0x15D6FA9492 /* fighter_kind_palutena */), 
		ui_series_id: the_csk_collection_api::Hash40Type::Overwrite(0x125B6C5D56 /* ui_series_palutena */), 
		fighter_type: the_csk_collection_api::Hash40Type::Overwrite(0x1353795179 /* fighter_type_normal */), 
		alt_chara_id: the_csk_collection_api::Hash40Type::Overwrite(0x0), 
		shop_item_tag: the_csk_collection_api::Hash40Type::Overwrite(0x0), 
		name_id: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("moon")), 
		exhibit_year: the_csk_collection_api::ShortType::Overwrite(1991), 
		exhibit_day_order: the_csk_collection_api::IntType::Overwrite(121902), 
		extra_flags: the_csk_collection_api::IntType::Overwrite(0), 
		ext_skill_page_num: the_csk_collection_api::SignedByteType::Overwrite(0), 
		skill_list_order: the_csk_collection_api::SignedByteType::Optional(Some(83)), 
		disp_order: the_csk_collection_api::SignedByteType::Optional(Some(83)), 
		save_no: the_csk_collection_api::SignedByteType::Overwrite(53), 
		chara_count: the_csk_collection_api::SignedByteType::Overwrite(1), 
		is_img_ext_skill_page0: the_csk_collection_api::BoolType::Overwrite(false), 
		is_img_ext_skill_page1: the_csk_collection_api::BoolType::Overwrite(false), 
		is_img_ext_skill_page2: the_csk_collection_api::BoolType::Overwrite(false), 
		can_select: the_csk_collection_api::BoolType::Overwrite(true), 
		is_usable_soundtest: the_csk_collection_api::BoolType::Overwrite(true), 
		is_called_pokemon: the_csk_collection_api::BoolType::Overwrite(false), 
		is_mii: the_csk_collection_api::BoolType::Overwrite(false), 
		is_boss: the_csk_collection_api::BoolType::Overwrite(false), 
		is_hidden_boss: the_csk_collection_api::BoolType::Overwrite(false), 
		is_dlc: the_csk_collection_api::BoolType::Overwrite(false), 
		is_patch: the_csk_collection_api::BoolType::Overwrite(false), 
		is_plural_message: the_csk_collection_api::BoolType::Overwrite(false), 
		is_plural_narration: the_csk_collection_api::BoolType::Overwrite(false), 
		is_article: the_csk_collection_api::BoolType::Overwrite(false), 
		unk_0x112b7bb52a: the_csk_collection_api::BoolType::Overwrite(false), 
		result_pf0: the_csk_collection_api::BoolType::Overwrite(true), 
		result_pf1: the_csk_collection_api::BoolType::Overwrite(true), 
		result_pf2: the_csk_collection_api::BoolType::Overwrite(true), 
		color_num: the_csk_collection_api::UnsignedByteType::Overwrite(color_num as u8), 
		extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(std::collections::HashMap::from([
				(0x1337FC912E /* characall_label_c00 */, the_csk_collection_api::Hash40Type::Overwrite(0x1F338F5AA3 /* vc_narration_characall_palutena */)), 
				(0x1340FBA1B8 /* characall_label_c01 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13D9F2F002 /* characall_label_c02 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13AEF5C094 /* characall_label_c03 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1330915537 /* characall_label_c04 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13479665A1 /* characall_label_c05 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13DE9F341B /* characall_label_c06 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13A998048D /* characall_label_c07 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B8B13E500 /* characall_label_article_c00 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1BFC14D596 /* characall_label_article_c01 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B651D842C /* characall_label_article_c02 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B121AB4BA /* characall_label_article_c03 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B8C7E2119 /* characall_label_article_c04 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1BFB79118F /* characall_label_article_c05 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B62704035 /* characall_label_article_c06 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B157770A3 /* characall_label_article_c07 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x160ab9eb98, the_csk_collection_api::Hash40Type::Overwrite(0x11E1FAA171 /* ui_chara_palutena */)),
		])), 
		extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(std::collections::HashMap::from([
				(0x915C075DE /* c00_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9B3B77E6A /* c01_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9825F64F7 /* c02_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x924286F43 /* c03_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9E18F51CD /* c04_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x947F85A79 /* c05_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9761040E4 /* c06_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9D0674B50 /* c07_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9E48F9289 /* n00_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x942F8993D /* n01_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9731083A0 /* n02_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9D5678814 /* n03_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x910C0B69A /* n04_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9B6B7BD2E /* n05_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9875FA7B3 /* n06_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x92128AC07 /* n07_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9F873561A /* c00_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x95E045DAE /* c01_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x96FEC4733 /* c02_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9C99B4C87 /* c03_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x90C3C7209 /* c04_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9AA4B79BD /* c05_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x99BA36320 /* c06_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x93DD46894 /* c07_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x11895f00fc, the_csk_collection_api::UnsignedByteType::Overwrite(lowest_color as _)),
		])), 
		..std::default::Default::default()
	});

	the_csk_collection_api::add_chara_layout_db_entry_info(the_csk_collection_api::CharacterLayoutDatabaseEntry {
		ui_layout_id: smash::hash40("ui_chara_moon_00"), 
		ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_chara_moon")),
		chara_color: the_csk_collection_api::UnsignedByteType::Optional(Some(0)), 
		eye_0_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(2)), 
		eye_1_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(2)), 
		eye_2_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(2)), 
		eye_0_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-45.0)), 
		eye_0_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(219.0)), 
		eye_0_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(80.0)), 
		eye_0_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(210.0)), 
		eye_0_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-36.0)), 
		eye_1_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(226.0)), 
		eye_1_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(92.0)), 
		eye_1_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(216.0)), 
		eye_1_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-13.0)), 
		eye_2_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(99.0)), 
		eye_2_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(77.0)), 
		eye_2_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(94.0)), 
		eye_2_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_flash_info_pos_x: the_csk_collection_api::FloatType::Optional(Some(19.0)), 
		eye_flash_info_pos_y: the_csk_collection_api::FloatType::Optional(Some(29.0)), 
		chara_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(-18.0)), 
		chara_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-65.0)), 
		chara_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.11)), 
		chara_1_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(-14.0)), 
		chara_1_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-68.0)), 
		chara_1_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.43)), 
		chara_1_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_2_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_1_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(-8.0)), 
		chara_1_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(-42.0)), 
		chara_1_3_scale: the_csk_collection_api::FloatType::Optional(Some(1.43)), 
		chara_1_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(-13.0)), 
		chara_1_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(-42.0)), 
		chara_1_4_scale: the_csk_collection_api::FloatType::Optional(Some(1.43)), 
		chara_1_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_3_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(12.0)), 
		chara_3_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(-2.0)), 
		chara_3_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_3_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(20.0)), 
		chara_3_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(11.0)), 
		chara_3_1_scale: the_csk_collection_api::FloatType::Optional(Some(0.97)), 
		chara_3_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(80.0)), 
		chara_3_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(-20.0)), 
		chara_3_2_scale: the_csk_collection_api::FloatType::Optional(Some(0.7)), 
		chara_3_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(12.0)), 
		chara_3_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(-2.0)), 
		chara_3_3_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_3_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(12.0)), 
		chara_3_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(-2.0)), 
		chara_3_4_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_3_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(10.0)), 
		chara_3_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(-43.0)), 
		chara_3_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.03)), 
		chara_3_6_offset_x: the_csk_collection_api::FloatType::Optional(Some(84.0)), 
		chara_3_6_offset_y: the_csk_collection_api::FloatType::Optional(Some(20.0)), 
		chara_3_6_scale: the_csk_collection_api::FloatType::Optional(Some(0.95)), 
		chara_3_7_offset_x: the_csk_collection_api::FloatType::Optional(Some(12.0)), 
		chara_3_7_offset_y: the_csk_collection_api::FloatType::Optional(Some(-2.0)), 
		chara_3_7_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_select_icon_list_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_select_icon_list_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_select_icon_list_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_7_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_7_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_7_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_7_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_7_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_7_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		spirits_eye_visible: the_csk_collection_api::BoolType::Optional(Some(true)), 
		..std::default::Default::default()
	});
}

pub unsafe fn is_moon(boma: *mut BattleObjectModuleAccessor) -> bool {
	let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
	crate::MARKED_COLORS[color as usize]
}

mod aerials;
mod grounded;
mod smashes;
mod special;
mod throws;
mod frame;

#[skyline::main(name = "sailor_moon_moveset")]
pub fn main() {
	if !check_deps() {
        return;
    }

	unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }

	unsafe {
		FIGHTER_PALUTENA_GENERATE_ARTICLE_TIARA += smashline::clone_weapon("link", *WEAPON_KIND_LINK_SWORD_BEAM, "palutena", "tiara", false);
	}
	
	aerials::install();
	grounded::install();
	smashes::install();
	special::install();
	throws::install();
	frame::install();
}