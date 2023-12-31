use crate::imports::*;

//status
unsafe extern "C" fn throw_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("throw_lw") {
        macros::STOP_SE(fighter, Hash40::new("se_samus_special_n01"));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
    }
    let original = smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_THROW);
    original(fighter)
}
//grab
unsafe extern "C" fn catch_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 10.5, 4.0, Some(0.0), Some(10.5), Some(10.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 10.5, 2.35, Some(0.0), Some(10.5), Some(12.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}
unsafe extern "C" fn catch_exp(fighter: &mut L2CAgentBase) {
  if is_excute(fighter) {
      slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
  }
  frame(fighter.lua_state_agent, 8.0);
  if is_excute(fighter) {
      ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
  }
}
unsafe extern "C" fn catch_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
    }
    wait(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_swing_06"));
    }
}
unsafe extern "C" fn catch_eff(_fighter: &mut L2CAgentBase) {}
//dash-grab
unsafe extern "C" fn catch_dash_game(fighter: &mut L2CAgentBase) {
  frame(fighter.lua_state_agent, 9.0);
  if is_excute(fighter) {
      GrabModule::set_rebound(fighter.module_accessor, true);
  }
  wait(fighter.lua_state_agent, 1.0);
  if is_excute(fighter) {
    macros::CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(12.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    macros::CATCH(fighter, 1, Hash40::new("top"), 1.3, 0.0, 10.0, 2.7, Some(0.0), Some(10.0), Some(14.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
  }
  game_CaptureCutCommon(fighter);
  wait(fighter.lua_state_agent, 2.0);
  if is_excute(fighter) {
      grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
      WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
      GrabModule::set_rebound(fighter.module_accessor, false);
  }
}
unsafe extern "C" fn catch_dash_exp(fighter: &mut L2CAgentBase) {
  if is_excute(fighter) {
      slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
  }
  frame(fighter.lua_state_agent, 9.0);
  if is_excute(fighter) {
      ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
  }
}
unsafe extern "C" fn catch_dash_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
    }
    wait(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_swing_06"));
    }
}
unsafe extern "C" fn catch_dash_eff(_fighter: &mut L2CAgentBase) {}
//pivot-grab
unsafe extern "C" fn catch_turn_game(fighter: &mut L2CAgentBase) {
  frame(fighter.lua_state_agent, 10.0);
  if is_excute(fighter) {
      GrabModule::set_rebound(fighter.module_accessor, true);
  }
  wait(fighter.lua_state_agent, 1.0);
  if is_excute(fighter) {
    macros::CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 10.0, -4.0, Some(0.0), Some(10.0), Some(-13.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    macros::CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 10.0, -2.35, Some(0.0), Some(10.0), Some(-14.85), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
  }
  game_CaptureCutCommon(fighter);
  wait(fighter.lua_state_agent, 2.0);
  if is_excute(fighter) {
      grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
      WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
      GrabModule::set_rebound(fighter.module_accessor, false);
  }
}
unsafe extern "C" fn catch_turn_exp(fighter: &mut L2CAgentBase) {
  if is_excute(fighter) {
      slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
  }
  frame(fighter.lua_state_agent, 10.0);
  if is_excute(fighter) {
      ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
  }
}
unsafe extern "C" fn catch_turn_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
    }
    wait(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_swing_06"));
    }
}
unsafe extern "C" fn catch_turn_eff(_fighter: &mut L2CAgentBase) {}
//grab-pull
unsafe extern "C" fn catch_pull_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}
//grab-wait
unsafe extern "C" fn catch_wait_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}
//pummel
unsafe extern "C" fn catch_attack_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.9, 361, 100, 30, 0, 7.2, 0.0, 8.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn catch_attack_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
}
unsafe extern "C" fn catch_attack_snd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_special_n02"));
    }
}
unsafe extern "C" fn catch_attack_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_cshot_shot"), Hash40::new("armr"), 7, 1, 0, 180, 90, 15, 0.2, true);
    }
}
//grab-release
unsafe extern "C" fn catch_cut_exp(fighter: &mut L2CAgentBase) {
    // methodlua2cpp::L2CFighterAnimcmdExpressionCommon::expression_CatchCutRumble()();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}
//forward-throw
unsafe extern "C" fn throw_f_game(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 40, 60, 0, 90, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 3.0, 361, 50, 0, 45, 3.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 3.0, 361, 50, 0, 45, 3.0, 8.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(fighter, 0.3);
    frame(fighter.lua_state_agent, 37.0);
    macros::FT_MOTION_RATE(fighter, 1.1);

    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 25, 15);
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 3.0, 361, 50, 0, 45, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }

    frame(fighter.lua_state_agent, 41.0);
    macros::FT_MOTION_RATE(fighter, 2.5);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 46.0);
    macros::FT_MOTION_RATE(fighter, 1.3);
}
unsafe extern "C" fn throw_f_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, 0); 
    }
}
unsafe extern "C" fn throw_f_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_swing_m"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
    }
}
unsafe extern "C" fn throw_f_eff(fighter: &mut L2CAgentBase) {
frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -17, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
    }
}
//back-throw
unsafe extern "C" fn throw_b_game(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 40, 55, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        macros::CHECK_FINISH_CAMERA(fighter, 25, 15);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 10.0, y: 3.0, z: 0.0});
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}
unsafe extern "C" fn throw_b_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
}
unsafe extern "C" fn throw_b_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}
unsafe extern "C" fn throw_b_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), -3, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
//up-throw
unsafe extern "C" fn throw_hi_game(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 90, 60, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 80, 100, 0, 10, 4.0, 0.0, 30.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 1, 32);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn throw_hi_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        let link_no = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
        let capture_boma = sv_battle_object::module_accessor(link_no);
        MotionModule::set_frame(capture_boma, 1.0, true);
        MotionModule::set_rate(capture_boma, 0.49);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
}
unsafe extern "C" fn throw_hi_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_kick_hit_s"));
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_smash_h01"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
    }
}
unsafe extern "C" fn throw_hi_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        macros::EFFECT_FLW_POS(fighter, Hash40::new("samus_throw_hi"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 0.65, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
//down-throw
unsafe extern "C" fn throw_lw_game(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 45, 80, 0, 100, 1.5, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 1, 32);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 50, 0, 45, 4.0, 0.0, 4.0, 12.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn throw_lw_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, -1);
        ArticleModule::set_float(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, 0.0, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);

        let link_no = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
        let capture_boma = sv_battle_object::module_accessor(link_no);
        LinkModule::set_model_constraint_joint(capture_boma, Hash40::new("trans"));
        MotionModule::change_motion(capture_boma, Hash40::new("down_damage_d3"), 0.0, 1.0, false, 0.0, false, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    for _ in 0..35 {
        if is_excute(fighter) {
            let charge = ArticleModule::get_float(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
            ArticleModule::set_float(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, charge +0.015, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beams"), 0, false, 0);
        
        let link_no = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
        let capture_boma = sv_battle_object::module_accessor(link_no);
        MotionModule::change_motion(capture_boma, Hash40::new("down_damage_d"), 0.0, 0.0, false, 0.0, false, false);
    }
}
unsafe extern "C" fn throw_lw_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_special_n01"));
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_samus_special_n01"));
        macros::PLAY_SE(fighter, Hash40::new("se_samus_special_n03"));
    }
}
unsafe extern "C" fn throw_lw_eff(fighter: &mut L2CAgentBase) {
frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 15, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_cshot_shot"), Hash40::new("armr"), 7, -1, 0, 180, 90, 10, 0.2, true);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    ////status
    agent.status(End, *FIGHTER_STATUS_KIND_THROW, throw_status_end);
    ////grab motions
    //grab
    agent.game_acmd("game_catch", catch_game);
    agent.expression_acmd("expression_catch", catch_exp);
    agent.sound_acmd("sound_catch", catch_snd);
    agent.effect_acmd("effect_catch", catch_eff);
    //dash-grab
    agent.game_acmd("game_catchdash", catch_dash_game);
    agent.expression_acmd("expression_catchdash", catch_dash_exp);
    agent.sound_acmd("sound_catchdash", catch_dash_snd);
    agent.effect_acmd("effect_catchdash", catch_dash_eff);
    //pivot-grab
    agent.game_acmd("game_catchturn", catch_turn_game);
    agent.expression_acmd("expression_catchturn", catch_turn_exp);
    agent.sound_acmd("sound_catchturn", catch_turn_snd);
    agent.effect_acmd("effect_catchturn", catch_turn_eff);
    //grab-pull
    agent.expression_acmd("expression_catchpull", catch_pull_exp);
    //grab-wait
    agent.expression_acmd("expression_catchwait", catch_wait_exp);
    //pummel
    agent.game_acmd("game_catchattack", catch_attack_game);
    agent.expression_acmd("expression_catchattack", catch_attack_exp);
    agent.sound_acmd("sound_catchattack", catch_attack_snd);
    agent.effect_acmd("effect_catchattack", catch_attack_eff);
    //grab-release
    agent.expression_acmd("expression_catchcut", catch_cut_exp);
    ////throw motions
    //forward-throw
    agent.game_acmd("game_throwf", throw_f_game);
    agent.expression_acmd("expression_throwf", throw_f_exp);
    agent.sound_acmd("sound_throwf", throw_f_snd);
    agent.effect_acmd("effect_throwf", throw_f_eff);
    //back-throw
    agent.game_acmd("game_throwb", throw_b_game);
    agent.expression_acmd("expression_throwb", throw_b_exp);
    agent.sound_acmd("sound_throwb", throw_b_snd);
    agent.effect_acmd("effect_throwb", throw_b_eff);
    //up-throw
    agent.game_acmd("game_throwhi", throw_hi_game);
    agent.expression_acmd("expression_throwhi", throw_hi_exp);
    agent.sound_acmd("sound_throwhi", throw_hi_snd);
    agent.effect_acmd("effect_throwhi", throw_hi_eff);
    //down-throw
    agent.game_acmd("game_throwlw", throw_lw_game);
    agent.expression_acmd("expression_throwlw", throw_lw_exp);
    agent.sound_acmd("sound_throwlw", throw_lw_snd);
    agent.effect_acmd("effect_throwlw", throw_lw_eff);
}