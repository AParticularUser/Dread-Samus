use crate::imports::*;

unsafe extern "C" fn entry_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        ArticleModule::set_frame(agent.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION, 0.0);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 90.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.expression_acmd("expression_entryl", entry_exp);
    agent.expression_acmd("expression_entryr", entry_exp);
}