use crate::imports::*;
use crate::samus::vars::*;
use crate::samus::consts::*;
use crate::samus::other::speedbooster::*;

////side-special is now Flash-Shift
/// note: statuses _SPECIAL_S1G and _SPECIAL_S2G are unused along with motions "special_s", "special_air_s", "special", & "special_air"

//determines whether side-special can be used
pub unsafe extern "C" fn special_s_enabled_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_USED)
    && (VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_CHAIN_TIMER) <= 0
    || VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_CHAIN_COUNT) >= param::SAMUS_INT_SPECIAL_S_CHAIN_MAX) {
        return false.into()
    }
    return true.into()
}
//handels enabling side-special and recharge effects
pub unsafe fn special_s_recharge_check(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_USED) {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if status_kind == *FIGHTER_STATUS_KIND_DEAD {
            VarModule::off_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_USED);
        }else if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_S
        && status_kind != *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A {
        // && status_kind != *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A {
            if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_RECHARGE_TIMER) > 0 {
                VarModule::dec_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_RECHARGE_TIMER);
                if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_RECHARGE_TIMER) == 7 {//<-- adjust flash/glow length
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_staff_hold"), Hash40::new("waist"), 0, 1, 0, 0, 0, 0, 1.2, true, 0.9);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_item_get"), Hash40::new("waist"), 0, 1, 0, 0, 0, 0, 1.2, true, 0.9);
                    macros::FLASH(fighter, 0.2, 0.4, 10.0, 0.5);
                    macros::BURN_COLOR(fighter, 1, 1, 1, 0.9);
                    macros::PLAY_SE(fighter, Hash40::new("se_samus_attackair_h01"));
                }
            }else {
                macros::COL_NORMAL(fighter);
                macros::BURN_COLOR_NORMAL(fighter);
                VarModule::off_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_USED);
            }
            if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_CHAIN_TIMER) > 0 {
                VarModule::dec_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_CHAIN_TIMER);
            }
        }
    }
}
//effect removal to fix AFTER_IMAGE() bugging out for a single frame when persisting through a status
pub unsafe fn special_s_effect_trail_check(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF) {
        if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_EFFECT_TRAIL_TIMER) <= 1 {
            VarModule::off_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF);
            VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_EFFECT_TRAIL_TIMER, param::SAMUS_FLOAT_SPECIAL_S_EFFECT_TRAIL_FRAME as i32);
            EffectModule::clear_all_after_image(fighter.module_accessor, 0);
        }else { 
            if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_EFFECT_TRAIL_TIMER) == param::SAMUS_FLOAT_SPECIAL_S_EFFECT_TRAIL_FRAME as i32 {
                macros::AFTER_IMAGE_OFF(fighter, param::SAMUS_FLOAT_SPECIAL_S_EFFECT_TRAIL_FRAME);
            }
            VarModule::dec_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_EFFECT_TRAIL_TIMER);
        }
    }
}

////status
//start
unsafe extern "C" fn special_s_start_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    //easier input for down-special while running
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON)
    && ControlModule::get_stick_y(fighter.module_accessor) < -0.4 {
        VarModule::set_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_S_LR, PostureModule::lr(fighter.module_accessor));
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
        0.into()
    }else {
        if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
            speedbooster_off(fighter);
        }
        //save direction of prev status
        let lr = PostureModule::lr(fighter.module_accessor);
        //invert for turning statuses
        if fighter.global_table[0xa].get_i32() == *FIGHTER_STATUS_KIND_TURN
        || fighter.global_table[0xa].get_i32() == *FIGHTER_STATUS_KIND_TURN_RUN
        || fighter.global_table[0xa].get_i32() == *FIGHTER_STATUS_KIND_TURN_DASH {
            VarModule::set_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_S_LR, lr*-1.0);
        }else {
            VarModule::set_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_S_LR, lr);
        }
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
            (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NO_ATTACK_SPECIAL | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
            0,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
            0
        );
        0.into()
    }
}
unsafe extern "C" fn special_s_start_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //sets lr to prevent side-special turn around
    PostureModule::set_lr(fighter.module_accessor, VarModule::get_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_S_LR));
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    //if facing opposite of stick dir use reverse motions
    if (PostureModule::lr(fighter.module_accessor) == 1.0 && ControlModule::get_stick_x(fighter.module_accessor) > 0.0)
    || (PostureModule::lr(fighter.module_accessor) == -1.0 && ControlModule::get_stick_x(fighter.module_accessor) < 0.0) {
        VarModule::off_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_REVERSE);
    }else {
        VarModule::on_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_REVERSE);
    }
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_b_start"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_f_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }else {
        if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_b_start"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_f_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_USED) {
        VarModule::inc_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_CHAIN_COUNT);
    }else {
        VarModule::on_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_USED);
        // VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_CHAIN_TIMER, param::SAMUS_INT_SPECIAL_S_CHAIN_FRAME);
        VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_RECHARGE_TIMER, param::SAMUS_INT_SPECIAL_S_RECHARGE_FRAME);
        VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_CHAIN_COUNT, 0);
        VarModule::off_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF);
        VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_EFFECT_TRAIL_TIMER, param::SAMUS_FLOAT_SPECIAL_S_EFFECT_TRAIL_FRAME as i32);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_start_status_loop as *const () as _))
}
pub unsafe fn special_s_start_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A.into(), true.into());
        return true.into()
    }else {
        //ground/air transition
        if fighter.global_table[0x16].get_i32() != fighter.global_table[0x17].get_i32() {
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_s_b_start"), -1.0, 1.0, 0.0);
                }else {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_s_f_start"), -1.0, 1.0, 0.0);
                }
            }else {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_s_b_start"), -1.0, 1.0, 0.0);
                }else {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_s_f_start"), -1.0, 1.0, 0.0);
                }
            }
        }
        return false.into()
    }
}
unsafe extern "C" fn special_s_start_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A {
        //added effect removal to fix AFTER_IMAGE(...) bugging out for a single frame when persisting through a status
        VarModule::on_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF);
    }
    0.into()
}
//loop
unsafe extern "C" fn special_s_loop_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NO_ATTACK_SPECIAL) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn special_s_loop_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //sets lr to prevent side-special turn around
    let lr = VarModule::get_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_S_LR);
    PostureModule::set_lr(fighter.module_accessor, lr);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        param::SAMUS_FLOAT_SPECIAL_S_SPEED,
        0.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        param::SAMUS_FLOAT_SPECIAL_S_SPEED,
        0.0
    );
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_b_loop"), 0.0, 1.0, false, 0.0, false, false);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            param::SAMUS_FLOAT_SPECIAL_S_SPEED*lr*-1.0,
            0.0
        );
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_f_loop"), 0.0, 1.0, false, 0.0, false, false);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            param::SAMUS_FLOAT_SPECIAL_S_SPEED*lr,
            0.0
        );
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    JostleModule::set_status(fighter.module_accessor, false);
    VarModule::set_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_S_LOOP_FRAME_TIMER, param::SAMUS_INT_SPECIAL_S_LOOP_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_loop_status_loop as *const () as _))
}
pub unsafe fn special_s_loop_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    if VarModule::get_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_S_LOOP_FRAME_TIMER) <= 0 {
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A.into(), true.into());
        return true.into()
    }else {
        VarModule::dec_int(fighter.module_accessor, status::SAMUS_INT_SPECIAL_S_LOOP_FRAME_TIMER);
        //ground/air transition
        if fighter.global_table[0x16].get_i32() != fighter.global_table[0x17].get_i32() {
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }else {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }else if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
            // corrects for awkward ecb to allow side-special to become grounded again
            let pos =  PostureModule::pos(fighter.module_accessor);
            let ground_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, pos, 90.0, true);
            if ground_dist < 1.0
            && ground_dist > 0.0 {
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
        return false.into()
    }
}
unsafe extern "C" fn special_s_loop_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    //added effect removal to fix AFTER_IMAGE bugging out for a single frame when persisting through a status
    if fighter.global_table[0xb].get_i32() != *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A {
        VarModule::on_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF);
    }
    0.into()
}
//end
unsafe extern "C" fn special_s_end_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NO_ATTACK_SPECIAL) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn special_s_end_status_exit(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
unsafe extern "C" fn special_s_end_status_init(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
unsafe extern "C" fn special_s_end_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {

    //sets lr to prevent side-special turn around
    let lr = VarModule::get_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_S_LR);
    PostureModule::set_lr(fighter.module_accessor, lr);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_b_end"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_f_end"), 0.0, 1.0, false, 0.0, false, false);
        }
    }else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_b_end"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_f_end"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        param::SAMUS_FLOAT_SPECIAL_S_SPEED,
        0.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        param::SAMUS_FLOAT_SPECIAL_S_SPEED,
        0.0
    );
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            param::SAMUS_FLOAT_SPECIAL_S_SPEED*lr*-1.0,
            0.0
        );
    }else {
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            param::SAMUS_FLOAT_SPECIAL_S_SPEED*lr,
            0.0
        );
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    JostleModule::set_status(fighter.module_accessor, false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
    VarModule::off_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_S_CHAIN_CANCEL_ENABLE);
    VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_CHAIN_TIMER, param::SAMUS_INT_SPECIAL_S_CHAIN_FRAME);
    // VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_RECHARGE_TIMER, param::SAMUS_INT_SPECIAL_S_RECHARGE_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_end_status_loop as *const () as _))
}
pub unsafe fn special_s_end_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    //air/ground transition
    if fighter.global_table[0x16].get_i32() != fighter.global_table[0x17].get_i32() {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_s_b_end"), -1.0, 1.0, 0.0);
            }else {
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_s_f_end"), -1.0, 1.0, 0.0);
            }
        }else {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_s_b_end"), -1.0, 1.0, 0.0);
            }else {
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_s_f_end"), -1.0, 1.0, 0.0);
            }
        }
    }
    //enable gravity
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
    //end
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
        return true.into()
    //checks to cancel side-special into itself
    }else if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPECIAL_S_CHAIN_COUNT) < param::SAMUS_INT_SPECIAL_S_CHAIN_MAX
    && VarModule::is_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_S_CHAIN_CANCEL_ENABLE)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && (ControlModule::get_stick_x(fighter.module_accessor) > 0.6 || ControlModule::get_stick_x(fighter.module_accessor) < -0.6) {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
        return true.into();
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    return false.into()
}
unsafe extern "C" fn special_s_end_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    //added effect removal to fix AFTER_IMAGE bugging out for a single frame when persisting through a status
    if fighter.global_table[0xb].get_i32() != *FIGHTER_STATUS_KIND_SPECIAL_S {
        VarModule::on_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF);
    }
    0.into()
}
////motion
//start
unsafe extern "C" fn special_s_start_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
      JostleModule::set_status(fighter.module_accessor, false);
    }
}
unsafe extern "C" fn special_s_start_snd(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_smash_s01"));
    }
}
unsafe extern "C" fn special_s_start_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        //head
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_samus_trail1"), Hash40::new("tex_samus_trail2"), 60, Hash40::new("head"), 1.0, 0.0, 0.0, Hash40::new("head"), -1.0, 0.0, 0.0, false, Hash40::new("null"), Hash40::new("head"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        //arm-canon
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_samus_trail1"), Hash40::new("tex_samus_trail2"), 60, Hash40::new("haver"), 0.0, 1.0, 0.0, Hash40::new("haver"), 0.0, -1.0, 0.0, false, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        //right foot
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_samus_trail1"), Hash40::new("tex_samus_trail2"), 60, Hash40::new("footr"), 1.0, 0.0, 0.0, Hash40::new("footr"), -1.0, 0.0, 0.0, false, Hash40::new("null"), Hash40::new("footr"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);

        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_genesis_end"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, 0.01);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.6);
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_deku_flash"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true, 0.01);
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_special_defense_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true, 0.001);
        macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.4, 10.0);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.2);
        macros::FLASH(fighter, 0.2, 0.4, 10.0, 0.5);
        macros::BURN_COLOR(fighter, 1, 1, 1, 0.9);
    }
}
//loop
unsafe extern "C" fn special_s_loop_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_special_defense_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true, 0.001);
        macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.4, 1.0);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.2);
        macros::FLASH(fighter, 0.2, 0.4, 1.0, 0.5);
        macros::BURN_COLOR(fighter, 1, 1, 1, 0.9);
    }
}
//end
unsafe extern "C" fn special_s_end_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_S_CHAIN_CANCEL_ENABLE);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
        JostleModule::set_status(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
unsafe extern "C" fn special_s_end_snd(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_smash_s02"));
    }
}
unsafe extern "C" fn special_s_end_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_special_defense_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true, 0.001);
        macros::LAST_EFFECT_SET_RATE(fighter, 2.0);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.4, 10.0);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.2);
        macros::FLASH(fighter, 0.2, 0.4, 10.0, 0.5);
        macros::BURN_COLOR(fighter, 1, 1, 1, 0.9);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        //added effect removal to fix AFTER_IMAGE bugging out for a single frame when persisting through a status
        VarModule::on_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_special_defense_up"), false, false);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        macros::COL_NORMAL(fighter);
        macros::BURN_COLOR_NORMAL(fighter);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    ////status
    //start
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_start_status_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_start_status_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_start_status_end);
    //loop
    agent.status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, special_s_loop_status_pre);
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, special_s_loop_status_main);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, special_s_loop_status_end);
    //end
    agent.status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, special_s_end_status_pre);
    agent.status(Init, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, special_s_end_status_init);
    agent.status(Main, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, special_s_end_status_main);
    agent.status(Exit, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, special_s_end_status_exit);
    agent.status(End, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, special_s_end_status_end);
    ////motion
    //start forward ground
    agent.game_acmd("game_specialsfstart", special_s_start_game);
    agent.sound_acmd("sound_specialsfstart", special_s_start_snd);
    agent.effect_acmd("effect_specialsfstart", special_s_start_eff);
    //start forward air
    agent.game_acmd("game_specialairsfstart", special_s_start_game);
    agent.sound_acmd("sound_specialairsfstart", special_s_start_snd);
    agent.effect_acmd("effect_specialairsfstart", special_s_start_eff);
    //start backward ground
    agent.game_acmd("game_specialsbstart", special_s_start_game);
    agent.sound_acmd("sound_specialsbstart", special_s_start_snd);
    agent.effect_acmd("effect_specialsbstart", special_s_start_eff);
    //start backward air
    agent.game_acmd("game_specialairsbstart", special_s_start_game);
    agent.sound_acmd("sound_specialairsbstart", special_s_start_snd);
    agent.effect_acmd("effect_specialairsbstart", special_s_start_eff);
    //loop
    agent.effect_acmd("effect_specialsfloop", special_s_loop_eff);
    agent.effect_acmd("effect_specialsbloop", special_s_loop_eff);
    //end forward ground
    agent.game_acmd("game_specialsfend", special_s_end_game);
    agent.sound_acmd("sound_specialsfend", special_s_end_snd);
    agent.effect_acmd("effect_specialsfend", special_s_end_eff);
    //end forward air
    agent.game_acmd("game_specialairsfend", special_s_end_game);
    agent.sound_acmd("sound_specialairsfend", special_s_end_snd);
    agent.effect_acmd("effect_specialairsfend", special_s_end_eff);
    //end backward ground
    agent.game_acmd("game_specialsbend", special_s_end_game);
    agent.sound_acmd("sound_specialsbend", special_s_end_snd);
    agent.effect_acmd("effect_specialsbend", special_s_end_eff);
    //end backward air
    agent.game_acmd("game_specialairsbend", special_s_end_game);
    agent.sound_acmd("sound_specialairsbend", special_s_end_snd);
    agent.effect_acmd("effect_specialairsbend", special_s_end_eff);
}