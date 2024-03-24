use crate::imports::*;
use crate::samus::vars::*;
use crate::samus::consts::*;
use crate::samus::other::shinespark::*;

//on/off funcs
pub unsafe fn speedbooster_on(fighter: &mut L2CFighterCommon) {
    shinespark_off(fighter);
    VarModule::on_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON);
    VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_STICK_TIMER, param::SAMUS_INT_SPEEDBOOSTER_STICK_FRAME);
    VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_FALL_TIMER, param::SAMUS_INT_SPEEDBOOSTER_FALL_FRAME);
    VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_TIMER, param::SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_FRAME);
    VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_EFFECT_TIMER, param::SAMUS_INT_SPEEDBOOSTER_EFFECT_FRAME);
    VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_PREV_STATUS_KIND, *FIGHTER_STATUS_KIND_RUN);
    VarModule::set_float(fighter.module_accessor, instance::SAMUS_FLOAT_SPEEDBOOSTER_PREV_SPEED_X, param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED);
    JostleModule::set_status(fighter.module_accessor, false);
    //start effects
    macros::FLASH(fighter, 0.1, 0.2, 3.5, 0.1);
    macros::BURN_COLOR(fighter, 0.4, 1.1, 6.0, 0.5);
    macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
    macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.5, true);
    macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 15, 0, -90, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    //sound
    macros::PLAY_SE(fighter, Hash40::new("se_item_magicball_warpin"));
}
pub unsafe fn speedbooster_off(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    VarModule::off_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON);
    JostleModule::set_status(fighter.module_accessor, true);
    //correct speed
    if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW { //down-special loop status
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPECIAL_LW_STABLE_SPEED,
            0.0
        );
    }else if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
    || status_kind == *FIGHTER_STATUS_KIND_DASH 
    || status_kind == *FIGHTER_STATUS_KIND_SQUAT_WAIT {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RUN_STOP);
    }else if situation_kind != *SITUATION_KIND_GROUND {
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable,
            0.0
        );
    }
    //clear effects
    macros::COL_NORMAL(fighter);
    macros::BURN_COLOR_NORMAL(fighter);
}
//used in opff
pub unsafe fn speedbooster_effect(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_EFFECT_TIMER) <= 0 {
        VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_EFFECT_TIMER, param::SAMUS_INT_SPEEDBOOSTER_EFFECT_FRAME);
        //effect stuff here
        macros::FLASH(fighter, 0.2, 0.5, 3.0, 0.7);
        macros::BURN_COLOR(fighter, 0.2, 0.4, 7.1, 0.3);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.5, true);
        if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW //down-special loop status
        && situation_kind == *SITUATION_KIND_GROUND {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 0, 0, -1, 135, 25, 0, 0.5, true);
        }
    }else {
        if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_EFFECT_TIMER) == param::SAMUS_INT_SPEEDBOOSTER_EFFECT_FRAME / 2 {
            //effect stuff here
            macros::FLASH(fighter, 0.1, 0.2, 3.5, 0.3);
            macros::BURN_COLOR(fighter, 0.4, 1.1, 6.0, 0.7);
            if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW //down-special loop status
            && situation_kind == *SITUATION_KIND_GROUND {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 0, 0, -1, 135, 25, 0, 0.5, true);
            }
        }
        VarModule::dec_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_EFFECT_TIMER);
    }
}
//speedbooster frame checks
pub unsafe fn speedbooster_opff(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    // let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
        //summon effects
        speedbooster_effect(fighter);
        //store shinespark
        if ControlModule::get_flick_y(fighter.module_accessor) <= 5 
        && ControlModule::get_stick_y(fighter.module_accessor) < 0.0 
        && ControlModule::get_stick_x(fighter.module_accessor).abs() < 0.25 {
            if status_kind == *FIGHTER_STATUS_KIND_SQUAT 
            || status_kind == *FIGHTER_STATUS_KIND_SQUAT_WAIT 
            || status_kind == *FIGHTER_STATUS_KIND_LANDING
            || status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT
            || status_kind == *FIGHTER_STATUS_KIND_DASH
            // || status_kind == *FIGHTER_STATUS_KIND_RUN
            || status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE {
                if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) == 0 
                && (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) == 0 
                && (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 {
                    speedbooster_off(fighter);
                    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, true);//shinespark-charge
                }
            }else if (status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
            || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW) //down-special end status
            && CancelModule::is_enable_cancel(fighter.module_accessor) {
                speedbooster_off(fighter);
                StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, true);//shinespark-charge
            }
        }
        //kinetic stuff
        if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_PREV_STATUS_KIND) != status_kind {
            if WorkModule::get_param_float(fighter.module_accessor, smash::hash40("common"), smash::hash40("air_speed_x_limit")) < param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED 
            && situation_kind != *SITUATION_KIND_GROUND {
                sv_kinetic_energy!(
                    set_limit_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
                    0.0
                );
            }
            JostleModule::set_status(fighter.module_accessor, false);
            if 
            // status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 || 
            status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                sv_kinetic_energy!(
                    clear_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_MOTION
                );
                sv_kinetic_energy!(
                    set_stable_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
                    0.0
                );
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED*lr,
                    0.0
                );
            }else 
            //footstool
            if status_kind == *FIGHTER_STATUS_KIND_TREAD_JUMP {
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_TREAD_JUMP_COUNT);//footstools performed
                VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_FALL_TIMER, param::SAMUS_INT_SPEEDBOOSTER_FALL_FRAME);
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, true);
            }else 
            //jump/fall 
            if status_kind == *FIGHTER_STATUS_KIND_JUMP 
            || status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL
            || status_kind == *FIGHTER_STATUS_KIND_FALL
            || status_kind == *FIGHTER_STATUS_KIND_FALL_AERIAL {
                sv_kinetic_energy!(
                    set_stable_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
                    0.0
                );
                let speed_x = VarModule::get_float(fighter.module_accessor, instance::SAMUS_FLOAT_SPEEDBOOSTER_PREV_SPEED_X);
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    speed_x,
                    0.0
                );
            }
            VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_PREV_STATUS_KIND, status_kind);
        }
        //update speed vars after kinetic stuff
        let curr = VarModule::get_float(fighter.module_accessor, instance::SAMUS_FLOAT_SPEEDBOOSTER_CURR_SPEED_X);
        VarModule::set_float(fighter.module_accessor, instance::SAMUS_FLOAT_SPEEDBOOSTER_PREV_SPEED_X, curr);
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) 
            - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) 
            - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
        VarModule::set_float(fighter.module_accessor, instance::SAMUS_FLOAT_SPEEDBOOSTER_CURR_SPEED_X, speed_x);
     
    
        ////ending conditions
        //conditional statuses
        if status_kind == *FIGHTER_STATUS_KIND_WALK
        || status_kind == *FIGHTER_STATUS_KIND_DASH
        || status_kind == *FIGHTER_STATUS_KIND_RUN
        || status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
        || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
        || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S //side-special start status
        || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW //down-special start status
        || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW //down-special loop status
        || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW //down-special end status
        || status_kind == *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP //shinespark-charge
        || status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT
        || status_kind == *FIGHTER_STATUS_KIND_TREAD_JUMP
        || status_kind == *FIGHTER_STATUS_KIND_JUMP
        || status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL
        || status_kind == *FIGHTER_STATUS_KIND_WALL_JUMP
        || status_kind == *FIGHTER_STATUS_KIND_FALL
        || status_kind == *FIGHTER_STATUS_KIND_FALL_AERIAL
        || status_kind == *FIGHTER_STATUS_KIND_LANDING
        || status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT {
            //wall condition
            if (GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32) && lr > 0.0)
            || (GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32) && lr < 0.0) {
                if situation_kind != *SITUATION_KIND_GROUND {
                    if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_TIMER) > 0 {
                        VarModule::dec_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_TIMER);
                    }else {
                        speedbooster_off(fighter);
                    }
                }else {
                    speedbooster_off(fighter);
                }
            }else {
                VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_TIMER, param::SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_FRAME);
            }
            //stick condition
            if ControlModule::get_stick_x(fighter.module_accessor)*lr >= 0.75 {
                VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_STICK_TIMER, param::SAMUS_INT_SPEEDBOOSTER_STICK_FRAME);
            }else if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_STICK_TIMER) > 0 {
                if (status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
                || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW) //start
                && MotionModule::frame(fighter.module_accessor) < (param::SAMUS_INT_SPEEDBOOSTER_STICK_FRAME as f32) {
                    if ControlModule::get_stick_x(fighter.module_accessor)*lr < -0.4 {
                        VarModule::dec_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_STICK_TIMER);
                    }
                }else {
                    VarModule::dec_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_STICK_TIMER);
                }
            }else {
                speedbooster_off(fighter);
            }
            //air condition
            if situation_kind != *SITUATION_KIND_GROUND {
                if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_FALL_TIMER) > 0 {
                    VarModule::dec_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_FALL_TIMER);
                }else {
                    speedbooster_off(fighter);
                }
            }else {
                VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_FALL_TIMER, param::SAMUS_INT_SPEEDBOOSTER_FALL_FRAME);
            }
            //dash-attack
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH 
            && MotionModule::frame(fighter.module_accessor) >= 30.0 {
                speedbooster_off(fighter);
            }
        }else {
            speedbooster_off(fighter);
        }
    //activate speedbooster
    }else {
        if status_kind == *FIGHTER_STATUS_KIND_RUN {
            VarModule::dec_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_START_TIMER);
            if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_START_TIMER) <= 0 {
                speedbooster_on(fighter);
            }
        }else {
            VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_START_TIMER, param::SAMUS_INT_SPEEDBOOSTER_START_FRAME);
        }
    }
}
//easier transition to running to maintain speedbooster
unsafe extern "C" fn walk_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON)
    && (fighter.global_table[0xa].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_LW3
    || fighter.global_table[0xa].get_i32() == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW) {//down-special end status
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
        0.into()
    }else {
        let original = smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_WALK);
        original(fighter)
    }
}
unsafe extern "C" fn landing_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON)
    && ControlModule::get_stick_x(fighter.module_accessor)*lr > 0.75  {
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
        0.into()
    }else {
        let original = smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_LANDING);
        original(fighter)
    }
}
unsafe extern "C" fn landing_light_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON)
    && ControlModule::get_stick_x(fighter.module_accessor)*lr > 0.75  {
        fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
        0.into()
    }else {
        let original = smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_LANDING_LIGHT);
        original(fighter)
    }
}
//easier input for down-tilt while running
unsafe extern "C" fn attack_dash_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if
    // VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) &&
    ControlModule::get_stick_y(fighter.module_accessor) < -0.4 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), true.into());
        0.into()
    } else {
        let original = smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH);
        original(fighter)
    }
}
///speedbooster kinetic stuff
unsafe extern "C" fn dash_status_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_DASH);
    original(fighter);
    let lr = PostureModule::lr(fighter.module_accessor);
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) 
    && (WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DASH_WORK_INT_DASH_TO_RUN_FRAME) as f32) < MotionModule::frame(fighter.module_accessor) {
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED*lr,
            0.0
        );
    }
    0.into()
}
unsafe extern "C" fn run_status_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {//motion default speed:1.504 | param default speed:1.654
        WorkModule::set_float(fighter.module_accessor, param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED*0.621125*lr, *FIGHTER_STATUS_RUN_WORK_FLOAT_SPEED);//not sure the conversion formula
    }
    let original = smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_RUN);
    original(fighter)
}
unsafe extern "C" fn jump_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_JUMP);
    original(fighter);
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
            0.0
        );
        let speed_x = VarModule::get_float(fighter.module_accessor, instance::SAMUS_FLOAT_SPEEDBOOSTER_PREV_SPEED_X);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x,
            0.0
        );
    }
    0.into()
}
unsafe extern "C" fn wall_jump_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_WALL_JUMP);
    original(fighter);
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_WALL_JUMP_PAST_FRAME);//wall-jump lockout timer
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_WALL_JUMP_COUNT);//wall-jumps performed
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_PASSIVE_WALL_WORK_INT_STOP_FRAME);//wall-jump attach timer
        CancelModule::enable_cancel(fighter.module_accessor);
        MotionModule::set_frame(fighter.module_accessor, 6.0, false);
        VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_FALL_TIMER, param::SAMUS_INT_SPEEDBOOSTER_FALL_FRAME);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_WALL_JUMP);
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
            0.0
        );
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED*lr,
            0.0
        );
    }
    0.into()
}
//motion
unsafe extern "C" fn dash_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }else {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footr"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn run_eff(fighter: &mut L2CAgentBase) {
    // sv_animcmd::wait_loop_sync_mot();
    frame(fighter.lua_state_agent, 3.0);
    loop {
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 3, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.6);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, -3, 180, 0, 0, 1, true);
                macros::LAST_PARTICLE_SET_COLOR(fighter, 0.2, 0.4, 5.0);
            }else {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 4, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.6);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, -3, 180, 0, 0, 1, true);
                macros::LAST_PARTICLE_SET_COLOR(fighter, 0.2, 0.4, 5.0);
            }else {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(fighter.lua_state_agent, 20.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 4, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.6);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, -3, 180, 0, 0, 1, true);
                macros::LAST_PARTICLE_SET_COLOR(fighter, 0.2, 0.4, 5.0);
            }else {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 4, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_RATE(fighter, 0.6);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, -3, 180, 0, 0, 1, true);
                macros::LAST_PARTICLE_SET_COLOR(fighter, 0.2, 0.4, 5.0);
            }else {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(fighter.lua_state_agent, 20.0);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    //status
    agent.status(Pre, *FIGHTER_STATUS_KIND_WALK, walk_status_pre);
    agent.status(Pre, *FIGHTER_STATUS_KIND_LANDING, landing_status_pre);
    agent.status(Pre, *FIGHTER_STATUS_KIND_LANDING_LIGHT, landing_light_status_pre);
    agent.status(Pre, *FIGHTER_STATUS_KIND_ATTACK_DASH, attack_dash_status_pre);
    agent.status(Exec, *FIGHTER_STATUS_KIND_DASH, dash_status_exec);
    agent.status(Exec, *FIGHTER_STATUS_KIND_RUN, run_status_exec);
    agent.status(Pre, *FIGHTER_STATUS_KIND_JUMP, jump_status_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_WALL_JUMP, wall_jump_status_main);
    //motion
    agent.effect_acmd("effect_dash", dash_eff);
    agent.effect_acmd("effect_run", run_eff);
}