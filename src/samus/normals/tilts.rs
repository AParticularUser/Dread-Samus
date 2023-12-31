use crate::imports::*;
use crate::samus::vars::*;
use crate::samus::consts::*;

////replaced up-tilt with morphed-counter
// motion
unsafe extern "C" fn attack_hi3_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(fighter, 1.3);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 8.0, 95, 95, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 8.0, 95, 95, 0, 50, 4.0, 6.4, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 5.0, 95, 90, 0, 46, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 5.0, 95, 90, 0, 46, 3.0, 6.4, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}
unsafe extern "C" fn attack_hi3_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 8);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}
unsafe extern "C" fn attack_hi3_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_samus_swing_l"));
        macros::PLAY_SE(fighter, Hash40::new("se_samus_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_step_right_m"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_landing01"));
    }
}
unsafe extern "C" fn attack_hi3_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 15, -2, 0, 60, 90, 1.4, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_atk_air_lw"), Hash40::new("top"), 0, 15, 0, 0, 0, 90, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -9, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}
////replaced side-tilt with melee-counter
// motion
unsafe extern "C" fn attack_s3_game(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 55, 100, 0, 30, 4.5, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 55, 100, 0, 30, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 55, 100, 0, 30, 6.0, 0.0, 11.0, 7.0, Some(0.0), Some(11.0),Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 3, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}
unsafe extern "C" fn attack_s3_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}
unsafe extern "C" fn attack_s3_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_swing_m"));
    }
}
unsafe extern "C" fn attack_s3_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 14, 4, 0, 25, 90, 1, true);
    }
}
////replaced down-tilt with slide
//status
unsafe extern "C" fn attack_lw3_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw3"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    let lr = PostureModule::lr(fighter.module_accessor);
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
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
        JostleModule::set_status(fighter.module_accessor, false);
        VarModule::off_flag(fighter.module_accessor, status::SAMUS_FLAG_ATTACK_LW3_CHECK_CEIL);
        VarModule::set_int(fighter.module_accessor, instance::SAMUS_INT_SPEEDBOOSTER_STICK_TIMER, param::SAMUS_INT_SPEEDBOOSTER_STICK_FRAME);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_lw3_status_loop as *const () as _))
}
pub unsafe fn attack_lw3_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), true.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
        return true.into()
    }else if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND 
    && GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32)
    && VarModule::is_flag(fighter.module_accessor, status::SAMUS_FLAG_ATTACK_LW3_CHECK_CEIL)  {
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW.into(), true.into()); //loop
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    if fighter.global_table[0x16].get_i32() != fighter.global_table[0x17].get_i32() {
        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("fall"), 0.0, 0.0, false, 0.0);
            MotionModule::set_weight(fighter.module_accessor, 1.0, false);
            AttackModule::clear_all(fighter.module_accessor);
            HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 0);

            let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) 
                - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) 
                - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.set_situation(SITUATION_KIND_AIR.into());
            if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
                if WorkModule::get_param_float(fighter.module_accessor, smash::hash40("common"), smash::hash40("air_speed_x_limit")) < param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED {
                    sv_kinetic_energy!(
                        set_limit_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                        param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
                        0.0
                    );
                }
                sv_kinetic_energy!(
                    set_stable_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
                    0.0
                );
                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    speed_x,
                    0.0
                );
            }
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), true.into());
            return true.into()
        }
    }else if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND 
    && fighter.global_table[0x17].get_i32() != *SITUATION_KIND_GROUND 
    && MotionModule::frame(fighter.module_accessor) > 8.0 {
        let weight = MotionModule::weight(fighter.module_accessor) -0.066;//<-blend-rate
        if weight > 0.0 {
            MotionModule::set_weight(fighter.module_accessor, weight, false);
        }else if weight <= 0.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
            return true.into()
        }
    }
    return false.into()
}
//motion
unsafe extern "C" fn attack_lw3_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.75);
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) 
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 80, 75, 0, 80, 2.5, 0.0, 2.5, 11.0, Some(0.0), Some(2.5), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 80, 75, 0, 80, 4.0, 0.0, 4.0, -2.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.module_accessor, status::SAMUS_FLAG_ATTACK_LW3_CHECK_CEIL);
    }
}
unsafe extern "C" fn attack_lw3_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) 
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 5);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) 
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) 
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) 
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) 
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}
unsafe extern "C" fn attack_lw3_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_samus_swing_m"));
    }
}
unsafe extern "C" fn attack_lw3_eff(fighter : &mut L2CAgentBase) {
    if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 7, 135, 25, 0, 0.5, true);
        }
        if is_excute(fighter) 
        && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 3, 0, 8, 135, 25, 0, 0.5, true);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 4, 0, -8, 135, 25, 0, 0.5, true);
        }
        frame(fighter.lua_state_agent, 8.0);
        for _ in 0..4 {
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 11, 135, 25, 0, 0.5, true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 3, 0, 6, 135, 25, 0, 0.5, true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 0, 0, 0, 135, 25, 0, 0.5, true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 4, 0, -8, 135, 25, 0, 0.5, true);
            }
            wait(fighter.lua_state_agent, 4.0);
        }
    }else {
            frame(fighter.lua_state_agent, 8.0);
            if is_excute(fighter) {
                macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.8, 0, 0, 4, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
            }
            frame(fighter.lua_state_agent, 10.0);
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.5, 0, 0, 5, 0, 0, 0, false);
            }
            frame(fighter.lua_state_agent, 12.0);
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                macros::FOOT_EFFECT(fighter, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 6, 0, -3, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
                macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
            }
            frame(fighter.lua_state_agent, 13.0);
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, -3, 0, 0, 0, 0.45, 0, 0, 5, 0, 0, 0, false);
            }
            frame(fighter.lua_state_agent, 16.0);
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7, 0, -3, 0, 0, 0, 0.35, 0, 0, 5, 0, 0, 0, false);
            }
            frame(fighter.lua_state_agent, 19.0);
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, -3, 0, 0, 0, 0.35, 0, 0, 5, 0, 0, 0, false);
            }
            frame(fighter.lua_state_agent, 23.0);
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7, 0, -3, 0, 0, 0, 0.35, 0, 0, 5, 0, 0, 0, false);
            }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    //up-tilt
    agent.game_acmd("game_attackhi3", attack_hi3_game);
    agent.expression_acmd("expression_attackhi3", attack_hi3_exp);
    agent.sound_acmd("sound_attackhi3", attack_hi3_snd);
    agent.effect_acmd("effect_attackhi3", attack_hi3_eff);
    //side-tilt
    agent.game_acmd("game_attacks3", attack_s3_game);
    agent.expression_acmd("expression_attacks3", attack_s3_exp);
    agent.sound_acmd("sound_attacks3", attack_s3_snd);
    agent.effect_acmd("effect_attacks3", attack_s3_eff);
    //down-tilt
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW3, attack_lw3_status_main);
    agent.game_acmd("game_attacklw3", attack_lw3_game);
    agent.expression_acmd("expression_attacklw3", attack_lw3_exp);
    agent.sound_acmd("sound_attacklw3", attack_lw3_snd);
    agent.effect_acmd("effect_attacklw3", attack_lw3_eff);
}