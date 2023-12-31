use crate::imports::*;

//side smash
#[acmd_script( agent = "samus", scripts = ["sound_attacks4", "sound_attacks4hi", "sound_attacks4lw"], category = ACMD_SOUND )]
unsafe fn side_smash_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_smash_h02"));
    }
}

//down smash
#[acmd_script( agent = "samus", script = "game_attacklw4", category = ACMD_GAME )]
unsafe fn down_smash_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.0, 75, 65, 0, 80, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 75, 65, 0, 80, 5.2, 0.0, 1.6, 14.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 14.0, 75, 65, 0, 80, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 75, 65, 0, 80, 5.2, 0.0, 1.6, -14.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "samus", script = "expression_attacklw4", category = ACMD_EXPRESSION )]
unsafe fn down_smash_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    // frame(fighter.lua_state_agent, 9.0);
    // if is_excute(fighter) {
    //     QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    // }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    // frame(fighter.lua_state_agent, 27.0);
    // if is_excute(fighter) {
    //     QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    // }
}
#[acmd_script( agent = "samus", script = "sound_attacklw4", category = ACMD_SOUND )]
unsafe fn down_smash_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_smash_h02"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_smash_h02"));
    }
}
#[acmd_script( agent = "samus", script = "effect_attacklw4", category = ACMD_EFFECT )]
unsafe fn down_smash_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        // EFFECT_BRANCH_SITUATION(fighter, Hash40::new("sys_bomb_a"), Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 13.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("sys_bomb_a"), Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 13.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        sv_animcmd::EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("samus_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        // EFFECT_BRANCH_SITUATION(fighter, Hash40::new("sys_bomb_a"), Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 13.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("sys_bomb_a"), Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, -13.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        sv_animcmd::EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }
}
//down smash charge
#[acmd_script( agent = "samus", script = "expression_attacklw4charge", category = ACMD_EXPRESSION )]
unsafe fn down_smash_charge_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        // VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
        // ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        // ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true);
        // ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("s4_hold"), false, -1.0);
        physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, 0.9, 0.8, -1, 0.9, 0.8, -1, Hash40::new("invalid"));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
#[acmd_script( agent = "samus", script = "effect_attacklw4charge", category = ACMD_EFFECT )]
unsafe fn down_smash_charge_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    for _ in 0..36 {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("armr"), 4.289, -0.272, -0.135, 0, 0, 0, 1, 4, 4, 4, 0, 0, 0, true);
        }
    }
}

pub fn install() {
  smashline::install_acmd_scripts!(
    // side_smash_game,
    // side_smash_exp,
    side_smash_snd,
    // side_smash_eff,

    down_smash_game,
    down_smash_exp,
    down_smash_snd,
    down_smash_eff,

    // down_smash_charge_game,
    down_smash_charge_exp,
    // down_smash_charge_snd,
    down_smash_charge_eff
  );
}