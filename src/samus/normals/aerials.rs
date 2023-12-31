use crate::imports::*;

////replaced up-air with arm cannon blast
//up-air scripts
#[acmd_script( agent = "samus", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn up_air_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {

        if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
            SET_SPEED_EX(fighter, 0, -0.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }else {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x:0.0, y:-0.6, z:0.0});
        }

        // ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 90, 65, 0, 80, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        // ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 90, 65, 0, 80, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.0, 90, 75, 0, 80, 6.0, 7.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 55.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
#[acmd_script( agent = "samus", script = "expression_attackairhi", category = ACMD_EXPRESSION )]
unsafe fn up_air_exp(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}
#[acmd_script( agent = "samus", script = "sound_attackairhi", category = ACMD_SOUND )]
unsafe fn up_air_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_attackhard_l01"));
    }
}
#[acmd_script( agent = "samus", script = "effect_attackairhi", category = ACMD_EFFECT )]
unsafe fn up_air_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    // frame(fighter.lua_state_agent, 9.0);
    // if is_excute(fighter) {
    //     // EFFECT_BRANCH_SITUATION(fighter, Hash40::new("sys_bomb_a"), Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 13.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    //     fighter.clear_lua_stack();
    //     lua_args!(fighter, Hash40::new("sys_bomb_a"), Hash40::new("sys_bomb_b"), Hash40::new("top"), -15, 0, -4, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    //     sv_animcmd::EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
    //     fighter.clear_lua_stack();
    // }
}
//up-air-landing scripts
#[acmd_script( agent = "samus", script = "game_landingairhi", category = ACMD_GAME )]
unsafe fn up_air_land_game(_fighter : &mut L2CAgentBase) {
}
#[acmd_script( agent = "samus", script = "expression_landingairhi", category = ACMD_EXPRESSION )]
unsafe fn up_air_land_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}
#[acmd_script( agent = "samus", script = "sound_landingairhi", category = ACMD_SOUND )]
unsafe fn up_air_land_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_samus_landing02"));
    }
}
#[acmd_script( agent = "samus", script = "effect_landingairhi", category = ACMD_EFFECT )]
unsafe fn up_air_land_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
////

////replaced forward-air with melee-counter
//forward-air scripts
#[acmd_script( agent = "samus", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn forward_air_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 361, 100, 0, 40, 4.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 9.0, 361, 100, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 10.0);
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
#[acmd_script( agent = "samus", script = "expression_attackairf", category = ACMD_EXPRESSION )]
unsafe fn forward_air_exp(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}
#[acmd_script( agent = "samus", script = "sound_attackairf", category = ACMD_SOUND )]
unsafe fn forward_air_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_swing_m"));
    }
}
#[acmd_script( agent = "samus", script = "effect_attackairf", category = ACMD_EFFECT )]
unsafe fn forward_air_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc"), Hash40::new("top"), 0, 14, 4, 0, 25, 90, 1, true);
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 0, 14, 34, 0, 25, 90, 1, true);
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_c"), Hash40::new("top"), 0, 14, 64, 0, 25, 90, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), -1, 11, 0, 0, 10, 125, 1, true);
        // EFFECT_FOLLOW(fighter, Hash40::new("sys_muzzleflash"), Hash40::new("armr"), 6, 0, 0, 0, 90, 0, 0.5, true);
        // EFFECT_FOLLOW(fighter, Hash40::new("samus_atk_air_lw"), Hash40::new("top"), 0, 14, 124, 0, -45, 90, 0.9, true);
        // LAST_EFFECT_SET_RATE(fighter, 1);
    }
}
//forward-air-landing scripts
#[acmd_script( agent = "samus", script = "game_landingairf", category = ACMD_GAME )]
unsafe fn forward_air_land_game(_fighter : &mut L2CAgentBase) {}
#[acmd_script( agent = "samus", script = "expression_landingairf", category = ACMD_EXPRESSION )]
unsafe fn forward_air_land_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}
#[acmd_script( agent = "samus", script = "sound_landingairf", category = ACMD_SOUND )]
unsafe fn forward_air_land_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_samus_landing02"));
    }
}
#[acmd_script( agent = "samus", script = "effect_landingairf", category = ACMD_EFFECT )]
unsafe fn forward_air_land_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
////

////replaced neutral-air with skrew-attack
//neutral-air scripts
#[acmd_script( agent = "samus", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn neutral_air_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, 4.0, Some(0.0), Some(2.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, -4.0, Some(0.0), Some(2.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 9.5, 4.0, Some(0.0), Some(9.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 9.5, -4.0, Some(0.0), Some(9.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear_all(fighter.module_accessor);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, 4.0, Some(0.0), Some(2.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, -4.0, Some(0.0), Some(2.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.0, 0.0, 9.5, 4.0, Some(0.0), Some(9.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.0, 0.0, 9.5, -4.0, Some(0.0), Some(9.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, 4.0, Some(0.0), Some(2.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, -4.0, Some(0.0), Some(2.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.0, 0.0, 9.5, 4.0, Some(0.0), Some(9.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.0, 0.0, 9.5, -4.0, Some(0.0), Some(9.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, 4.0, Some(0.0), Some(2.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.2, 0.0, 2.5, -4.0, Some(0.0), Some(2.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.0, 0.0, 9.5, 4.0, Some(0.0), Some(9.5), Some(4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
        ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 367, 100, 25, 0, 3.0, 0.0, 9.5, -4.0, Some(0.0), Some(9.5), Some(-4.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUS_SCREW, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 70, 160, 0, 40, 8.0, 0.0, 6.5, 0.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
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
#[acmd_script( agent = "samus", script = "expression_attackairn", category = ACMD_EXPRESSION )]
unsafe fn neutral_air_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_spinattack"), 6);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_spinattacks"), 0, true, 0);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
}
#[acmd_script( agent = "samus", script = "sound_attackairn", category = ACMD_SOUND )]
unsafe fn neutral_air_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_samus_special_h01"));
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
#[acmd_script( agent = "samus", script = "effect_attackairn", category = ACMD_EFFECT )]
unsafe fn neutral_air_eff(fighter : &mut L2CAgentBase) {
    // if is_excute(fighter) {
    //     EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    //     LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    // }
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
        EFFECT_OFF_KIND(fighter, Hash40::new("samus_screwattack"), false, true);
        // EFFECT_LIGHT_END(fighter, 3, *EFFECT_SUB_ATTRIBUTE_FOLLOW);
        fighter.clear_lua_stack();
        lua_args!(fighter, 3, *EFFECT_SUB_ATTRIBUTE_FOLLOW);
        sv_animcmd::EFFECT_LIGHT_END(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }
}
//neutral-air-landing scripts
#[acmd_script( agent = "samus", script = "game_landingairn", category = ACMD_GAME )]
unsafe fn neutral_air_land_game(_fighter : &mut L2CAgentBase) {}
#[acmd_script( agent = "samus", script = "expression_landingairn", category = ACMD_EXPRESSION )]
unsafe fn neutral_air_land_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}
#[acmd_script( agent = "samus", script = "sound_landingairn", category = ACMD_SOUND )]
unsafe fn neutral_air_land_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_samus_landing02"));
    }
}
#[acmd_script( agent = "samus", script = "effect_landingairn", category = ACMD_EFFECT )]
unsafe fn neutral_air_land_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
////


pub fn install() {
    smashline::install_acmd_scripts!(
        up_air_game,
        up_air_exp,
        up_air_snd,
        up_air_eff,
        up_air_land_game,
        up_air_land_exp,
        up_air_land_snd,
        up_air_land_eff,

        forward_air_game,
        forward_air_exp,
        forward_air_snd,
        forward_air_eff,
        forward_air_land_game,
        forward_air_land_exp,
        forward_air_land_snd,
        forward_air_land_eff,

        neutral_air_game,
        neutral_air_exp,
        neutral_air_snd,
        neutral_air_eff,
        neutral_air_land_game,
        neutral_air_land_exp,
        neutral_air_land_snd,
        neutral_air_land_eff
    );
}