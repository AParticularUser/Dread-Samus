use crate::imports::*;

////replaced up-air with arm cannon blast
//up-air scripts
unsafe extern "C" fn attack_air_hi_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {

        if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
            macros::SET_SPEED_EX(fighter, 0, -0.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }else {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x:0.0, y:-0.6, z:0.0});
        }
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.0, 90, 75, 0, 80, 6.0, 7.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 55.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
unsafe extern "C" fn attack_air_hi_exp(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}
unsafe extern "C" fn attack_air_hi_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_attackhard_l01"));
    }
}
unsafe extern "C" fn attack_air_hi_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
}
//up-air-landing scripts
unsafe extern "C" fn landing_air_hi_game(_fighter : &mut L2CAgentBase) {}
unsafe extern "C" fn landing_air_hi_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}
unsafe extern "C" fn landing_air_hi_snd(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_samus_landing02"));
    }
}
unsafe extern "C" fn landing_air_hi_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
////replaced forward-air with melee-counter
//forward-air scripts
unsafe extern "C" fn attack_air_f_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 361, 86, 0, 50, 5.0, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 361, 86, 0, 50, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 361, 86, 0, 50, 6.0, 0.0, 9.0, 6.0, Some(0.0), Some(9.0),Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
unsafe extern "C" fn attack_air_f_exp(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}
unsafe extern "C" fn attack_air_f_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_swing_m"));
    }
}
unsafe extern "C" fn attack_air_f_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -1, 11, 0, 0, 10, 125, 1, true);
    }
}
//forward-air-landing scripts
unsafe extern "C" fn landing_air_f_game(_fighter : &mut L2CAgentBase) {}
unsafe extern "C" fn landing_air_f_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}
unsafe extern "C" fn landing_air_f_snd(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_samus_landing02"));
    }
}
unsafe extern "C" fn landing_air_f_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
////replaced neutral-air with skrew-attack
//neutral-air scripts
unsafe extern "C" fn attack_air_n_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, 4.0, Some(0.0), Some(2.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, -4.0, Some(0.0), Some(2.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 9.5, 4.0, Some(0.0), Some(9.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 9.5, -4.0, Some(0.0), Some(9.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 5.0, false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, 4.0, Some(0.0), Some(2.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, -4.0, Some(0.0), Some(2.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.0, 0.0, 9.5, 4.0, Some(0.0), Some(9.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.0, 0.0, 9.5, -4.0, Some(0.0), Some(9.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 5.0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, 4.0, Some(0.0), Some(2.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, -4.0, Some(0.0), Some(2.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.0, 0.0, 9.5, 4.0, Some(0.0), Some(9.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.0, 0.0, 9.5, -4.0, Some(0.0), Some(9.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 5.0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, 4.0, Some(0.0), Some(2.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, -4.0, Some(0.0), Some(2.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.0, 0.0, 9.5, 4.0, Some(0.0), Some(9.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.0, 0.0, 9.5, -4.0, Some(0.0), Some(9.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 5.0, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 70, 160, 0, 40, 8.0, 0.0, 6.5, 0.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
unsafe extern "C" fn attack_air_n_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_spinattack"), 6);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_spinattacks"), 0, true, 0);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
}
unsafe extern "C" fn attack_air_n_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_samus_special_h01"));
    }
    wait(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        // sound!(fighter, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        sv_module_access::sound(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }
}
unsafe extern "C" fn attack_air_n_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        // EFFECT_FOLLOW_LIGHT(fighter, Hash40::new("samus_screwattack"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.6, true, 0.471, 0.471, 0.471, 16, 1, 1, 0);
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("samus_screwattack"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.6, true, 0.471, 0.471, 0.471, 16, 1, 1, 0);
        sv_animcmd::EFFECT_FOLLOW_LIGHT(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samus_screwattack"), false, true);
        // EFFECT_LIGHT_END(fighter, 3, *EFFECT_SUB_ATTRIBUTE_FOLLOW);
        fighter.clear_lua_stack();
        lua_args!(fighter, 3, *EFFECT_SUB_ATTRIBUTE_FOLLOW);
        sv_animcmd::EFFECT_LIGHT_END(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }
}
//neutral-air-landing scripts
unsafe extern "C" fn landing_air_n_game(_fighter : &mut L2CAgentBase) {}
unsafe extern "C" fn landing_air_n_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}
unsafe extern "C" fn landing_air_n_snd(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_samus_landing02"));
    }
}
unsafe extern "C" fn landing_air_n_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    //up-air
    agent.game_acmd("game_attackairhi", attack_air_hi_game);
    agent.expression_acmd("expression_attackairhi", attack_air_hi_exp);
    agent.sound_acmd("sound_attackairhi", attack_air_hi_snd);
    agent.effect_acmd("effect_attackairhi", attack_air_hi_eff);
    //up-air-landing
    agent.game_acmd("game_landingairhi", landing_air_hi_game);
    agent.expression_acmd("expression_landingairhi", landing_air_hi_exp);
    agent.sound_acmd("sound_landingairhi", landing_air_hi_snd);
    agent.effect_acmd("effect_landingairhi", landing_air_hi_eff);
    //forward-air
    agent.game_acmd("game_attackairf", attack_air_f_game);
    agent.expression_acmd("expression_attackairf", attack_air_f_exp);
    agent.sound_acmd("sound_attackairf", attack_air_f_snd);
    agent.effect_acmd("effect_attackairf", attack_air_f_eff);
    //forward-air-landing
    agent.game_acmd("game_landingairf", landing_air_f_game);
    agent.expression_acmd("expression_landingairf", landing_air_f_exp);
    agent.sound_acmd("sound_landingairf", landing_air_f_snd);
    agent.effect_acmd("effect_landingairf", landing_air_f_eff);
    //neutral-air
    agent.game_acmd("game_attackairn", attack_air_n_game);
    agent.expression_acmd("expression_attackairn", attack_air_n_exp);
    agent.sound_acmd("sound_attackairn", attack_air_n_snd);
    agent.effect_acmd("effect_attackairn", attack_air_n_eff);
    //neutral-air-landing
    agent.game_acmd("game_landingairn", landing_air_n_game);
    agent.expression_acmd("expression_landingairn", landing_air_n_exp);
    agent.sound_acmd("sound_landingairn", landing_air_n_snd);
    agent.effect_acmd("effect_landingairn", landing_air_n_eff);
}