use crate::imports::*;
// use crate::samus::vars::*;
// use crate::samus::consts::*;

//forward and back roll changed to sense-move
//forward dodge
#[acmd_script( agent = "samus", script = "game_escapef", category = ACMD_GAME )]
unsafe fn escapef_game(fighter: &mut L2CAgentBase) {
    // frame(agent.lua_state_agent, 2.0);
    // if macros::is_excute(agent) {
    //     VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
    // }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
    }
    // frame(agent.lua_state_agent, 23.0);
    // if macros::is_excute(agent) {
    //     VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    // }
}
#[acmd_script( agent = "samus", script = "expression_escapef", category = ACMD_EXPRESSION )]
unsafe fn escapef_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        // ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        // ItemModule::set_attach_item_visibility(agent.module_accessor, false);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 10, true);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        // ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        // ItemModule::set_attach_item_visibility(agent.module_accessor, true);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}
#[acmd_script( agent = "samus", script = "sound_escapef", category = ACMD_SOUND )]
unsafe fn escapef_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_escape"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_landing01"));
    }
}
#[acmd_script( agent = "samus", script = "effect_escapef", category = ACMD_EFFECT )]
unsafe fn escapef_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
//backwards dodge
#[acmd_script( agent = "samus", script = "game_escapeb", category = ACMD_GAME )]
unsafe fn escapeb_game(_fighter: &mut L2CAgentBase) {
    // frame(agent.lua_state_agent, 2.0);
    // if macros::is_excute(agent) {
    //     VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
    // }
    // frame(agent.lua_state_agent, 23.0);
    // if macros::is_excute(agent) {
    //     VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    // }
}
#[acmd_script( agent = "samus", script = "expression_escapeb", category = ACMD_EXPRESSION )]
unsafe fn escapeb_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        // ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        // ItemModule::set_attach_item_visibility(agent.module_accessor, false);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 10, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        // ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        // ItemModule::set_attach_item_visibility(agent.module_accessor, true);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}
#[acmd_script( agent = "samus", script = "sound_escapeb", category = ACMD_SOUND )]
unsafe fn escapeb_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_escape"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_landing01"));
    }
}
#[acmd_script( agent = "samus", script = "effect_escapeb", category = ACMD_EFFECT )]
unsafe fn escapeb_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}


pub fn install() {
    // smashline::install_status_scripts!(
    // );
    smashline::install_acmd_scripts!(
        escapef_game,
        escapef_exp,
        escapef_snd,
        escapef_eff,
        
        escapeb_game,
        escapeb_exp,
        escapeb_snd,
        escapeb_eff
    );
}