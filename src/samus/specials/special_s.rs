use crate::imports::*;
use crate::samus::vars::*;
use crate::samus::consts::*;


////side-special is now Flash-Shift
/// note: statuses _SPECIAL_S1G and _SPECIAL_S2G are unused along with motions "special_s", "special_air_s", "special", & "special_air"
/// note: see opff for additional use of timers and flags

//start
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_s_start_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    //save direction of prev status
    let lr = PostureModule::lr(fighter.module_accessor);
    //exception for turning statuses
    if fighter.global_table[0xa].get_i32() == *FIGHTER_STATUS_KIND_TURN
    || fighter.global_table[0xa].get_i32() == *FIGHTER_STATUS_KIND_TURN_RUN
    || fighter.global_table[0xa].get_i32() == *FIGHTER_STATUS_KIND_TURN_DASH {
        VarModule::set_float(fighter.battle_object, status::SAMUS_FLOAT_SPECIAL_S_LR, lr*-1.0);
    }else {
        VarModule::set_float(fighter.battle_object, status::SAMUS_FLOAT_SPECIAL_S_LR, lr);
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
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s_start_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //easier input for down-special while running
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON)
    && ControlModule::get_stick_y(fighter.module_accessor) < -0.4 {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
        0.into()
    }else {
        //sets lr to prevent side-special turn around
        PostureModule::set_lr(fighter.module_accessor, VarModule::get_float(fighter.battle_object, status::SAMUS_FLOAT_SPECIAL_S_LR));
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        //if facing opposite of stick dir use reverse motions
        if (PostureModule::lr(fighter.module_accessor) == 1.0 && ControlModule::get_stick_x(fighter.module_accessor) > 0.0)
        || (PostureModule::lr(fighter.module_accessor) == -1.0 && ControlModule::get_stick_x(fighter.module_accessor) < 0.0) {
            VarModule::set_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_REVERSE, false);
        }else {
            VarModule::set_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_REVERSE, true);
        }
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_b_start"), 0.0, 1.0, false, 0.0, false, false);
            }else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_f_start"), 0.0, 1.0, false, 0.0, false, false);
            }
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }else {
            if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
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
        if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_USED) {
            VarModule::inc_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_CHAIN_COUNT);
        }else {
            VarModule::on_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_USED);
            // VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_CHAIN_TIMER, param::SAMUS_INT_SPECIAL_S_CHAIN_FRAME);
            VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_RECHARGE_TIMER, param::SAMUS_INT_SPECIAL_S_RECHARGE_FRAME);
            VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_CHAIN_COUNT, 0);
            VarModule::off_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF);
            VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_EFFECT_TRAIL_TIMER, param::SAMUS_FLOAT_SPECIAL_S_EFFECT_TRAIL_FRAME as i32);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(special_s_start_status_loop as *const () as _))
    }
}
pub unsafe fn special_s_start_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A.into(), true.into());
        return true.into()
    }else {
        if StatusModule::situation_kind(fighter.module_accessor) != StatusModule::prev_situation_kind(fighter.module_accessor) {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_s_b_start"), -1.0, 1.0, 0.0);
                }else {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_s_f_start"), -1.0, 1.0, 0.0);
                }
            }else {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_s_b_start"), -1.0, 1.0, 0.0);
                }else {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_s_f_start"), -1.0, 1.0, 0.0);
                }
            }
        }
        return false.into()
    }
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_s_start_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A {
        //added effect removal to fix AFTER_IMAGE(...) bugging out for a single frame when persisting through a status
        AFTER_IMAGE_OFF(fighter, param::SAMUS_FLOAT_SPECIAL_S_EFFECT_TRAIL_FRAME);
        VarModule::on_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF);
    }
    0.into()
}
//loop
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_s_loop_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s_loop_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //sets lr to prevent side-special turn around
    let lr = VarModule::get_float(fighter.battle_object, status::SAMUS_FLOAT_SPECIAL_S_LR);
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
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
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
    VarModule::set_int(fighter.battle_object, status::SAMUS_INT_SPECIAL_S_LOOP_FRAME_TIMER, param::SAMUS_INT_SPECIAL_S_LOOP_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_loop_status_loop as *const () as _))
}
pub unsafe fn special_s_loop_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    if VarModule::get_int(fighter.battle_object, status::SAMUS_INT_SPECIAL_S_LOOP_FRAME_TIMER) <= 0 {
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A.into(), true.into());
        return true.into()
    }else {
        VarModule::dec_int(fighter.battle_object, status::SAMUS_INT_SPECIAL_S_LOOP_FRAME_TIMER);
        if StatusModule::situation_kind(fighter.module_accessor) != StatusModule::prev_situation_kind(fighter.module_accessor) {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                // KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }else {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }else if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
            // corrects for awkward ecb
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
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_s_loop_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    //added effect removal to fix AFTER_IMAGE bugging out for a single frame when persisting through a status
    if fighter.global_table[0xb].get_i32() != *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A {
        AFTER_IMAGE_OFF(fighter, param::SAMUS_FLOAT_SPECIAL_S_EFFECT_TRAIL_FRAME);
        VarModule::on_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF);
    }
    0.into()
}
//end
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_s_end_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn special_s_end_status_exit(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn special_s_end_status_init(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s_end_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //sets lr to prevent side-special turn around
    let lr = VarModule::get_float(fighter.battle_object, status::SAMUS_FLOAT_SPECIAL_S_LR);
    PostureModule::set_lr(fighter.module_accessor, lr);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_b_end"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_f_end"), 0.0, 1.0, false, 0.0, false, false);
        }
    }else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
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
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
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
    VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_S_CHAIN_CANCEL_ENABLE);
    VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_CHAIN_TIMER, param::SAMUS_INT_SPECIAL_S_CHAIN_FRAME);
    // VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_RECHARGE_TIMER, param::SAMUS_INT_SPECIAL_S_RECHARGE_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_end_status_loop as *const () as _))
}
pub unsafe fn special_s_end_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    //air/ground transition
    if StatusModule::situation_kind(fighter.module_accessor) != StatusModule::prev_situation_kind(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) != StatusModule::prev_situation_kind(fighter.module_accessor) {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_s_b_end"), -1.0, 1.0, 0.0);
                }else {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_s_f_end"), -1.0, 1.0, 0.0);
                }
            }else {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_REVERSE) {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_s_b_end"), -1.0, 1.0, 0.0);
                }else {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_s_f_end"), -1.0, 1.0, 0.0);
                }
            }
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
        return true.into()
    //checks to cancel side-special into itself
    }else if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_CHAIN_COUNT) < param::SAMUS_INT_SPECIAL_S_CHAIN_MAX
    && VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_S_CHAIN_CANCEL_ENABLE)
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
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_s_end_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    //added effect removal to fix AFTER_IMAGE bugging out for a single frame when persisting through a status
    if fighter.global_table[0xb].get_i32() != *FIGHTER_STATUS_KIND_SPECIAL_S {
        AFTER_IMAGE_OFF(fighter, param::SAMUS_FLOAT_SPECIAL_S_EFFECT_TRAIL_FRAME);
        VarModule::on_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF);
    }
    0.into()
}

////motion
//start
#[acmd_script( agent = "samus", scripts = ["game_specialsfstart", "game_specialairsfstart", "game_specialsbstart", "game_specialairsbstart"], category = ACMD_GAME )]
unsafe fn special_s_start_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
      JostleModule::set_status(fighter.module_accessor, false);
    }
}
#[acmd_script( agent = "samus", scripts = ["sound_specialsfstart", "sound_specialairsfstart", "sound_specialsbstart", "sound_specialairsbstart"], category = ACMD_SOUND )]
unsafe fn special_s_start_snd(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_smash_s01"));
    }
}
#[acmd_script( agent = "samus", scripts = ["effect_specialsfstart", "effect_specialairsfstart", "effect_specialsbstart", "effect_specialairsbstart"], category = ACMD_EFFECT )]
unsafe fn special_s_start_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        //head
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_samus_trail1"), Hash40::new("tex_samus_trail2"), 60, Hash40::new("head"), 1.0, 0.0, 0.0, Hash40::new("head"), -1.0, 0.0, 0.0, false, Hash40::new("null"), Hash40::new("head"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        //arm-canon
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_samus_trail1"), Hash40::new("tex_samus_trail2"), 60, Hash40::new("haver"), 0.0, 1.0, 0.0, Hash40::new("haver"), 0.0, -1.0, 0.0, false, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        //right foot
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_samus_trail1"), Hash40::new("tex_samus_trail2"), 60, Hash40::new("footr"), 1.0, 0.0, 0.0, Hash40::new("footr"), -1.0, 0.0, 0.0, false, Hash40::new("null"), Hash40::new("footr"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);

        EFFECT_ALPHA(fighter, Hash40::new("sys_genesis_end"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, 0.01);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
        EFFECT_ALPHA(fighter, Hash40::new("sys_deku_flash"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, true, 0.01);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_special_defense_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true, 0.001);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.4, 10.0);
        LAST_EFFECT_SET_RATE(fighter, 0.2);
        FLASH(fighter, 0.2, 0.4, 10.0, 0.5);
        BURN_COLOR(fighter, 1, 1, 1, 0.9);
        // BURN_COLOR_FRAME(fighter, 5, 1, 1, 1, 0.8);
    }
}
//loop
#[acmd_script( agent = "samus", scripts = ["effect_specialsfloop", "effect_specialsbloop"], category = ACMD_EFFECT )]
unsafe fn special_s_loop_eff(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_special_defense_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true, 0.001);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.4, 10.0);
        LAST_EFFECT_SET_RATE(fighter, 0.2);
        FLASH(fighter, 0.2, 0.4, 10.0, 0.5);
        BURN_COLOR(fighter, 1, 1, 1, 0.9);
        // BURN_COLOR_FRAME(fighter, 5, 1, 1, 1, 0.8);
    }
}
//end
#[acmd_script( agent = "samus", scripts = ["game_specialsfend", "game_specialairsfend", "game_specialsbend", "game_specialairsbend"], category = ACMD_GAME )]
unsafe fn special_s_end_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_S_CHAIN_CANCEL_ENABLE);
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
#[acmd_script( agent = "samus", scripts = ["sound_specialsfend", "sound_specialairsfend", "sound_specialsbend", "sound_specialairsbend"], category = ACMD_SOUND )]
unsafe fn special_s_end_snd(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_smash_s02"));
    }
}
#[acmd_script( agent = "samus", scripts = ["effect_specialsfend", "effect_specialairsfend", "effect_specialsbend", "effect_specialairsbend"], category = ACMD_EFFECT )]
unsafe fn special_s_end_eff(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_special_defense_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true, 0.001);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.4, 10.0);
        LAST_EFFECT_SET_RATE(fighter, 0.2);
        FLASH(fighter, 0.2, 0.4, 10.0, 0.5);
        BURN_COLOR(fighter, 1, 1, 1, 0.9);
        // BURN_COLOR_FRAME(fighter, 5, 1, 1, 1, 0.8);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        //added effect removal to fix AFTER_IMAGE bugging out for a single frame when persisting through a status
        AFTER_IMAGE_OFF(fighter, param::SAMUS_FLOAT_SPECIAL_S_EFFECT_TRAIL_FRAME);
        VarModule::on_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_special_defense_up"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        COL_NORMAL(fighter);
        BURN_COLOR_NORMAL(fighter);
    }
}


pub fn install() {
    smashline::install_status_scripts!(
        special_s_start_status_pre,
        special_s_start_status_main,
        special_s_start_status_end,

        special_s_loop_status_pre,
        special_s_loop_status_main,
        special_s_loop_status_end,

        special_s_end_status_pre,
        special_s_end_status_exit,
        special_s_end_status_init,
        special_s_end_status_main,
        special_s_end_status_end
    );
    smashline::install_acmd_scripts!(
        special_s_start_game,
        // special_s_start_exp,
        special_s_start_snd,
        special_s_start_eff,

        // special_s_loop_game,
        // special_s_loop_exp,
        // special_s_loop_snd,
        special_s_loop_eff,

        special_s_end_game,
        // special_s_end_exp,
        special_s_end_snd,
        special_s_end_eff
    );
}