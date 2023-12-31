use crate::imports::*;
use crate::samus::vars::*;
use crate::samus::consts::*;
use crate::common::*;

////////rework neutral special
////status
//start
unsafe extern "C" fn neutral_special_start_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn neutral_special_start_status_init(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
unsafe extern "C" fn neutral_special_start_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_s"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_s"), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_s") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_s") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    air_to_ground_transition_status_func(fighter, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(neutral_special_start_main_loop as *const () as _))
}
pub unsafe fn neutral_special_start_main_loop(fighter: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H.into(), true.into());
        return true
    }else if fighter.global_table[0x16].get_i32() != fighter.global_table[0x17].get_i32() {
        air_to_ground_transition_status_func(fighter, true);
    }
    return false
}
unsafe extern "C" fn neutral_special_start_status_exit(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
// unsafe fn neutral_special_start_status_end(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
//aim
unsafe extern "C" fn neutral_special_aim_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn neutral_special_aim_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("special_s"), false, 1.0);
    ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 0.0);
    let frame;
    let weight;
    if fighter.global_table[0xa].get_i32() == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F {
        if VarModule::is_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_MISSILE_MODE) {
            ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 25.0);
            ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 0, Hash40::new("color"), 2.0, 0.0, false, false, 0.0, false, false, false);
            MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR, Hash40::new("visor"), 3.0, 0.0, false, false, 0.0, true, true, false);
        }
        frame = MotionModule::frame(fighter.module_accessor);
        weight = MotionModule::weight(fighter.module_accessor);
    }else {
        VarModule::off_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_MISSILE_MODE);
        frame = 0.0;
        weight = 1.0;
    }
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_h"), frame, 0.0, false, 0.0, false, false);
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_n_f"), frame, 0.0, false, 0.0);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_h"), frame, 0.0, false, 0.0, false, false);
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_air_n_f"), frame, 0.0, false, 0.0);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    MotionModule::set_weight(fighter.module_accessor, weight, false);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);
    VarModule::off_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_IS_CHARGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT);
    if fighter.global_table[0xa].get_i32() != *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F {
        VarModule::off_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_MISSILE_MODE);
        VarModule::set_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_N_ANGLE, 0.0);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(neutral_special_aim_main_loop as *const () as _))
}
pub unsafe fn neutral_special_aim_main_loop(fighter: &mut L2CFighterCommon) -> bool {
    let frame = MotionModule::frame(fighter.module_accessor);
    let weight = MotionModule::weight(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x_no_clamp = ControlModule::get_stick_x_no_clamp(fighter.module_accessor)*lr;
    let stick_y_no_clamp = ControlModule::get_stick_y_no_clamp(fighter.module_accessor);
    //air-ground transition
    if fighter.global_table[0x16].get_i32() != fighter.global_table[0x17].get_i32() {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_h"), frame, 0.0, false, 0.0, false, false);
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_n_f"), frame, 0.0, false, 0.0);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }else {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_h"), frame, 0.0, false, 0.0, false, false);
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_air_n_f"), frame, 0.0, false, 0.0);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        MotionModule::set_weight(fighter.module_accessor, weight, false);
    }
    //calculate control-stick angle
    let mut stick_deg = (stick_y_no_clamp / stick_x_no_clamp).atan().to_degrees(); 
    if stick_deg.is_nan() {
        stick_deg = 0.0;
    }else if stick_x_no_clamp < 0.0 
    || (lr < 0.0 && stick_x_no_clamp == 0.0) {
        stick_deg += 180.0;
    }else {
        if stick_y_no_clamp == 0.0 {
            stick_deg = 0.0;
        }else if stick_y_no_clamp < 0.0 {
            stick_deg += 360.0;
        }
    }
    //decay recoil
    if weight < 1.0 {    
        MotionModule::set_weight(fighter.module_accessor, weight+(1.0/param::SAMUS_FLOAT_SPECIAL_N_RECOIL_FRAME), false);
    }
    //smooth rotation
    let mut frame_new = (stick_deg+frame)/2.0;
    if (stick_deg-frame).abs() >= 180.0 {
        frame_new += 180.0;
        if frame_new >= 360.0 {
            frame_new -= 360.0;
        }
    }
    MotionModule::set_frame(fighter.module_accessor, frame_new, true);
    MotionModule::set_frame_2nd(fighter.module_accessor, frame_new, true);
    //end
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E.into(), true.into());
        return true
    }else //jump-cancel
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
            return true
        }else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
            return true
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT) {
            VarModule::set_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_N_ANGLE, stick_deg);
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F.into(), true.into());
        return true
    }
    return false
}
unsafe extern "C" fn neutral_special_aim_status_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    //charge hold
    if VarModule::is_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_IS_CHARGE) {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT);
        }else {
            if VarModule::is_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_MISSILE_MODE) {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) < param::SAMUS_INT_SPECIAL_N_HOMINGMISSILE_CHARGE_FRAME {
                    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) == param::SAMUS_INT_SPECIAL_N_HOMINGMISSILE_CHARGE_FRAME {
                        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_specialflag_light"), true, true);
                        macros::PLAY_SE(fighter, Hash40::new("se_common_metal_step_s"));
                        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_staff_hold"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true);
                        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_chewing"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
                    }
                }
            }else {
                let charge_frame_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame"));
                let mut charge_frame_curr = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32;
                if charge_frame_curr < charge_frame_max {
                    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);
                    charge_frame_curr += 1.0;
                    ArticleModule::set_float(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, charge_frame_curr / charge_frame_max, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
                    if charge_frame_curr == charge_frame_max {
                        macros::STOP_SE(fighter, Hash40::new("se_samus_special_n01"));
                        macros::PLAY_STATUS(fighter, Hash40::new("se_samus_special_n05"));
                    }
                }
            }
        }
    }else //charge start
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        VarModule::on_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_IS_CHARGE);
        if VarModule::is_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_MISSILE_MODE) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_specialflag_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
        }else if ArticleModule::get_active_num(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT) < 4 {
            ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, -1);
            macros::PLAY_STATUS(fighter, Hash40::new("se_samus_special_n01"));
        }
    }
    //missile mode toggle
    if VarModule::is_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_MISSILE_MODE) {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            VarModule::off_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_MISSILE_MODE);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("special_s"), false, 1.0);
            ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 36.0);
            ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 2.0);
            ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 0, Hash40::new("color"), 0.0, 0.0, false, false, 0.0, false, false, false);
            MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR, false);
            macros::PLAY_SE(fighter, Hash40::new("se_samus_rise"));
            if VarModule::is_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_IS_CHARGE) {
                ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, -1);
                macros::PLAY_STATUS(fighter, Hash40::new("se_samus_special_n01"));
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_chewing"), true, true);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_specialflag_light"), true, true);
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);
            }
        }
    }else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        VarModule::on_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_MISSILE_MODE);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("special_s"), false, 1.0);
        ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 12.0);
        ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 0, Hash40::new("color"), 2.0, 0.0, false, false, 0.0, false, false, false);
        MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR, Hash40::new("visor"), 3.0, 0.0, false, false, 0.0, true, true, false);
        macros::PLAY_SE(fighter, Hash40::new("se_samus_appeal_s01"));
        if VarModule::is_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_IS_CHARGE) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
            macros::STOP_SE(fighter, Hash40::new("se_samus_special_n01"));
            macros::STOP_SE(fighter, Hash40::new("se_samus_special_n05"));
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_specialflag_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.25, true);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);
        }
    }
    0.into()
}
unsafe extern "C" fn neutral_special_aim_status_exit(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
unsafe extern "C" fn neutral_special_aim_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_MATERIAL_MOTION);
        MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR, false);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
    }
    return false.into()
}
//shoot
unsafe extern "C" fn neutral_special_shoot_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn neutral_special_shoot_status_init(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
unsafe extern "C" fn neutral_special_shoot_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let angle = VarModule::get_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_N_ANGLE);
    MotionModule::set_frame(fighter.module_accessor, angle, true);
    MotionModule::set_frame_2nd(fighter.module_accessor, angle, true);
    if VarModule::is_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_MISSILE_MODE) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("special_s"/*"special_s_end"*/), false, 1.0);
        ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 36.0);
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 0.0);
        ArticleModule::add_motion_partial(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, 0, Hash40::new("color"), 2.0, 0.0, false, false, 0.0, false, false, false);
        MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR, Hash40::new("visor"), 2.0, 0.0, false, false, 0.0, true, true, false);
        
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) >= param::SAMUS_INT_SPECIAL_N_HOMINGMISSILE_CHARGE_FRAME {
            VarModule::set_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_MISSILE_COUNT, param::SAMUS_INT_SPECIAL_N_HOMINGMISSILE_MAX);
            VarModule::set_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_FIRE_COUNT, param::SAMUS_INT_SPECIAL_N_FIRE_HOMINGMISSILE_FRAME);
        }else {
            VarModule::set_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_MISSILE_COUNT, 1);
            VarModule::set_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_FIRE_COUNT, param::SAMUS_INT_SPECIAL_N_FIRE_MISSILE_FRAME);
        }
        VarModule::set_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_HOMINGMISSILE_DELAY_COUNT, 0);
    }else { 
        VarModule::set_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_FIRE_COUNT, param::SAMUS_INT_SPECIAL_N_FIRE_CSHOT_FRAME);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beams"), 0, false, 0);
        let max_weight = 0.75; //set between 0.0 and 1.0
        let charge_frame_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("cshot_charge_frame"));
        let charge_frame_curr = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) as f32;
        let charge_ratio = charge_frame_curr /charge_frame_max;
        let weight = (1.0 -charge_ratio) *max_weight;
        MotionModule::set_weight(fighter.module_accessor, weight, false);

        macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_cshot_shot"), Hash40::new("armr"), 7, -1, 0, 0, 90, 15, charge_ratio.clamp(0.1, 0.4), false);
    }
    slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    fighter.sub_shift_status_main(L2CValue::Ptr(neutral_special_shoot_status_loop as *const () as _))
}
pub unsafe fn neutral_special_shoot_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    //air-ground transition
    if fighter.global_table[0x16].get_i32() != fighter.global_table[0x17].get_i32() {
        let weight = MotionModule::weight(fighter.module_accessor);
        let angle = VarModule::get_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_N_ANGLE);
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_h"), angle, 0.0, false, 0.0, false, false);
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_n_f"), angle, 0.0, false, 1.0);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_h"), angle, 0.0, false, 0.0, false, false);
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_air_n_f"), angle, 0.0, false, 1.0);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        MotionModule::set_weight(fighter.module_accessor, weight, false);
    }
    if VarModule::is_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_N_MISSILE_MODE) {
        let missile_count = VarModule::get_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_MISSILE_COUNT);
        let missile_delay = VarModule::get_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_HOMINGMISSILE_DELAY_COUNT);
        if missile_count > 0 {
            if missile_delay <= 0 {
                VarModule::set_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_HOMINGMISSILE_DELAY_COUNT, param::SAMUS_INT_SPECIAL_N_HOMINGMISSILE_DELAY_FRAME);
                VarModule::dec_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_MISSILE_COUNT);

                MotionModule::set_weight(fighter.module_accessor, 0.0, false);
                let angle = VarModule::get_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_N_ANGLE);
                macros::EFFECT(fighter, Hash40::new("samus_missile_shot"), Hash40::new("haver"), 0, 0, 0, -angle, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attacks"), 0, false, 0);

                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT) >= param::SAMUS_INT_SPECIAL_N_HOMINGMISSILE_CHARGE_FRAME {
                    ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_MISSILE, false, -1);
                    ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_MISSILE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
                    macros::PLAY_SE(fighter, Hash40::new("se_samus_special_s01"));
                }else  {
                    ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_SUPERMISSILE, false, -1);
                    ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_SUPERMISSILE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
                    macros::PLAY_SE(fighter, Hash40::new("se_samus_special_s03"));
                    if ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_SUPERMISSILE) == false {
                        VarModule::set_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_FIRE_COUNT, param::SAMUS_INT_SPECIAL_N_FIRE_MISSILE_FRAME /3);
                        MotionModule::set_weight(fighter.module_accessor, 0.5, false);
                    }
                }
            }else {
                VarModule::dec_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_HOMINGMISSILE_DELAY_COUNT);
                let weight = MotionModule::weight(fighter.module_accessor);
                if weight < 1.0 {
                    MotionModule::set_weight(fighter.module_accessor, weight+(1.0/param::SAMUS_FLOAT_SPECIAL_N_RECOIL_FRAME), false);
                }
            }
        }else {
            if VarModule::get_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_FIRE_COUNT) > 0 {
                VarModule::dec_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_FIRE_COUNT);
                let weight = MotionModule::weight(fighter.module_accessor);
                if weight < 1.0 {
                    MotionModule::set_weight(fighter.module_accessor, weight+(1.0/param::SAMUS_FLOAT_SPECIAL_N_RECOIL_FRAME), false);
                }
            }else{
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
                || (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0 
                || (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0
                || (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0
                || (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0
                || (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 
                {
                    fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E.into(), true.into());
                }else {
                    fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H.into(), true.into());
                }
                return true
            }
        }
    }else {
        if VarModule::get_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_FIRE_COUNT) > 0 {
            VarModule::dec_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_N_FIRE_COUNT);
            let weight = MotionModule::weight(fighter.module_accessor);
            if weight < 1.0 {
                MotionModule::set_weight(fighter.module_accessor, weight+(1.0/param::SAMUS_FLOAT_SPECIAL_N_RECOIL_FRAME), false);
            }
        }else{
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H.into(), true.into());
            return true
        }
    }
    return false
}
unsafe extern "C" fn neutral_special_shoot_status_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
unsafe extern "C" fn neutral_special_shoot_status_exit(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
unsafe extern "C" fn neutral_special_shoot_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_MATERIAL_MOTION);
        MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR, false);
    }
    return false.into()
}
//end
unsafe extern "C" fn neutral_special_end_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn neutral_special_end_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_e"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_e"), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_e") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_e") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    air_to_ground_transition_status_func(fighter, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(neutral_special_end_main_loop as *const () as _))
}
pub unsafe fn neutral_special_end_main_loop(fighter: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
        return true
    }else {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return true
            }
        }
        if fighter.global_table[0x16].get_i32() != fighter.global_table[0x17].get_i32() {
            air_to_ground_transition_status_func(fighter, true);
        }
    }
    return false
}
unsafe extern "C" fn neutral_special_end_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_MATERIAL_MOTION);
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_SAMUS_MOTION_PART_SET_KIND_VISOR, false);
    return false.into()
}
////motion
unsafe extern "C" fn neutral_special_all_game(_fighter : &mut L2CAgentBase) {}
unsafe extern "C" fn neutral_special_all_exp(_fighter : &mut L2CAgentBase) {}
unsafe extern "C" fn neutral_special_all_snd(_fighter : &mut L2CAgentBase) {}
unsafe extern "C" fn neutral_special_all_eff(_fighter : &mut L2CAgentBase) {}
//////arm-cannon
////motion
unsafe extern "C" fn gun_special_s_eff(weapon: &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 25.0);
    if is_excute(weapon) {
        MotionModule::set_rate(weapon.module_accessor, 0.0);
    }
}
//////chargeshot
////status
unsafe extern "C" fn cshot_shoot_status_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let original = smashline::original_status(Init, weapon, *WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT);
    let ret = original(weapon);

    let charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);

    let fighter_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let fighter_boma = sv_battle_object::module_accessor(fighter_id);
    let angle = VarModule::get_float(fighter_boma, status::SAMUS_FLOAT_SPECIAL_N_ANGLE);

    let lr = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_SHOOT_LR);
    let min_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_speed"));
    let max_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("max_speed"));

    let speed = ((max_speed-min_speed)*charge)+min_speed;

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        angle.to_radians().cos() * speed * lr,
        angle.to_radians().sin() * speed
    );
    return ret
}
unsafe extern "C" fn cshot_shoot_status_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("samus_cshot_ground"), true, true);
    0.into()
}
unsafe extern "C" fn cshot_shoot_status_exit(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        let touch_pos = &mut Vector2f{x:0.0, y:0.0};
        let touch_rot = &mut Vector2f{x:0.0, y:0.0};
        FighterUtil::get_air_ground_touch_info(weapon.module_accessor, touch_pos, touch_rot);
        let pos_z = GroundModule::get_z(weapon.module_accessor);
        let rot = (-touch_rot.x).atan2(touch_rot.y);
        let charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
        let min_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_scale"));
        let max_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("max_scale"));
        let size = ((max_scale -min_scale) *charge) +min_scale;
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_ground_shockwave"), &Vector3f{x:touch_pos.x, y:touch_pos.y, z:pos_z}, &Vector3f{x:0.0, y:0.0, z:rot}, size, 0, -1, false, 0);
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_hit_magic_s"), &Vector3f{x:touch_pos.x, y:touch_pos.y, z:pos_z}, &Vector3f{x:0.0, y:0.0, z:0.0}, size*0.7, 0, -1, false, 0);
    }
    let original = smashline::original_status(Exit, weapon, *WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT);
    original(weapon)
}
////motion
unsafe extern "C" fn cshot_shoot_game(weapon: &mut L2CAgentBase) {
    if is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 1.0, 361, 0, 0, 0, 1.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        if WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.25 {
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 35.0, 361, 0, 0, 0, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        }else {
            macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 35.0, 40, 50, 0, 45, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        }
        attack!(weapon, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
    }
}
unsafe extern "C" fn cshot_shoot_snd(weapon : &mut L2CAgentBase) {
    if is_excute(weapon) {
        macros::STOP_SE(weapon, Hash40::new("se_samus_special_n01"));
        if WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.25 {
            macros::PLAY_SE_REMAIN(weapon, Hash40::new("se_samus_special_n02"));
        }else if WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.625 {
            macros::PLAY_SE_REMAIN(weapon, Hash40::new("se_samus_special_n03"));
        }else if WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE) <= 0.875 {
            macros::PLAY_SE_REMAIN(weapon, Hash40::new("se_samus_special_n04"));
        }else {
            macros::PLAY_SE_REMAIN(weapon, Hash40::new("se_samus_special_n04"));
        }
    }
}
////homing missile
unsafe extern "C" fn missile_homing_status_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let fighter_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID);
    let fighter_boma = sv_battle_object::module_accessor(fighter_id as u32);
    let mut angle = VarModule::get_float(fighter_boma, status::SAMUS_FLOAT_SPECIAL_N_ANGLE);

    let missile_count = VarModule::get_int(fighter_boma, status::SAMUS_INT_SPECIAL_N_MISSILE_COUNT);
    if missile_count < param::SAMUS_INT_SPECIAL_N_HOMINGMISSILE_MAX-1 {
        if missile_count%2 == 0 {
            angle += param::SAMUS_FLOAT_SPECIAL_N_HOMINGMISSILE_ANGLE_OFFSET;
        }else {
            angle -= param::SAMUS_FLOAT_SPECIAL_N_HOMINGMISSILE_ANGLE_OFFSET;
        }
    }

    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("rot"), &Vector3f{ x: -angle, y: 0.0, z: 0.0 }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});

    if PostureModule::lr(fighter_boma) < 0.0 {
        if angle <= 180.0 {
            angle = 180.0-angle;
        }else {
            angle = 540.0-angle;
        }
        if angle < 90.0 || angle > 270.0 {
            PostureModule::reverse_lr(weapon.module_accessor);
        }
    }else {
        if angle > 90.0 && angle < 270.0 {
            PostureModule::reverse_lr(weapon.module_accessor);
        }
    }
    angle = (PI*2.0)*((360.0-angle)/360.0);
    if angle > PI {
        angle = angle-(PI*2.0);
    }
    WorkModule::set_float(weapon.module_accessor, angle, *WEAPON_SAMUS_MISSILE_INSTANCE_WORK_ID_FLOAT_ROT);

    let original = smashline::original_status(Main, weapon, *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING);
    original(weapon)
}
//////supermissile
////status
//start
unsafe extern "C" fn supermissile_ready_status_main(weapon: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("ready"), 0.0, 1.0, false, 0.0, false, false);

    let fighter_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID);
    let fighter_boma = sv_battle_object::module_accessor(fighter_id as u32);
    let angle = VarModule::get_float(fighter_boma, status::SAMUS_FLOAT_SPECIAL_N_ANGLE);
    VarModule::set_float(weapon.module_accessor, instance::SAMUS_SUPERMISSILE_FLOAT_ANGLE, angle);

    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("rot"), &Vector3f{ x: -angle, y: 0.0, z: 0.0 }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});

    let accel_frame = WorkModule::get_param_int(weapon.module_accessor, hash40("param_supermissile"), hash40("s_acc_f"));
    WorkModule::set_int(weapon.module_accessor, accel_frame, *WEAPON_SAMUS_SUPERMISSILE_STATUS_READY_WORK_ID_INT_FRAME);

    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_start_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_supermissile"), hash40("s_spd_x0"));
    let speed_start_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_supermissile"), hash40("s_spd_y0"));
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_start_x*lr,
        -speed_start_y
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -speed_start_x*lr/accel_frame as f32,
        speed_start_y/accel_frame as f32
    );
    let rot = WorkModule::get_param_float(weapon.module_accessor, hash40("param_supermissile"), hash40("s_rot"));
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL,
        -rot/accel_frame as f32,
        0.0,
        0.0
    );
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL);
    weapon.global_table[0x15].assign(&L2CValue::Ptr(supermissile_ready_status_substatus as *const () as _));
    weapon.fastshift(L2CValue::Ptr(supermissile_ready_status_loop as *const () as _))
}
unsafe extern "C" fn supermissile_ready_status_substatus(weapon: &mut L2CWeaponCommon, param_2: L2CValue) -> L2CValue {
    if param_2.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_SAMUS_SUPERMISSILE_STATUS_READY_WORK_ID_INT_FRAME);
    }
    false.into()
}
pub unsafe fn supermissile_ready_status_loop(weapon: &mut L2CFighterCommon) -> bool {
    let angle = VarModule::get_float(weapon.module_accessor, instance::SAMUS_SUPERMISSILE_FLOAT_ANGLE);
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("rot"), &Vector3f{ x: -angle, y: 0.0, z: 0.0 }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        weapon.change_status(WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_S_BURST.into(), false.into());
    }else if WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_SUPERMISSILE_STATUS_READY_WORK_ID_INT_FRAME) <= 0 {
        weapon.change_status(WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_STRAIGHT.into(), false.into());
    }
    false.into()
}
//fly
unsafe extern "C" fn supermissile_straight_status_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL, // Originally _NONE
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn supermissile_straight_status_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("straight"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::unable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL);
    let angle = VarModule::get_float(weapon.module_accessor, instance::SAMUS_SUPERMISSILE_FLOAT_ANGLE);
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("rot"), &Vector3f{ x: -angle, y: 0.0, z: 0.0 }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_supermissile"), hash40("s_acc_x"));
    let max_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_supermissile"), hash40("s_spd_x_max"));
    let lr = PostureModule::lr(weapon.module_accessor);
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        angle.to_radians().cos() * accel * lr,
        angle.to_radians().sin() * accel
    );
    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        angle.to_radians().cos() * max_speed,
        angle.to_radians().sin() * max_speed
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        angle.to_radians().cos() * max_speed,
        angle.to_radians().sin() * max_speed
    );

    weapon.global_table[0x15].assign(&L2CValue::Ptr(supermissile_straight_status_substatus as *const () as _));
    weapon.fastshift(L2CValue::Ptr(supermissile_straight_status_loop as *const () as _))
}
unsafe extern "C" fn supermissile_straight_status_substatus(weapon: &mut L2CWeaponCommon, param_2: L2CValue) -> L2CValue {
    if param_2.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_SAMUS_SUPERMISSILE_STATUS_STRAIGHT_WORK_ID_INT_FRAME)
    }
    0.into()
}
unsafe fn supermissile_straight_status_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let angle = VarModule::get_float(weapon.module_accessor, instance::SAMUS_SUPERMISSILE_FLOAT_ANGLE);
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("rot"), &Vector3f{ x: -angle, y: 0.0, z: 0.0 }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});

    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32)
    || WorkModule::get_int(weapon.module_accessor, *WEAPON_SAMUS_SUPERMISSILE_STATUS_STRAIGHT_WORK_ID_INT_FRAME) <= 0 {
        weapon.change_status(WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_S_BURST.into(), false.into());
    }
    0.into()
}
////motion
unsafe extern "C" fn supermissile_ready_game(weapon : &mut L2CAgentBase) {
    if is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 7.0, 65, 0, 100, 60, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
    }
}
unsafe extern "C" fn supermissile_straight_game(weapon : &mut L2CAgentBase) {
    if is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 7.0, 65, 0, 100, 60, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
    }
}
unsafe extern "C" fn supermissile_straight_eff(weapon : &mut L2CAgentBase) {
    if is_excute(weapon) {
        macros::EFFECT_FOLLOW(weapon, Hash40::new("samus_missile_straight"), Hash40::new("rot"), 0, 0, -1, 0, 0, 0, 1, true);
    }
}
unsafe extern "C" fn supermissile_sburst_game(weapon : &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 1.0);
    if is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, 361, 0, 100, 30, 18.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_OBJECT);
        ControlModule::set_rumble(weapon.module_accessor, Hash40::new("rbkind_erase"), 0, false, 0);
    }
    wait(weapon.lua_state_agent, 1.0);
    if is_excute(weapon) {
        ControlModule::set_rumble(weapon.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
        AttackModule::clear_all(weapon.module_accessor);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
}
unsafe extern "C" fn supermissile_sburst_snd(weapon : &mut L2CAgentBase) {
    if is_excute(weapon) {
        macros::PLAY_SE_REMAIN(weapon, Hash40::new("se_common_frieze_ll"));
        macros::PLAY_SE_REMAIN(weapon, Hash40::new("se_common_bomb_l"));
    }
}
unsafe extern "C" fn supermissile_sburst_eff(weapon : &mut L2CAgentBase) {
    if is_excute(weapon) {
        // EFFECT_BRANCH_SITUATION(fighter, Hash40::new("sys_hit_ice_shock"), Hash40::new("sys_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
        weapon.clear_lua_stack();
        lua_args!(weapon, Hash40::new("sys_freezer"), Hash40::new("sys_hit_ice_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        sv_animcmd::EFFECT_BRANCH_SITUATION(weapon.lua_state_agent);
        weapon.clear_lua_stack();
    }
}

pub fn install(agent: &mut smashline::Agent) {
    ////status
    //start
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, neutral_special_start_status_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, neutral_special_start_status_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, neutral_special_start_status_main);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, neutral_special_start_status_exit);
    // agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, neutral_special_start_status_end);
    //aim
    agent.status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, neutral_special_aim_status_pre);
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, neutral_special_aim_status_main);
    agent.status(Exec, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, neutral_special_aim_status_exec);
    agent.status(Exit, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, neutral_special_aim_status_exit);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, neutral_special_aim_status_end);
    //shoot
    agent.status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, neutral_special_shoot_status_pre);
    agent.status(Init, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, neutral_special_shoot_status_init);
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, neutral_special_shoot_status_main);
    agent.status(Exec, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, neutral_special_shoot_status_exec);
    agent.status(Exit, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, neutral_special_shoot_status_exit);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, neutral_special_shoot_status_end);
    //end
    agent.status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E, neutral_special_end_status_pre);
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E, neutral_special_end_status_main);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E, neutral_special_end_status_end);
    ////motion
    //start-ground
    agent.game_acmd("game_specialnstart", neutral_special_all_game);
    agent.expression_acmd("expression_specialnstart", neutral_special_all_exp);
    agent.sound_acmd("sound_specialnstart", neutral_special_all_snd);
    agent.effect_acmd("effect_specialnstart", neutral_special_all_eff);
    //start-air
    agent.game_acmd("game_specialairnstart", neutral_special_all_game);
    agent.expression_acmd("expression_specialairnstart", neutral_special_all_exp);
    agent.sound_acmd("sound_specialairnstart", neutral_special_all_snd);
    agent.effect_acmd("effect_specialairnstart", neutral_special_all_eff);
    //aim-ground
    agent.game_acmd("game_specialnhold", neutral_special_all_game);
    agent.expression_acmd("expression_specialnhold", neutral_special_all_exp);
    agent.sound_acmd("sound_specialnhold", neutral_special_all_snd);
    agent.effect_acmd("effect_specialnhold", neutral_special_all_eff);
    //aim-air
    agent.game_acmd("game_specialairnhold", neutral_special_all_game);
    agent.expression_acmd("expression_specialairnhold", neutral_special_all_exp);
    agent.sound_acmd("sound_specialairnhold", neutral_special_all_snd);
    agent.effect_acmd("effect_specialairnhold", neutral_special_all_eff);
    //shoot-ground
    agent.game_acmd("game_specialnfire", neutral_special_all_game);
    agent.expression_acmd("expression_specialnfire", neutral_special_all_exp);
    agent.sound_acmd("sound_specialnfire", neutral_special_all_snd);
    agent.effect_acmd("effect_specialnfire", neutral_special_all_eff);
    //shoot-air
    agent.game_acmd("game_specialairnfire", neutral_special_all_game);
    agent.expression_acmd("expression_specialairnfire", neutral_special_all_exp);
    agent.sound_acmd("sound_specialairnfire", neutral_special_all_snd);
    agent.effect_acmd("effect_specialairnfire", neutral_special_all_eff);
    //end-ground
    agent.game_acmd("game_specialnend", neutral_special_all_game);
    agent.expression_acmd("expression_specialnend", neutral_special_all_exp);
    agent.sound_acmd("sound_specialnend", neutral_special_all_snd);
    agent.effect_acmd("effect_specialnend", neutral_special_all_eff);
    //end-air
    agent.game_acmd("game_specialairnend", neutral_special_all_game);
    agent.expression_acmd("expression_specialairnend", neutral_special_all_exp);
    agent.sound_acmd("sound_specialairnend", neutral_special_all_snd);
    agent.effect_acmd("effect_specialairnend", neutral_special_all_eff);
    ////gun
    Agent::new("samus_gun")
    .effect_acmd("effect_specials", gun_special_s_eff)
    .install();
    ////charge-beam
    Agent::new("samus_cshot")
    .status(Init, *WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, cshot_shoot_status_init)
    .status(Exec, *WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, cshot_shoot_status_exec)
    .status(Exit, *WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, cshot_shoot_status_exit)
    .game_acmd("game_shoot", cshot_shoot_game)
    .sound_acmd("sound_shoot", cshot_shoot_snd)
    .install();
    ////homing-missle
    Agent::new("samus_missile")
    .status(Main, *WEAPON_SAMUS_MISSILE_STATUS_KIND_HOMING, missile_homing_status_main)
    .install();
    ////ice-missle
    Agent::new("samus_supermissile")
    //status
    .status(Main, *WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_READY, supermissile_ready_status_main)
    .status(Pre, *WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_STRAIGHT, supermissile_straight_status_pre)
    .status(Main, *WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_STRAIGHT, supermissile_straight_status_main)
    //start
    .game_acmd("game_ready", supermissile_ready_game)
    //fly
    .game_acmd("game_straight", supermissile_straight_game)
    .effect_acmd("effect_straight", supermissile_straight_eff)
    //explode
    .game_acmd("game_sburst", supermissile_sburst_game)
    .sound_acmd("sound_sburst", supermissile_sburst_snd)
    .effect_acmd("effect_sburst", supermissile_sburst_eff)
    .install();
}