use crate::imports::*;

////grounded forward spin jump
//status
unsafe extern "C" fn jump_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_JUMP);
    let ret = original(fighter);
    if ControlModule::get_stick_x(fighter.module_accessor)*PostureModule::lr(fighter.module_accessor) > 0.4 
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SCREW) == false {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("jump_f_spin_mini"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("jump_f_spin"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    return ret
}
//motion
unsafe extern "C" fn jump_f_spin_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GIMMICK_SPRING_JUMP_FROM_RING) == false {
            macros::PLAY_SE(fighter, Hash40::new("se_samus_jump01"));
        }
    }
}
unsafe extern "C" fn jump_f_spin_mini_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_jump03"));
    }
}
unsafe extern "C" fn jump_f_spin_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samus_jump_jet"), false, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_jump"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.65, true);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_JUMP, jump_status_main);
    agent.sound_acmd("sound_jumpfrontspin", jump_f_spin_snd);
    agent.sound_acmd("sound_jumpfrontspinmini", jump_f_spin_mini_snd);
    agent.effect_acmd("effect_jumpfrontspin", jump_f_spin_eff);
    agent.effect_acmd("effect_jumpfrontspinmini", jump_f_spin_eff);
}