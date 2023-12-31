use crate::imports::*;
use crate::samus::vars::*;
use crate::samus::consts::*;
use crate::samus::other::shinespark::*;

//on/off funcs
pub unsafe fn speedbooster_on(fighter: &mut L2CFighterCommon) {
    shinespark_off(fighter);
    VarModule::on_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON);
    VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_STICK_TIMER, param::SAMUS_INT_SPEEDBOOSTER_STICK_FRAME);
    VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_FALL_TIMER, param::SAMUS_INT_SPEEDBOOSTER_FALL_FRAME);
    VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_TIMER, param::SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_FRAME);
    VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_EFFECT_TIMER, param::SAMUS_INT_SPEEDBOOSTER_EFFECT_FRAME);
    VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_PREV_STATUS_KIND, *FIGHTER_STATUS_KIND_RUN);
    VarModule::set_float(fighter.battle_object, instance::SAMUS_FLOAT_SPEEDBOOSTER_PREV_SPEED_X, param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED);
    JostleModule::set_status(fighter.module_accessor, false);
    //start effects here
    FLASH(fighter, 0.1, 0.2, 3.5, 0.1);
    BURN_COLOR(fighter, 0.4, 1.1, 6.0, 0.5);
    EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.5, true);
    EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 15, 0, -90, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    //sound
    // PLAY_SE_REMAIN(fighter, Hash40::new("se_item_specialflag_raise"));
    PLAY_SE(fighter, Hash40::new("se_item_magicball_warpin"));
}
pub unsafe fn speedbooster_off(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    VarModule::off_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON);
    JostleModule::set_status(fighter.module_accessor, true);
    //correct speed
    if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW { //loop
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPECIAL_LW_STABLE_SPEED,
            0.0
        );
    }else if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
    || status_kind == *FIGHTER_STATUS_KIND_DASH {
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
    COL_NORMAL(fighter);
    BURN_COLOR_NORMAL(fighter);
    //sound
    // STOP_SE(fighter, Hash40::new("se_item_specialflag_raise"));
}
//used in opff
pub unsafe fn speedbooster_effect(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_EFFECT_TIMER) <= 0 {
        VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_EFFECT_TIMER, param::SAMUS_INT_SPEEDBOOSTER_EFFECT_FRAME);
        //effect stuff here
        FLASH(fighter, 0.2, 0.5, 3.0, 0.7);
        BURN_COLOR(fighter, 0.2, 0.4, 7.1, 0.3);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.5, true);
        if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW //loop
        && situation_kind == *SITUATION_KIND_GROUND {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 0, 0, -1, 135, 25, 0, 0.5, true);
        }
    }else {
        if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_EFFECT_TIMER) == param::SAMUS_INT_SPEEDBOOSTER_EFFECT_FRAME / 2 {
            //effect stuff here
            FLASH(fighter, 0.1, 0.2, 3.5, 0.3);
            BURN_COLOR(fighter, 0.4, 1.1, 6.0, 0.7);
            if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW //loop
            && situation_kind == *SITUATION_KIND_GROUND {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 0, 0, -1, 135, 25, 0, 0.5, true);
            }
        }
        VarModule::dec_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_EFFECT_TIMER);
    }
}


//easier transition to running to maintain speedbooster
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_WALK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn walk_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON)
    && (fighter.global_table[0xa].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_LW3
    || fighter.global_table[0xa].get_i32() == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW) {
      fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
      0.into()
    }else {
      original!(fighter)
    }
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn landing_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON)
    && ControlModule::get_stick_x(fighter.module_accessor)*lr > 0.75  {
      fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
      0.into()
    }else {
      original!(fighter)
    }
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_LANDING_LIGHT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn landing_light_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON)
    && ControlModule::get_stick_x(fighter.module_accessor)*lr > 0.75  {
      fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
      0.into()
    }else {
      original!(fighter)
    }
}
//easier input for down-tilt while running
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn attack_dash_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if
    // VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) &&
    ControlModule::get_stick_y(fighter.module_accessor) < -0.4 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), true.into());
        0.into()
    } else {
        original!(fighter)
    }
}


///speedbooster kinetic stuff
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn dash_status_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    original!(fighter);
    let lr = PostureModule::lr(fighter.module_accessor);
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) 
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
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_RUN, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn run_status_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {//motion default speed:1.504 | param default speed:1.654
        WorkModule::set_float(fighter.module_accessor, param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED*0.621125*lr, *FIGHTER_STATUS_RUN_WORK_FLOAT_SPEED);//not sure the conversion formula
    }
    original!(fighter)
}


#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn jump_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
            0.0
        );
        let speed_x = VarModule::get_float(fighter.battle_object, instance::SAMUS_FLOAT_SPEEDBOOSTER_PREV_SPEED_X);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x,
            0.0
        );
    }
    ret
    // 0.into()
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_WALL_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn wall_jump_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    original!(fighter);
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
        // WorkModule::set_int(fighter.module_accessor, i32::MAX, *FIGHTER_STATUS_WALL_JUMP_WORK_INT_DISABLE_CONT_FRAME);//wall-jump attach timer
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_WALL_JUMP_PAST_FRAME);//wall-jump disable timer
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_WALL_JUMP_COUNT);//wall-jumps performed
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_PASSIVE_WALL_WORK_INT_STOP_FRAME);//wall-jump attach timer but weirder
        CancelModule::enable_cancel(fighter.module_accessor);
        MotionModule::set_frame(fighter.module_accessor, 6.0, false);
        VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_FALL_TIMER, param::SAMUS_INT_SPEEDBOOSTER_FALL_FRAME);
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
#[acmd_script( agent = "samus", script = "effect_dash", category = ACMD_EFFECT )]
unsafe fn dash_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
            FOOT_EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }else {
            FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("footr"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "samus", script = "effect_run", category = ACMD_EFFECT )]
unsafe fn run_eff(fighter: &mut L2CAgentBase) {
    // wait_loop_sync_mot();
    frame(fighter.lua_state_agent, 3.0);
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
                EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
                FOOT_EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 3, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(fighter, 0.6);

                EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, -3, 180, 0, 0, 1, true);
                LAST_PARTICLE_SET_COLOR(fighter, 0.2, 0.4, 5.0);
            }else {
                FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
                EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
                FOOT_EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 4, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(fighter, 0.6);

                EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, -3, 180, 0, 0, 1, true);
                LAST_PARTICLE_SET_COLOR(fighter, 0.2, 0.4, 5.0);
            }else {
                FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(fighter.lua_state_agent, 20.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
                EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
                FOOT_EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 4, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(fighter, 0.6);

                EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, -3, 180, 0, 0, 1, true);
                LAST_PARTICLE_SET_COLOR(fighter, 0.2, 0.4, 5.0);
            }else {
                FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
                EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
                FOOT_EFFECT(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 4, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(fighter, 0.6);

                EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12, -3, 180, 0, 0, 1, true);
                LAST_PARTICLE_SET_COLOR(fighter, 0.2, 0.4, 5.0);
            }else {
                FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            }
        }
        wait(fighter.lua_state_agent, 20.0);
    }
}



pub fn install() {
    smashline::install_status_scripts!(
        walk_status_pre,
        landing_status_pre,
        landing_light_status_pre,
        attack_dash_status_pre,

        dash_status_exec,
        run_status_exec,
        
        jump_status_pre,
        wall_jump_status_main,
    );
    smashline::install_acmd_scripts!(
        dash_eff,
        run_eff,
    );
}