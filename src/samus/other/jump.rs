use crate::imports::*;


////grounded forward spin jump
//status
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn jump_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
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
#[acmd_script( agent = "samus", script = "sound_jumpfrontspin", category = ACMD_SOUND )]
unsafe fn jump_f_spin_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GIMMICK_SPRING_JUMP_FROM_RING) == false {
            PLAY_SE(fighter, Hash40::new("se_samus_jump01"));
        }
    }
}
#[acmd_script( agent = "samus", script = "sound_jumpfrontspinmini", category = ACMD_SOUND )]
unsafe fn jump_f_spin_mini_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_jump03"));
    }
}
#[acmd_script( agent = "samus", scripts = ["effect_jumpfrontspin", "effect_jumpfrontspinmini"], category = ACMD_EFFECT )]
unsafe fn jump_f_spin_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90, -90, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samus_jump_jet"), false, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samus_jump"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.65, true);
    }
}

pub fn install() {
  smashline::install_status_scripts!(
    jump_status_main
  );
  smashline::install_acmd_scripts!(
    // jump_f_spin_game,
    // jump_f_spin_exp,
    jump_f_spin_snd,
    jump_f_spin_mini_snd,
    jump_f_spin_eff
  );
}