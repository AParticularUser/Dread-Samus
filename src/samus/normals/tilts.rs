use crate::imports::*;
use crate::samus::vars::*;
use crate::samus::consts::*;

////replaced up-tilt with morphed-counter
// up-tilt scripts
#[acmd_script( agent = "samus", script = "game_attackhi3", category = ACMD_GAME )]
unsafe fn up_tilt_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    FT_MOTION_RATE(fighter, 1.3);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 8.0, 95, 100, 0, 46, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 8.0, 95, 100, 0, 46, 4.0, 6.4, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 5.0, 95, 100, 0, 46, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 5.0, 95, 100, 0, 46, 3.0, 6.4, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    FT_MOTION_RATE(fighter, 1.0);
}
#[acmd_script( agent = "samus", script = "expression_attackhi3", category = ACMD_EXPRESSION )]
unsafe fn up_tilt_exp(fighter: &mut L2CAgentBase) {
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
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
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
#[acmd_script( agent = "samus", script = "sound_attackhi3", category = ACMD_SOUND )]
unsafe fn up_tilt_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        // PLAY_SE(fighter, Hash40::new("se_common_swing_09"));
        PLAY_STATUS(fighter, Hash40::new("se_samus_swing_l"));
        PLAY_SE(fighter, Hash40::new("se_samus_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_step_right_m"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_landing01"));
    }
}
#[acmd_script( agent = "samus", script = "effect_attackhi3", category = ACMD_EFFECT )]
unsafe fn up_tilt_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 15, -2, 0, 60, 90, 1.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samus_atk_air_lw"), Hash40::new("top"), 0, 15, 0, 0, 0, 90, 1.1, true);
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), -2, 35, 0, -17, 15, 81, 1.5, true);
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 50, 0, -17, 15, 81, 1.5, true);
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_c"), Hash40::new("top"), -2, 65, 0, -17, 15, 81, 1.5, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -9, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}

////replaced side-tilt with melee-counter
// side-tilt scripts
#[acmd_script( agent = "samus", scripts = ["game_attacks3", "game_attacks3hi", "game_attacks3lw"], category = ACMD_GAME )]
unsafe fn side_tilt_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 55, 100, 0, 30, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.0, 55, 100, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        // AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}
#[acmd_script( agent = "samus", scripts = ["expression_attacks3", "expression_attacks3hi", "expression_attacks3lw"], category = ACMD_EXPRESSION )]
unsafe fn side_tilt_exp(fighter: &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}
#[acmd_script( agent = "samus", scripts = ["sound_attacks3", "sound_attacks3hi", "sound_attacks3lw"], category = ACMD_SOUND )]
unsafe fn side_tilt_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_swing_m"));
    }
}
#[acmd_script( agent = "samus", scripts = ["effect_attacks3", "effect_attacks3hi", "effect_attacks3lw"], category = ACMD_EFFECT )]
unsafe fn side_tilt_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 14, 4, 0, 25, 90, 1, true);
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 14, 34, 0, 25, 90, 1, true);
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_c"), Hash40::new("top"), 0, 14, 64, 0, 25, 90, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 14, 4, 0, 25, 90, 1, true);
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_muzzleflash"), Hash40::new("armr"), 6, 0, 0, 0, 90, 0, 0.5, true);
        // EFFECT_FOLLOW(fighter, Hash40::new("samus_atk_air_lw"), Hash40::new("top"), 0, 14, 124, 0, -45, 90, 0.9, true);
        // LAST_EFFECT_SET_RATE(fighter, 1);
    }
}

////replaced down-tilt with slide
//status
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn down_tilt_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw3"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    let lr = PostureModule::lr(fighter.module_accessor);
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
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
        VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_ATTACK_LW3_CHECK_CEIL);
        VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_STICK_TIMER, param::SAMUS_INT_SPEEDBOOSTER_STICK_FRAME);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(down_tilt_status_loop as *const () as _))
}
pub unsafe fn down_tilt_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), true.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
        return true.into()
    }else if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND 
    && GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_UP as u32)
    && VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_ATTACK_LW3_CHECK_CEIL)  {
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
            // MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw3_fall"), -1.0, 1.0, 0.0);

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
            if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
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
#[acmd_script( agent = "samus", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn down_tilt_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, 0.75);
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) 
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 80, 75, 0, 80, 2.5, 0.0, 2.5, 11.0, Some(0.0), Some(2.5), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 80, 75, 0, 80, 4.0, 0.0, 4.0, -2.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
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
        VarModule::on_flag(fighter.battle_object, status::SAMUS_FLAG_ATTACK_LW3_CHECK_CEIL);
    }
}
#[acmd_script( agent = "samus", script = "expression_attacklw3", category = ACMD_EXPRESSION )]
unsafe fn down_tilt_exp(fighter : &mut L2CAgentBase) {
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
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
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
#[acmd_script( agent = "samus", script = "sound_attacklw3", category = ACMD_SOUND )]
unsafe fn down_tilt_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_samus_swing_m"));
    }
}
#[acmd_script( agent = "samus", script = "effect_attacklw3", category = ACMD_EFFECT )]
unsafe fn down_tilt_eff(fighter : &mut L2CAgentBase) {
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 7, 135, 25, 0, 0.5, true);
        }
        if is_excute(fighter) 
        && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 3, 0, 8, 135, 25, 0, 0.5, true);
            // EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 0, 0, 0, 135, 25, 0, 0.5, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 4, 0, -8, 135, 25, 0, 0.5, true);
        }
        frame(fighter.lua_state_agent, 8.0);
        for _ in 0..4 {
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), -1, 0, 11, 135, 25, 0, 0.5, true);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 3, 0, 6, 135, 25, 0, 0.5, true);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 0, 0, 0, 135, 25, 0, 0.5, true);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_spark"), Hash40::new("top"), 4, 0, -8, 135, 25, 0, 0.5, true);
            }
            wait(fighter.lua_state_agent, 4.0);
        }
    }else {
            frame(fighter.lua_state_agent, 8.0);
            if is_excute(fighter) {
                LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.8, 0, 0, 4, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(fighter, 1.2);
            }
            frame(fighter.lua_state_agent, 10.0);
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.5, 0, 0, 5, 0, 0, 0, false);
            }
            frame(fighter.lua_state_agent, 12.0);
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                FOOT_EFFECT(fighter, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 6, 0, -3, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_RATE(fighter, 1.4);
            }
            frame(fighter.lua_state_agent, 13.0);
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, -3, 0, 0, 0, 0.45, 0, 0, 5, 0, 0, 0, false);
            }
            frame(fighter.lua_state_agent, 16.0);
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7, 0, -3, 0, 0, 0, 0.35, 0, 0, 5, 0, 0, 0, false);
            }
            frame(fighter.lua_state_agent, 19.0);
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, -3, 0, 0, 0, 0.35, 0, 0, 5, 0, 0, 0, false);
            }
            frame(fighter.lua_state_agent, 23.0);
            if is_excute(fighter) 
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7, 0, -3, 0, 0, 0, 0.35, 0, 0, 5, 0, 0, 0, false);
            }
    }
}

pub fn install() {
    smashline::install_status_scripts!(
        down_tilt_status_main
    );
    smashline::install_acmd_scripts!(
        up_tilt_game,
        up_tilt_exp,
        up_tilt_snd,
        up_tilt_eff,

        side_tilt_game,
        side_tilt_exp,
        side_tilt_snd,
        side_tilt_eff,

        down_tilt_game,
        down_tilt_exp,
        down_tilt_snd,
        down_tilt_eff
    );
}