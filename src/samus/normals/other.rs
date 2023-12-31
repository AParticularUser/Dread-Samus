use crate::imports::*;
// use crate::samus::vars::*;
// use crate::samus::consts::*;


////fixed jab combo
//jab-1
#[acmd_script( agent = "samus", script = "game_attack11", category = ACMD_GAME )]
unsafe fn attack_11_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 10, 0, 25, 2.0, 0.0, 10.0, 6.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 10, 0, 25, 2.3, 0.0, 10.0, 8.8, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 361, 10, 0, 25, 2.5, 0.0, 10.0, 12.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

////replaced dash-attack with dash-counter
// dash-attack scripts
#[acmd_script( agent = "samus", script = "game_attackdash", category = ACMD_GAME )]
unsafe fn attack_dash_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("armr"), 10.0, 55, 90, 0, 80, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 6.0, 55, 90, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_INVINCIBLE);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}
#[acmd_script( agent = "samus", script = "expression_attackdash", category = ACMD_EXPRESSION )]
unsafe fn attack_dash_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}
#[acmd_script( agent = "samus", script = "sound_attackdash", category = ACMD_SOUND )]
unsafe fn attack_dash_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_09"));
        PLAY_STATUS(fighter, Hash40::new("se_samus_swing_l"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_samus_landing01"));
    }
}
#[acmd_script( agent = "samus", script = "effect_attackdash", category = ACMD_EFFECT )]
unsafe fn attack_dash_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 18, 2, -17, -15, 81, 1.4, true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 17, 0, -17, 15, 81, 1.5, true);
        LAST_EFFECT_SET_RATE(fighter, 0.35);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        attack_11_game,

        attack_dash_game,
        attack_dash_exp,
        attack_dash_snd,
        attack_dash_eff
    );
}