use crate::imports::*;

//forward and back roll changed to sense-move
//forward dodge
unsafe extern "C" fn escape_f_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        macros::REVERSE_LR(fighter);
    }
}
unsafe extern "C" fn escape_f_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 10, true);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}
unsafe extern "C" fn escape_f_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_escape"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_landing01"));
    }
}
unsafe extern "C" fn escape_f_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
//backwards dodge
unsafe extern "C" fn escape_b_game(_fighter: &mut L2CAgentBase) {}
unsafe extern "C" fn escape_b_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 10, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}
unsafe extern "C" fn escape_b_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_escape"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_landing01"));
    }
}
unsafe extern "C" fn escape_b_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    //forward-roll
    agent.game_acmd("game_escapef", escape_f_game);
    agent.expression_acmd("expression_escapef", escape_f_exp);
    agent.sound_acmd("sound_escapef", escape_f_snd);
    agent.effect_acmd("effect_escapef", escape_f_eff);
    //back-roll
    agent.game_acmd("game_escapeb", escape_b_game);
    agent.expression_acmd("expression_escapeb", escape_b_exp);
    agent.sound_acmd("sound_escapeb", escape_b_snd);
    agent.effect_acmd("effect_escapeb", escape_b_eff);
}