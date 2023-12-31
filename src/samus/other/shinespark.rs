use crate::imports::*;
use crate::samus::vars::*;
use crate::samus::consts::*;
use crate::samus::other::speedbooster::*;

//on/off funcs
pub unsafe fn shinespark_on(fighter: &mut L2CFighterCommon) {
    VarModule::on_flag(fighter.battle_object, instance::SAMUS_FLAG_SHINESPARK_ON);
    VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SHINESPARK_CHARGE_TIMER, param::SAMUS_INT_SHINESPARK_CHARGE_FRAME);
    VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SHINESPARK_EFFECT_TIMER, 0);
}
pub unsafe fn shinespark_off(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.battle_object, instance::SAMUS_FLAG_SHINESPARK_ON);
    //clear effects
    COL_NORMAL(fighter);
    BURN_COLOR_NORMAL(fighter);
}
//used in opff
pub unsafe fn shinespark_effect(fighter: &mut L2CFighterCommon) {
    if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SHINESPARK_EFFECT_TIMER) <= 0 {
        VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SHINESPARK_EFFECT_TIMER, param::SAMUS_INT_SHINESPARK_EFFECT_FRAME);
        FLASH(fighter, 1.0, 0.0, 4.0, 0.5);
        BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.1);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }else {
        if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SHINESPARK_EFFECT_TIMER) == param::SAMUS_INT_SHINESPARK_EFFECT_FRAME / 2 {
            FLASH(fighter, 1.0, 0.0, 4.0, 0.1);
            BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 0.8, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.8, true);
        }
        VarModule::dec_int(fighter.battle_object, instance::SAMUS_INT_SHINESPARK_EFFECT_TIMER);
    }
}

//////shinespark neutral jump inputs
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn jump_squat_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SHINESPARK_ON) 
    && ControlModule::get_stick_x(fighter.module_accessor).abs() < 0.4 {
        // StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, false);
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A.into(), true.into());//shinespark-start
        0.into()
    }else {
        original!(fighter)
    }
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn jump_aerial_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SHINESPARK_ON) 
    && ControlModule::get_stick_x(fighter.module_accessor).abs() < 0.4 {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A.into(), true.into());//shinespark-start
        0.into()
    }else {
        original!(fighter)
    }
}

//////ready
////status
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn shinespark_ready_status_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn shinespark_ready_status_init(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn shinespark_ready_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_GROUND_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_NONE | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NO_ATTACK_SPECIAL | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_SWIM as u32,
        0,
        0
    );
    0.into()
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn shinespark_ready_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_ready"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(shinespark_ready_status_loop as *const () as _))
}
pub unsafe fn shinespark_ready_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        return true.into()
    }else if StatusModule::situation_kind(fighter.module_accessor) != SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    //cancel into down attacks
    else if MotionModule::frame(fighter.module_accessor) <= 3.0 {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into()); //start
            return true.into()
        }else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
        || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            VarModule::on_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON);
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW3.into(), true.into());
            return true.into()
        }
    }else if MotionModule::frame(fighter.module_accessor) > 3.0 
    && VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
        shinespark_off(fighter);
    }
    return false.into()
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn shinespark_ready_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_LW 
    || fighter.global_table[0xb].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            shinespark_on(fighter);
        }
    }else {
        shinespark_on(fighter);
    }
    0.into()
}
////motion
#[acmd_script( agent = "samus", script = "game_shinesparkready", category = ACMD_GAME )]
unsafe fn shinespark_ready_game(_fighter : &mut L2CAgentBase) {}
#[acmd_script( agent = "samus", script = "expression_shinesparkready", category = ACMD_EXPRESSION )]
unsafe fn shinespark_ready_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
#[acmd_script( agent = "samus", script = "sound_shinesparkready", category = ACMD_SOUND )]
unsafe fn shinespark_ready_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        // PLAY_SE(fighter, Hash40::new("se_common_electric_hit_m"));
        // PLAY_SE(fighter, Hash40::new("se_common_elec_ll_damage"));
        // PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_elec_spark2"));
        // PLAY_SE(fighter, Hash40::new("se_common_spirits_machstamp_landing"));
        PLAY_LANDING_SE(fighter, Hash40::new("se_samus_landing02"));
        PLAY_SE(fighter, Hash40::new("se_item_magicball_warpout"));
        PLAY_SE(fighter, Hash40::new("se_samus_catch"));
        // PLAY_SE(fighter, Hash40::new("se_samus_smash_s01"));
        // PLAY_SE(fighter, Hash40::new("se_samus_smash_s02"));
    }
}
#[acmd_script( agent = "samus", script = "effect_shinesparkready", category = ACMD_EFFECT )]
unsafe fn shinespark_ready_eff(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FLASH(fighter, 0.2, 0.4, 10.0, 0.5);
        BURN_COLOR(fighter, 1, 1, 1, 0.9);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 7, -135, 25, 0, 0.5, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 1, 0.5);
        BURN_COLOR(fighter, 1, 1, 1, 0.9);

        EFFECT_FOLLOW(fighter, Hash40::new("sys_genesis_end"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_deku_flash"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.4, 10.0);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_special_defense_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true, 0.001);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 1.0, true);
        // LAST_EFFECT_SET_COLOR(fighter, 0.2, 0.4, 10.0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 7, -135, 25, 0, 0.5, true);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 7, -135, 25, 0, 0.5, true);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 7, -135, 25, 0, 0.5, true);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        FLASH(fighter, 1.0, 0.0, 4.0, 0.5);
        BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.1);
    }
}
//////start & aim
////status
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn shinespark_start_status_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn shinespark_start_status_init(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn shinespark_start_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ADDITIONS_ATTACK_01 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_SWIM as u32,
        0,
        0
    );
    0.into()
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]/////////////////////////////////////
unsafe fn shinespark_start_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xA].get_i32() == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW { //loop
        VarModule::on_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_start"), 0.0, 1.0, false, 0.0, false, false);
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_lw_loop"), 0.0, 1.0, false, 0.0);
        MotionModule::set_weight(fighter.module_accessor, 0.0, false);
        PostureModule::add_pos(fighter.module_accessor, &mut Vector3f{x:0.0, y:0.01, z:0.0});
    }else {
        VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW);
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_start"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_start"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GROUND_MOVEMENT);
    
    VarModule::off_flag(fighter.battle_object, instance::SAMUS_FLAG_SHINESPARK_ON);
    VarModule::set_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_AIM_TIMER, param::SAMUS_INT_SHINESPARK_AIM_FRAME);
    VarModule::set_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_AIM_EFFECT_TIMER, 14);
    fighter.sub_shift_status_main(L2CValue::Ptr(shinespark_start_status_loop as *const () as _))
}
pub unsafe fn shinespark_start_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {///////////////////////////////////////////////////////////////////////////// 
    if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("shinespark_air_aim") {
        if VarModule::get_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_AIM_TIMER) <= 0 {
        // && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G.into(), true.into());//shinespark-end
            return true.into()
        }else {
            VarModule::dec_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_AIM_TIMER);
            if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW) == false {
                //get stick direction
                let lr = PostureModule::lr(fighter.module_accessor);
                let mut stick_deg = ControlModule::get_stick_angle(fighter.module_accessor).to_degrees();
                if lr < 0.0 {
                    if stick_deg < 0.0 {
                        stick_deg = -180.0 - stick_deg;

                    }else {
                        stick_deg = 180.0 - stick_deg;
                    }
                }
                if stick_deg < 0.0 {
                    stick_deg += 360.0;
                }
                //smoother transition between angles
                let frame_prev = MotionModule::frame(fighter.module_accessor);
                let mut frame = (stick_deg+frame_prev)/2.0;
                if (stick_deg-frame_prev).abs() >= 180.0 {
                    frame += 180.0;
                    if frame >= 360.0 {
                        frame -= 360.0;
                    }
                }
                MotionModule::set_frame(fighter.module_accessor, frame, true);
                //get stick tilt
                let weight_prev = MotionModule::weight(fighter.module_accessor);
                let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
                let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
                let stick_tilt = sv_math::vec2_length(stick_x, stick_y);
                let weight = (stick_tilt+weight_prev)/2.0;
                MotionModule::set_weight(fighter.module_accessor, weight, false);
                //effects
                if VarModule::get_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_AIM_EFFECT_TIMER) <= 0 {
                    VarModule::set_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_AIM_EFFECT_TIMER, 14);
                    FLASH(fighter, 1.0, 0.0, 4.0, 0.4);
                    BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.1);
                    EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, -5, -90, 0, 0, 1.4, true);
                    LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3.5, 3, 0, 0, 0, 1.5, true);
                }else {
                    if VarModule::get_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_AIM_EFFECT_TIMER) == 7 {
                        FLASH(fighter, 1.0, 0.0, 4.0, 0.3);
                        BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.5);
                        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, 5, -90, 0, 0, 1.4, true);
                        LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
                        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3.5, -3, 0, 0, 0, 1.5, true);
                    }
                    VarModule::dec_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_AIM_EFFECT_TIMER);
                }
            }else {
                //effects
                if VarModule::get_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_AIM_EFFECT_TIMER) <= 0 {
                    VarModule::set_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_AIM_EFFECT_TIMER, 14);
                    FLASH(fighter, 1.0, 0.0, 4.0, 0.4);
                    BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.1);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, -3, -90, 0, 0, 0.9, true);
                    LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
                }else {
                    if VarModule::get_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_AIM_EFFECT_TIMER) == 7 {
                        FLASH(fighter, 1.0, 0.0, 4.0, 0.3);
                        BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.5);
                        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, 3, -90, 0, 0, 0.9, true);
                        LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
                        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
                    }
                    VarModule::dec_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_AIM_EFFECT_TIMER);
                }
            }
        }
    }else if MotionModule::is_end(fighter.module_accessor) {
        if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW) == false {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_aim"), 0.0, 0.0, false, 0.0, false, false);
            let end_frame = MotionModule::end_frame(fighter.module_accessor);
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("shinespark_air_aim"), end_frame, 0.0, false, 0.0);
            MotionModule::set_weight(fighter.module_accessor, 0.0, false);
        }else {
            let frame_2nd = MotionModule::frame_2nd(fighter.module_accessor);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_aim"), 0.0, 0.0, false, 0.0, false, false);
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_lw_loop"), frame_2nd, param::SAMUS_FLOAT_SHINESPARK_SPECIAL_LW_SPIN_SPEED_STABLE, false, 0.0);
            MotionModule::set_weight(fighter.module_accessor, 0.0, false);
        }
    }else if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW) {
        // let mut rate = MotionModule::rate_2nd(fighter.module_accessor);
        // DamageModule::add_damage(fighter.module_accessor, rate, 0);

        // if rate < param::SAMUS_FLOAT_SHINESPARK_SPECIAL_LW_SPIN_SPEED_STABLE {
        //     rate = rate *(1.0/param::SAMUS_FLOAT_SPECIAL_LW_AIR_SPIN_SPEED_ACCEL);
        // }else if rate > param::SAMUS_FLOAT_SHINESPARK_SPECIAL_LW_SPIN_SPEED_STABLE {
        //     rate = param::SAMUS_FLOAT_SHINESPARK_SPECIAL_LW_SPIN_SPEED_STABLE
        // }
        // MotionModule::set_rate_2nd(fighter.module_accessor, rate);
        
        println!("frame :{:?}", MotionModule::frame(fighter.module_accessor));
        println!("frame_2nd :{:?}", MotionModule::frame_2nd(fighter.module_accessor));
        println!("rate_2nd :{:?}", MotionModule::rate_2nd(fighter.module_accessor));
        let mut rate = MotionModule::frame(fighter.module_accessor).powf(1.25);
        
        if rate > param::SAMUS_FLOAT_SHINESPARK_SPECIAL_LW_SPIN_SPEED_STABLE {
            rate = param::SAMUS_FLOAT_SHINESPARK_SPECIAL_LW_SPIN_SPEED_STABLE;
        }
        println!("new_rate_2nd :{:?}", rate);
        MotionModule::set_rate_2nd(fighter.module_accessor, rate);
    }
    return false.into()
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn shinespark_start_status_end(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
////motion
//start
#[acmd_script( agent = "samus", scripts = ["game_shinesparkstart", "game_shinesparkairstart"], category = ACMD_GAME )]
unsafe fn shinespark_start_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 25.0);
    }
}
#[acmd_script( agent = "samus", scripts = ["expression_shinesparkstart", "expression_shinesparkairstart"], category = ACMD_EXPRESSION )]
unsafe fn shinespark_start_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_32_hold_lv1"), 0, true, 0);
        if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
            ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        }
    }
}
#[acmd_script( agent = "samus", scripts = ["sound_shinesparkstart", "sound_shinesparkairstart"], category = ACMD_SOUND )]
unsafe fn shinespark_start_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_superscope_charge"));
    }
}
#[acmd_script( agent = "samus", scripts = ["effect_shinesparkstart", "effect_shinesparkairstart"], category = ACMD_EFFECT )]
unsafe fn shinespark_start_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 5.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 4.0);
        // ColorBlendModule::set_disable_camera_depth_influence(fighter.module_accessor, true);
        FLASH(fighter, 1.0, 0.0, 4.0, 0.5);
        BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.5);
    }
    if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW) == false {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, -5, -90, 0, 0, 1.4, true);
            LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
            LAST_EFFECT_SET_RATE(fighter, 0.3);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3.5, 3, 0, 0, 0, 1.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
        }
        wait(fighter.lua_state_agent, 9.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, 5, -90, 0, 0, 1.4, true);
            LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
            LAST_EFFECT_SET_RATE(fighter, 0.4);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3.5, -3, 0, 0, 0, 1.5, true);
        }
        wait(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 2, -5, -90, 0, 0, 1.4, true);
            LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3.5, 3, 0, 0, 0, 1.5, true);
        }
    }else{
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, -3, -90, 0, 0, 0.5, true);
            LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
            LAST_EFFECT_SET_RATE(fighter, 0.3);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
        }
        wait(fighter.lua_state_agent, 9.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, 3, -90, 0, 0, 0.5, true);
            LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
            LAST_EFFECT_SET_RATE(fighter, 0.4);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
        }
        wait(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 0, -3, -90, 0, 0, 0.5, true);
            LAST_PARTICLE_SET_COLOR(fighter, 0.5, 0.2, 0.5);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
        }
    }
}

//aim
// #[acmd_script( agent = "samus", script = "game_shinesparkairaim", category = ACMD_GAME )]
// unsafe fn shinespark_air_aim_game(_fighter : &mut L2CAgentBase) {}
// #[acmd_script( agent = "samus", script = "expression_shinesparkairaim", category = ACMD_EXPRESSION )]
// unsafe fn shinespark_air_aim_exp(_fighter : &mut L2CAgentBase) {}
// #[acmd_script( agent = "samus", script = "sound_shinesparkairaim", category = ACMD_SOUND )]
// unsafe fn shinespark_air_aim_snd(_fighter : &mut L2CAgentBase) {}
// #[acmd_script( agent = "samus", script = "effect_shinesparkairaim", category = ACMD_EFFECT )]
// unsafe fn shinespark_air_aim_eff(_fighter : &mut L2CAgentBase) {}

//////loop & end
////status
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn shinespark_end_status_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn shinespark_end_status_init(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn shinespark_end_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ADDITIONS_ATTACK_01 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_SWIM as u32,
        0,
        0
    );
    0.into()
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]////////////////////////////////////////
unsafe fn shinespark_end_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    //determine direction
    let speed_x: f32;
    let speed_y: f32;
    if stick_x > 0.4 
    || stick_x < -0.4 {
        if stick_x*lr < 0.0 {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            lr *= -1.0;
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_loop_s"), 0.0, 1.0, false, 0.0, false, false);
        if stick_y > 0.4 {
            //side-up
            let diagonal = ((param::SAMUS_FLOAT_SHINESPARK_SPEED.powi(2))/2.0).sqrt();
            speed_x = diagonal;
            speed_y = diagonal;
        }else if stick_y < -0.4 {
            //side-down
            let diagonal = ((param::SAMUS_FLOAT_SHINESPARK_SPEED.powi(2))/2.0).sqrt();
            speed_x = diagonal;
            speed_y = diagonal*-1.0;
        }else {
            //side
            speed_x = param::SAMUS_FLOAT_SHINESPARK_SPEED;
            speed_y = 0.0;
        }
    }else if stick_y < -0.4 {
        //down
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_loop_lw"), 0.0, 1.0, false, 0.0, false, false);
        speed_x = 0.0;
        speed_y = param::SAMUS_FLOAT_SHINESPARK_SPEED*-1.0;
    }else {
        //up
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_loop_hi"), 0.0, 1.0, false, 0.0, false, false);
        speed_x = 0.0;
        speed_y = param::SAMUS_FLOAT_SHINESPARK_SPEED;
    }
    if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_loop_s"), 0.0, 1.0, false, 0.0, false, false);
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_lw_loop"), 0.0, param::SAMUS_FLOAT_SHINESPARK_SPECIAL_LW_SPIN_SPEED_STABLE, false, 0.0);
        MotionModule::set_weight(fighter.module_accessor, 0.0, false);
        // VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
        // ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        // ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
    }
    //set kinetic energy
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GROUND_MOVEMENT);
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        param::SAMUS_FLOAT_SHINESPARK_SPEED,
        param::SAMUS_FLOAT_SHINESPARK_SPEED
    );
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        param::SAMUS_FLOAT_SHINESPARK_SPEED,
        param::SAMUS_FLOAT_SHINESPARK_SPEED
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        speed_x*lr,
        speed_y
    );
    VarModule::set_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_LOOP_TIMER, param::SAMUS_INT_SHINESPARK_LOOP_FRAME);
    VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_GRAVITY);
    VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_CONTROL);
    fighter.sub_shift_status_main(L2CValue::Ptr(shinespark_end_status_loop as *const () as _))
}
pub unsafe fn shinespark_end_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {//////////////////////////////////////////////////////////////////////////////////
    //fly-loop
    if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("shinespark_air_loop_hi") {
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
            shinespark_air_ceil(fighter);
        }
        if VarModule::get_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_LOOP_TIMER) <= 0 {
            shinespark_air_end(fighter);
        }else if SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) < 2
        && StopModule::is_stop(fighter.module_accessor) == false {
            VarModule::dec_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_LOOP_TIMER);
        }
    }else if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("shinespark_air_loop_s") {
        if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW) == false {
            let mut angle = 0.0;
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                let vec_rot = &mut Vector2f{x:0.0, y:0.0};
                FighterUtil::get_air_ground_touch_info(fighter.module_accessor, &mut Vector2f{x:0.0, y:0.0}, vec_rot);
                angle = (-vec_rot.x).atan2(vec_rot.y).to_degrees();
            }
            let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            let stop_speed_y = KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(stop_energy));
            if stop_speed_y > 0.0 
            && GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) {
                shinespark_air_ceil(fighter);
            }else if stop_speed_y < 0.0 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                shinespark_landing(fighter);
            }else if stop_speed_y == 0.0 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
            && (
                (angle > 5.0 && angle < 50.0) ||
                (angle < -5.0 && angle > -50.0)
            ) {
                speedbooster_on(fighter);
                fighter.change_status(FIGHTER_STATUS_KIND_DASH.into(), true.into());
                return true.into()
            }else if (GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) 
            && PostureModule::lr(fighter.module_accessor) > 0.0) 
            || (GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) 
            && PostureModule::lr(fighter.module_accessor) < 0.0) {
                let pos =  PostureModule::pos(fighter.module_accessor);
                let ground_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, pos, 90.0, true);
                if ground_dist < 2.0
                && ground_dist > 0.0 {
                    // PostureModule::add_pos(fighter.module_accessor, &mut Vector3f{x:0.0, y:-ground_dist, z:0.0});
                    shinespark_wall(fighter);
                }else {
                    shinespark_air_wall(fighter);
                }
            }else if VarModule::get_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_LOOP_TIMER) <= 0 {
                let pos =  PostureModule::pos(fighter.module_accessor);
                let ground_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, pos, 90.0, true);
                if ground_dist < 2.0
                && ground_dist > 0.0 {
                    PostureModule::add_pos(fighter.module_accessor, &mut Vector3f{x:0.0, y:-ground_dist, z:0.0});
                    shinespark_end(fighter);
                }else {
                    shinespark_air_end(fighter);
                }
            }else if SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) < 2
            && StopModule::is_stop(fighter.module_accessor) == false {
                VarModule::dec_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_LOOP_TIMER);
            }
        }else {
            if VarModule::get_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_LOOP_TIMER) > 0 { 
                //collision check
                let angle;
                let stop_speed_y;
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                    let vec_rot = &mut Vector2f{x:0.0, y:0.0};
                    FighterUtil::get_air_ground_touch_info(fighter.module_accessor, &mut Vector2f{x:0.0, y:0.0}, vec_rot);
                    angle = (-vec_rot.x).atan2(vec_rot.y).to_degrees();
                    let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                    stop_speed_y = KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(stop_energy));
                }else {
                    angle = 0.0;
                    stop_speed_y = 0.0;
                }
                //wall/ceil
                if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32)
                || (GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) 
                && PostureModule::lr(fighter.module_accessor) > 0.0) 
                || (GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) 
                && PostureModule::lr(fighter.module_accessor) < 0.0) {
                    request_wall_effect(fighter.module_accessor);
                    shinespark_special_lw_wall(fighter);
                }else
                //landing 
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND 
                && stop_speed_y < 0.0 {
                    LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
                    shinespark_special_lw_wall(fighter);
                }else 
                //continue speedbooster
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND 
                && ((angle > 5.0  && angle < 50.0) 
                ||  (angle < -5.0 && angle > -50.0)) {
                    speedbooster_on(fighter);
                    fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW.into(), true.into()); //loop
                    return true.into()
                }else 
                //count down
                if SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) < 2
                && StopModule::is_stop(fighter.module_accessor) == false {
                    VarModule::dec_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_LOOP_TIMER);
                }
            }else {
                shinespark_special_lw_end(fighter);
            }
        }
    }else if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("shinespark_air_loop_lw") {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            shinespark_landing(fighter);
        }else if VarModule::get_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_LOOP_TIMER) <= 0 {
            shinespark_air_end(fighter);
        }else if SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) < 2
        && StopModule::is_stop(fighter.module_accessor) == false {
            VarModule::dec_int(fighter.battle_object, status::SAMUS_INT_SHINESPARK_LOOP_TIMER);
        }
    }
    //ground-end
    else if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("shinespark_end") 
    || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("shinespark_wall") 
    || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("shinespark_landing") {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
            return true.into()
        }else if StatusModule::situation_kind(fighter.module_accessor) != SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
            return true.into()
        }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return true.into()
            }
        }
    }
    //air-end
    else if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("shinespark_air_end") 
    || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("shinespark_air_ceil") 
    || MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("shinespark_air_wall") {
        if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW) == false {
            if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_GRAVITY) {
                VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_GRAVITY);
                let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, -1);
                let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, -1);
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    speed_x,
                    0.0
                );
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    speed_y
                );
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_CONTROL) {
                VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_CONTROL);
                let stable_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, -1);
                sv_kinetic_energy!(
                    set_stable_speed, 
                    fighter, 
                    FIGHTER_KINETIC_ENERGY_ID_STOP, 
                    0.0, 
                    stable_y
                );
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    speed_x,
                    0.0
                );
                sv_kinetic_energy!(
                    reset_energy,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST_NO_CAP,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0
                );
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            }
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), true.into());
                return true.into()
            }else if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), true.into());
                return true.into()
            }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return true.into()
                }
            }
        }else {


            let mut rate = MotionModule::rate_2nd(fighter.module_accessor);
            if rate > param::SAMUS_FLOAT_SPECIAL_LW_AIR_SPIN_SPEED_MAX {
                rate -= param::SAMUS_FLOAT_SPECIAL_LW_AIR_SPIN_SPEED_BRAKE;
                MotionModule::set_rate_2nd(fighter.module_accessor, rate);
            }
            // else if rate < param::SAMUS_FLOAT_SPECIAL_LW_AIR_SPIN_SPEED_MAX {
            //     rate = param::SAMUS_FLOAT_SPECIAL_LW_AIR_SPIN_SPEED_MAX
            // }


            if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_GRAVITY) {
                VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_GRAVITY);
                let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, -1);
                let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, -1);
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_STOP,
                    speed_x,
                    0.0
                );
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    speed_y
                );
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }else if MotionModule::is_end(fighter.module_accessor) 
            || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND 
            || VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_CONTROL) {
                fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW.into(), true.into()); //loop
                return true.into()
            }

        }
    }
    return false.into()
}
//summon effect with rotation relative to ceil/wall/floor
pub unsafe fn request_wall_effect(boma: *mut BattleObjectModuleAccessor) {
    let touch_pos = &mut Vector2f{x:0.0, y:0.0};
    let touch_rot = &mut Vector2f{x:0.0, y:0.0};
    FighterUtil::get_air_ground_touch_info(boma, touch_pos, touch_rot);
    let pos_z = GroundModule::get_z(boma);
    let rot = (-touch_rot.x).atan2(touch_rot.y);
    EffectModule::req(boma, Hash40::new("sys_crown"), &Vector3f{x:touch_pos.x, y:touch_pos.y, z:pos_z}, &Vector3f{x:0.0, y:0.0, z:rot}, 1.0, 0, -1, false, 0);
}
//ending funcs
pub unsafe fn shinespark_end(fighter: &mut L2CFighterCommon) {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_end"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    sv_kinetic_energy!(
        set_stable_speed, 
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP, 
        0.0, 
        0.0
    );
    sv_kinetic_energy!(
        set_brake, 
        fighter, 
        FIGHTER_KINETIC_ENERGY_ID_STOP, 
        param::SAMUS_FLOAT_SHINESPARK_GROUND_BRAKE
    );
}
pub unsafe fn shinespark_air_end(fighter: &mut L2CFighterCommon) {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_end"), 0.0, 1.0, false, 0.0, false, false);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    let stable_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let stable_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    sv_kinetic_energy!(
        set_stable_speed, 
        fighter, 
        FIGHTER_KINETIC_ENERGY_ID_STOP, 
        stable_x, 
        stable_y
    );
    sv_kinetic_energy!(
        set_brake, 
        fighter, 
        FIGHTER_KINETIC_ENERGY_ID_STOP, 
        param::SAMUS_FLOAT_SHINESPARK_AIR_BRAKE, 
        param::SAMUS_FLOAT_SHINESPARK_AIR_BRAKE
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        param::SAMUS_FLOAT_SHINESPARK_AIR_BRAKE*-1.0
    );
}
pub unsafe fn shinespark_air_ceil(fighter: &mut L2CFighterCommon) {
    request_wall_effect(fighter.module_accessor);

    let touch_pos = &mut Vector2f{x:0.0, y:0.0};
    FighterUtil::get_air_ground_touch_info(fighter.module_accessor, touch_pos, &mut Vector2f{x:0.0, y:0.0});
    // let lr = PostureModule::lr(fighter.module_accessor);
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    // let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);
    PostureModule::set_pos(fighter.module_accessor, &Vector3f{x:pos_x, y:touch_pos.y-17.0, z:pos_z});//offset trans from ceiling

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_ceil"), 0.0, 1.0, false, 0.0, false, false);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );
}
pub unsafe fn shinespark_wall(fighter: &mut L2CFighterCommon) {
    // request_wall_effect(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = GroundModule::get_z(fighter.module_accessor);
    let ground_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, &Vector3f{x:pos_x-(5.5*lr), y:pos_y, z:pos_z}, 90.0, true);
    let hit_pos = &mut Vector2f{x:0.0, y:0.0};
    let hit_rot = &mut Vector2f{x:0.0, y:0.0};
    let is_hit = GroundModule::ray_check_hit_pos_normal(fighter.module_accessor, &Vector2f{x:pos_x, y:pos_y+7.0}, &Vector2f{x:10.0*lr, y:0.1}, hit_pos, hit_rot, true);
    if is_hit == 1 {
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x:hit_pos.x-(5.5*lr), y:pos_y-ground_dist, z:pos_z});//offset trans from wall
        let rot = (-hit_rot.x).atan2(hit_rot.y);
        EffectModule::req(fighter.module_accessor, Hash40::new("sys_crown"), &Vector3f{x:hit_pos.x, y:hit_pos.y, z:pos_z}, &Vector3f{x:0.0, y:0.0, z:rot}, 1.0, 0, -1, false, 0);
    }else {
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x:pos_x, y:pos_y-ground_dist, z:pos_z});
        EffectModule::req(fighter.module_accessor, Hash40::new("sys_crown"), &Vector3f{x:pos_x+5.5, y:pos_y+7.0, z:pos_z}, &Vector3f{x:0.0, y:0.0, z:90.0}, 1.0, 0, -1, false, 0);
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_wall"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
}
pub unsafe fn shinespark_air_wall(fighter: &mut L2CFighterCommon) {
    request_wall_effect(fighter.module_accessor);

    let touch_pos = &mut Vector2f{x:0.0, y:0.0};
    FighterUtil::get_air_ground_touch_info(fighter.module_accessor, touch_pos, &mut Vector2f{x:0.0, y:0.0});
    let lr = PostureModule::lr(fighter.module_accessor);
    // let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);
    PostureModule::set_pos(fighter.module_accessor, &Vector3f{x:touch_pos.x -(5.5*lr), y:pos_y, z:pos_z});//offset trans from wall

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_wall"), 0.0, 1.0, false, 0.0, false, false);
    
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );
}
pub unsafe fn shinespark_landing(fighter: &mut L2CFighterCommon) {
    // request_wall_effect(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_landing"), 0.0, 1.0, false, 0.0, false, false);
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
}
pub unsafe fn shinespark_special_lw_wall(fighter: &mut L2CFighterCommon) {////////////////////////////////////////////////////////////////////////////////////////////
    let frame_2nd = MotionModule::frame_2nd(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_wall"), 0.0, 1.0, false, 0.0, false, false);
    MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_lw_loop"), frame_2nd, 0.0, false, 0.0);
    MotionModule::set_weight(fighter.module_accessor, 0.0, false);

    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );
}
pub unsafe fn shinespark_special_lw_end(fighter: &mut L2CFighterCommon) {////////////////////////////////////////////////////////////////////////////////////////////
    let frame_2nd = MotionModule::frame_2nd(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("shinespark_air_end"), 0.0, 1.0, false, 0.0, false, false);
    MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_lw_loop"), frame_2nd, param::SAMUS_FLOAT_SHINESPARK_SPECIAL_LW_SPIN_SPEED_STABLE, false, 0.0);
    MotionModule::set_weight(fighter.module_accessor, 0.0, false);

    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
    sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    let stable_x = param::SAMUS_FLOAT_SPECIAL_LW_STABLE_SPEED;
    let stable_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    sv_kinetic_energy!(
        set_stable_speed, 
        fighter, 
        FIGHTER_KINETIC_ENERGY_ID_STOP, 
        stable_x, 
        stable_y
    );
    sv_kinetic_energy!(
        set_brake, 
        fighter, 
        FIGHTER_KINETIC_ENERGY_ID_STOP, 
        param::SAMUS_FLOAT_SHINESPARK_AIR_BRAKE, 
        param::SAMUS_FLOAT_SHINESPARK_AIR_BRAKE
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        param::SAMUS_FLOAT_SHINESPARK_AIR_BRAKE*-1.0
    );
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn shinespark_end_status_end(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
////motion
//loop
#[acmd_script( agent = "samus", script = "game_shinesparkairloophi", category = ACMD_GAME )]
unsafe fn shinespark_loop_hi_game(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 90, 0, 80, 8.0, 0.0, 11.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
}
#[acmd_script( agent = "samus", script = "game_shinesparkairloops", category = ACMD_GAME )]
unsafe fn shinespark_loop_s_game(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW) == false {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 90, 0, 80, 8.0, 0.0, 9.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        }else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 90, 0, 80, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        }
    }
}
#[acmd_script( agent = "samus", script = "game_shinesparkairlooplw", category = ACMD_GAME )]
unsafe fn shinespark_loop_lw_game(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 90, 0, 80, 8.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
}





#[acmd_script( agent = "samus", scripts = ["expression_shinesparkairloophi", "expression_shinesparkairloops", "expression_shinesparkairlooplw"], category = ACMD_EXPRESSION )]
unsafe fn shinespark_loop_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beaml"), 0, false, 0);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);

        if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
            ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        }
    }
    
}
#[acmd_script( agent = "samus", scripts = ["sound_shinesparkairloophi", "sound_shinesparkairloops", "sound_shinesparkairlooplw"], category = ACMD_SOUND )]
unsafe fn shinespark_loop_snd(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        // PLAY_SE(fighter, Hash40::new("se_item_magicball_warpin"));
        PLAY_SE(fighter, Hash40::new("se_item_superscope_chargeshot_l"));
    }
}
#[acmd_script( agent = "samus", script = "effect_shinesparkairloophi", category = ACMD_EFFECT )]
unsafe fn shinespark_loop_hi_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 5.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 4.0);
        FLASH(fighter, 1.0, 0.0, 4.0, 0.5);
        BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.1);
        EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 3, 0, 180, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, -90, 0, 0, 1, true);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_drill"), Hash40::new("top"), 0, 10, 0, -90, 0, 0, 1, true);
    }
    for _ in 0..16 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, -90, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.5, true);
        }
        wait(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, -90, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.5, true);
        }
        wait(fighter.lua_state_agent, 10.0);
    }
}
#[acmd_script( agent = "samus", script = "effect_shinesparkairloops", category = ACMD_EFFECT )]
unsafe fn shinespark_loop_s_eff(fighter : &mut L2CAgentBase) {
    if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW) == false {
        let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let stop_speed_y = KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(stop_energy));
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 5.0, true);
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 4.0);
            FLASH(fighter, 1.0, 0.0, 4.0, 0.5);
            BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.1);
            if stop_speed_y > 0.0 {
                EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 10, -5, -135, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, -45, 0, 0, 1, true);
            }else if stop_speed_y < 0.0 {
                EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 10, -5, -45, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 45, 0, 0, 1, true);
            }else{
                EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 10, -5, -90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
            }
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            if stop_speed_y > 0.0 {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_drill"), Hash40::new("top"), 0, 10, 0, -45, 0, 0, 1, true);
            }else if stop_speed_y < 0.0 {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_drill"), Hash40::new("top"), 0, 10, 0, 45, 0, 0, 1, true);
            }else{
                EFFECT_FOLLOW(fighter, Hash40::new("sys_drill"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
            }
        }
        for _ in 0..16 {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.5, true);
                if stop_speed_y > 0.0 {
                    EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, -45, 0, 0, 1, true);
                }else if stop_speed_y < 0.0 {
                    EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 45, 0, 0, 1, true);
                }else{
                    EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
                    let ground_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, PostureModule::pos(fighter.module_accessor), 90.0, true);
                    if ground_dist < 1.0 
                    && ground_dist >= 0.0 {
                        LANDING_EFFECT(fighter, Hash40::new("samus_cshot_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
                    }
                }
            }
            wait(fighter.lua_state_agent, 5.0);
            if is_excute(fighter) {
                let ground_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, PostureModule::pos(fighter.module_accessor), 90.0, true);
                if stop_speed_y == 0.0 
                && ground_dist < 1.0 && ground_dist >= 0.0 {
                    LANDING_EFFECT(fighter, Hash40::new("samus_cshot_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
                }
            }
            wait(fighter.lua_state_agent, 5.0);
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.5, true);
                if stop_speed_y > 0.0 {
                    EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, -45, 0, 0, 1, true);
                }else if stop_speed_y < 0.0 {
                    EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 45, 0, 0, 1, true);
                }else{
                    EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
                    let ground_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, PostureModule::pos(fighter.module_accessor), 90.0, true);
                    if ground_dist < 1.0 
                    && ground_dist >= 0.0 {
                        LANDING_EFFECT(fighter, Hash40::new("samus_cshot_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
                    }
                }
            }
            wait(fighter.lua_state_agent, 5.0);
            if is_excute(fighter) {
                let ground_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, PostureModule::pos(fighter.module_accessor), 90.0, true);
                if stop_speed_y == 0.0 
                && ground_dist < 1.0 && ground_dist >= 0.0 {
                    LANDING_EFFECT(fighter, Hash40::new("samus_cshot_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
                }
            }
            wait(fighter.lua_state_agent, 5.0);
        }
    }else {
        let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let stop_speed_x = KineticEnergy::get_speed_x(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(stop_energy));
        let stop_speed_y = KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(stop_energy));
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 3.0, true);
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 4.0);
            FLASH(fighter, 1.0, 0.0, 4.0, 0.5);
            BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.1);
            if stop_speed_y > 0.0 {
                if stop_speed_x.abs() > 0.0 {
                    EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 5, 2, -135, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                    EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, -45, 0, 0, 1, true);
                }else {
                    EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 5, 0, 180, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                    EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, -90, 0, 0, 1, true);
                }
            }else if stop_speed_y < 0.0 {
                if stop_speed_x.abs() > 0.0 {
                    EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 1, 2, -45, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                    EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, 45, 0, 0, 1, true);
                }else {
                    EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                    EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, 90, 0, 0, 1, true);
                }
            }else{
                EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 3, 2, -90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
            }
        }
        wait(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            if stop_speed_y > 0.0 {
                if stop_speed_x.abs() > 0.0 {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_drill"), Hash40::new("top"), 0, 3, 0, -45, 0, 0, 1, true);
                }else {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_drill"), Hash40::new("top"), 0, 3, 0, -90, 0, 0, 1, true);
                }
            }else if stop_speed_y < 0.0 {
                if stop_speed_x.abs() > 0.0 {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_drill"), Hash40::new("top"), 0, 3, 0, 45, 0, 0, 1, true);
                }else {
                    EFFECT_FOLLOW(fighter, Hash40::new("sys_drill"), Hash40::new("top"), 0, 3, 0, 90, 0, 0, 1, true);
                }
            }else{
                EFFECT_FOLLOW(fighter, Hash40::new("sys_drill"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
            }
        }
        for _ in 0..16 {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
                if stop_speed_y > 0.0 {
                    if stop_speed_x.abs() > 0.0 {
                        EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, -45, 0, 0, 1, true);
                    }else {
                        EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, -90, 0, 0, 1, true);
                    }
                }else if stop_speed_y < 0.0 {
                    if stop_speed_x.abs() > 0.0 {
                        EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, 45, 0, 0, 1, true);
                    }else {
                        EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, 90, 0, 0, 1, true);
                    }
                }else{
                    EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
                    let ground_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, PostureModule::pos(fighter.module_accessor), 90.0, true);
                    if ground_dist < 1.0 
                    && ground_dist >= 0.0 {
                        LANDING_EFFECT(fighter, Hash40::new("samus_cshot_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                    }
                }
            }
            wait(fighter.lua_state_agent, 5.0);
            if is_excute(fighter) {
                let ground_dist = GroundModule::get_distance_to_floor(fighter.module_accessor, PostureModule::pos(fighter.module_accessor), 90.0, true);
                if stop_speed_y == 0.0 
                && ground_dist < 1.0 && ground_dist >= 0.0 {
                    LANDING_EFFECT(fighter, Hash40::new("samus_cshot_ground"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
                }
            }
            wait(fighter.lua_state_agent, 5.0);
        }
    }
}
#[acmd_script( agent = "samus", script = "effect_shinesparkairlooplw", category = ACMD_EFFECT )]
unsafe fn shinespark_loop_lw_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 5.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 4.0);
        FLASH(fighter, 1.0, 0.0, 4.0, 0.5);
        BURN_COLOR(fighter, 1.0, 0.0, 4.0, 0.1);
        EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 90, 0, 0, 1, true);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_drill"), Hash40::new("top"), 0, 10, 0, 90, 0, 0, 1, true);
    }
    for _ in 0..16 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 90, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.5, true);
        }
        wait(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("samus_dash_attack"), Hash40::new("top"), 0, 10, 0, 90, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.5, true);
        }
        wait(fighter.lua_state_agent, 10.0);
    }
}
//end-ground
#[acmd_script( agent = "samus", script = "game_shinesparkend", category = ACMD_GAME )]
unsafe fn shinespark_end_game(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "samus", script = "expression_shinesparkend", category = ACMD_EXPRESSION )]
unsafe fn shinespark_end_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_LR, 6);
    }
}
#[acmd_script( agent = "samus", script = "sound_shinesparkend", category = ACMD_SOUND )]
unsafe fn shinespark_end_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_step_left_m"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_step_right_m"));
    }
}
#[acmd_script( agent = "samus", script = "effect_shinesparkend", category = ACMD_EFFECT )]
unsafe fn shinespark_end_eff(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_drill"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        COL_NORMAL(fighter);
        BURN_COLOR_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
//end-air
#[acmd_script( agent = "samus", script = "game_shinesparkairend", category = ACMD_GAME )]
unsafe fn shinespark_air_end_game(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_GRAVITY);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_CONTROL);
    }
}
#[acmd_script( agent = "samus", script = "expression_shinesparkairend", category = ACMD_EXPRESSION )]
unsafe fn shinespark_air_end_exp(_fighter : &mut L2CAgentBase) {}
#[acmd_script( agent = "samus", script = "sound_shinesparkairend", category = ACMD_SOUND )]
unsafe fn shinespark_air_end_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_samus_jump02"));
    }
}
#[acmd_script( agent = "samus", script = "effect_shinesparkairend", category = ACMD_EFFECT )]
unsafe fn shinespark_air_end_eff(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_drill"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        COL_NORMAL(fighter);
        BURN_COLOR_NORMAL(fighter);
    }
}
//ceilling
#[acmd_script( agent = "samus", script = "game_shinesparkairceil", category = ACMD_GAME )]
unsafe fn shinespark_ceil_game(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 355, 70, 0, 80, 6.5, 0.0, 19.0, -10.0, Some(0.0), Some(19.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_GRAVITY);
    }
    frame(fighter.lua_state_agent, 52.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_CONTROL);
    }
}
#[acmd_script( agent = "samus", script = "expression_shinesparkairceil", category = ACMD_EXPRESSION )]
unsafe fn shinespark_ceil_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
    }
}
#[acmd_script( agent = "samus", script = "sound_shinesparkairceil", category = ACMD_SOUND )]
unsafe fn shinespark_ceil_snd(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_machstamp_landing"));
    }
}
#[acmd_script( agent = "samus", script = "effect_shinesparkairceil", category = ACMD_EFFECT )]
unsafe fn shinespark_ceil_eff(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_drill"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        COL_NORMAL(fighter);
        BURN_COLOR_NORMAL(fighter);
    }
}
//wall-ground
#[acmd_script( agent = "samus", script = "game_shinesparkwall", category = ACMD_GAME )]
unsafe fn shinespark_wall_game(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 70, 0, 80, 6.5, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "samus", script = "expression_shinesparkwall", category = ACMD_EXPRESSION )]
unsafe fn shinespark_wall_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
    }
}
#[acmd_script( agent = "samus", script = "sound_shinesparkwall", category = ACMD_SOUND )]
unsafe fn shinespark_wall_snd(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_machstamp_landing"));
    }
}
#[acmd_script( agent = "samus", script = "effect_shinesparkwall", category = ACMD_EFFECT )]
unsafe fn shinespark_wall_eff(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_drill"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        COL_NORMAL(fighter);
        BURN_COLOR_NORMAL(fighter);
    }
}
//wall-air
#[acmd_script( agent = "samus", script = "game_shinesparkairwall", category = ACMD_GAME )]
unsafe fn shinespark_air_wall_game(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 70, 0, 80, 6.5, 0.0, 0.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW) == false {
        frame(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            VarModule::on_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_GRAVITY);
            SET_SPEED_EX(fighter, -0.4, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        frame(fighter.lua_state_agent, 37.0);
        if is_excute(fighter) {
            VarModule::on_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_CONTROL);
        }
    }else {
        frame(fighter.lua_state_agent, 20.0);
        if is_excute(fighter) {
            VarModule::on_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_GRAVITY);
        }
        frame(fighter.lua_state_agent, 23.0);
        if is_excute(fighter) {
            VarModule::on_flag(fighter.battle_object, status::SAMUS_FLAG_SHINESPARK_ENABLE_CONTROL);
        }
    }
}
#[acmd_script( agent = "samus", script = "expression_shinesparkairwall", category = ACMD_EXPRESSION )]
unsafe fn shinespark_air_wall_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
    }
}
#[acmd_script( agent = "samus", script = "sound_shinesparkairwall", category = ACMD_SOUND )]
unsafe fn shinespark_air_wall_snd(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_machstamp_landing"));
    }
}
#[acmd_script( agent = "samus", script = "effect_shinesparkairwall", category = ACMD_EFFECT )]
unsafe fn shinespark_air_wall_eff(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_drill"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        COL_NORMAL(fighter);
        BURN_COLOR_NORMAL(fighter);
    }
}
//landing
#[acmd_script( agent = "samus", script = "game_shinesparklanding", category = ACMD_GAME )]
unsafe fn shinespark_landing_game(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 70, 0, 80, 6.5, 0.0, 6.0, -10.0, Some(0.0), Some(6.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "samus", script = "expression_shinesparklanding", category = ACMD_EXPRESSION )]
unsafe fn shinespark_landing_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
    }
}
#[acmd_script( agent = "samus", script = "sound_shinesparklanding", category = ACMD_SOUND )]
unsafe fn shinespark_landing_snd(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_machstamp_landing"));
    }
}
#[acmd_script( agent = "samus", script = "effect_shinesparklanding", category = ACMD_EFFECT )]
unsafe fn shinespark_landing_eff(fighter : &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_drill"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
        COL_NORMAL(fighter);
        BURN_COLOR_NORMAL(fighter);
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}


pub fn install() {
    smashline::install_status_scripts!(
        jump_squat_status_pre,
        jump_aerial_status_pre,

        shinespark_ready_status_exec,
        shinespark_ready_status_init,
        shinespark_ready_status_pre,
        shinespark_ready_status_main,
        shinespark_ready_status_end,

        shinespark_start_status_exec,
        shinespark_start_status_init,
        shinespark_start_status_pre,
        shinespark_start_status_main,
        shinespark_start_status_end,
        
        shinespark_end_status_exec,
        shinespark_end_status_init,
        shinespark_end_status_pre,
        shinespark_end_status_main,
        shinespark_end_status_end
    );
    smashline::install_acmd_scripts!(
        shinespark_ready_game,
        shinespark_ready_exp,
        shinespark_ready_snd,
        shinespark_ready_eff,

        shinespark_start_game,
        shinespark_start_exp,
        shinespark_start_snd,
        shinespark_start_eff,

        // shinespark_air_aim_game,
        // shinespark_air_aim_exp,
        // shinespark_air_aim_snd,
        // shinespark_air_aim_eff,

        shinespark_loop_hi_game,
        shinespark_loop_s_game,
        shinespark_loop_lw_game,
        shinespark_loop_exp,
        shinespark_loop_snd,
        shinespark_loop_hi_eff,
        shinespark_loop_s_eff,
        shinespark_loop_lw_eff,

        shinespark_end_game,
        shinespark_end_exp,
        shinespark_end_snd,
        shinespark_end_eff,

        shinespark_air_end_game,
        shinespark_air_end_exp,
        shinespark_air_end_snd,
        shinespark_air_end_eff,
        
        shinespark_ceil_game,
        shinespark_ceil_exp,
        shinespark_ceil_snd,
        shinespark_ceil_eff,
        
        shinespark_wall_game,
        shinespark_wall_exp,
        shinespark_wall_snd,
        shinespark_wall_eff,
        
        shinespark_air_wall_game,
        shinespark_air_wall_exp,
        shinespark_air_wall_snd,
        shinespark_air_wall_eff,
        
        shinespark_landing_game,
        shinespark_landing_exp,
        shinespark_landing_snd,
        shinespark_landing_eff
    );
}