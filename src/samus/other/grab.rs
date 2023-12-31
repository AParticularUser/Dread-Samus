use crate::imports::*;



//status
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn throw_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("throw_lw") {
        STOP_SE(fighter, Hash40::new("se_samus_special_n01"));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
    }
    original!(fighter)
}


//grab
#[acmd_script( agent = "samus", script = "game_catch", category = ACMD_GAME )]
unsafe fn catch_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
      CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(7.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
      CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 8.0, 2.35, Some(0.0), Some(8.0), Some(9.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}
#[acmd_script( agent = "samus", script = "expression_catch", category = ACMD_EXPRESSION )]
unsafe fn catch_exp(fighter: &mut L2CAgentBase) {
  if is_excute(fighter) {
      slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
  }
  frame(fighter.lua_state_agent, 8.0);
  if is_excute(fighter) {
      ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
  }
}
#[acmd_script( agent = "samus", script = "sound_catch", category = ACMD_SOUND )]
unsafe fn catch_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
    }
    wait(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_swing_06"));
    }
}
#[acmd_script( agent = "samus", script = "effect_catch", category = ACMD_EFFECT )]
unsafe fn catch_eff(_fighter: &mut L2CAgentBase) {}
//dash-grab
#[acmd_script( agent = "samus", script = "game_catchdash", category = ACMD_GAME )]
unsafe fn catch_dash_game(fighter: &mut L2CAgentBase) {
  frame(fighter.lua_state_agent, 9.0);
  if is_excute(fighter) {
      GrabModule::set_rebound(fighter.module_accessor, true);
  }
  wait(fighter.lua_state_agent, 1.0);
  if is_excute(fighter) {
      CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(9.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
      CATCH(fighter, 1, Hash40::new("top"), 1.3, 0.0, 7.0, 2.7, Some(0.0), Some(7.0), Some(10.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
  }
  game_CaptureCutCommon(fighter);
  wait(fighter.lua_state_agent, 2.0);
  if is_excute(fighter) {
      grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
      WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
      GrabModule::set_rebound(fighter.module_accessor, false);
  }
}
#[acmd_script( agent = "samus", script = "expression_catchdash", category = ACMD_EXPRESSION )]
unsafe fn catch_dash_exp(fighter: &mut L2CAgentBase) {
  if is_excute(fighter) {
      slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
  }
  frame(fighter.lua_state_agent, 9.0);
  if is_excute(fighter) {
      ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
  }
}
#[acmd_script( agent = "samus", script = "sound_catchdash", category = ACMD_SOUND )]
unsafe fn catch_dash_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
    }
    wait(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_swing_06"));
    }
}
#[acmd_script( agent = "samus", script = "effect_catchdash", category = ACMD_EFFECT )]
unsafe fn catch_dash_eff(_fighter: &mut L2CAgentBase) {}
//pivot-grab
#[acmd_script( agent = "samus", script = "game_catchturn", category = ACMD_GAME )]
unsafe fn catch_turn_game(fighter: &mut L2CAgentBase) {
  frame(fighter.lua_state_agent, 10.0);
  if is_excute(fighter) {
      GrabModule::set_rebound(fighter.module_accessor, true);
  }
  wait(fighter.lua_state_agent, 1.0);
  if is_excute(fighter) {
    CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 7.0, -4.0, Some(0.0), Some(7.0), Some(-13.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 7.0, -2.35, Some(0.0), Some(7.0), Some(-14.85), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
  }
  game_CaptureCutCommon(fighter);
  wait(fighter.lua_state_agent, 2.0);
  if is_excute(fighter) {
      grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
      WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
      GrabModule::set_rebound(fighter.module_accessor, false);
  }
}
#[acmd_script( agent = "samus", script = "expression_catchturn", category = ACMD_EXPRESSION )]
unsafe fn catch_turn_exp(fighter: &mut L2CAgentBase) {
  if is_excute(fighter) {
      slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
  }
  frame(fighter.lua_state_agent, 10.0);
  if is_excute(fighter) {
      ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
  }
}
#[acmd_script( agent = "samus", script = "sound_catchturn", category = ACMD_SOUND )]
unsafe fn catch_turn_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
    }
    wait(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_swing_06"));
    }
}
#[acmd_script( agent = "samus", script = "effect_catchturn", category = ACMD_EFFECT )]
unsafe fn catch_turn_eff(_fighter: &mut L2CAgentBase) {}
//grab-pull
#[acmd_script( agent = "samus", script = "expression_catchpull", category = ACMD_EXPRESSION )]
unsafe fn catch_pull_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}
//grab-wait
#[acmd_script( agent = "samus", script = "expression_catchwait", category = ACMD_EXPRESSION )]
unsafe fn catch_wait_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}
//pummel
#[acmd_script( agent = "samus", script = "game_catchattack", category = ACMD_GAME )]
unsafe fn catch_attack_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 0.9, 361, 100, 30, 0, 7.2, 0.0, 8.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "samus", script = "expression_catchattack", category = ACMD_EXPRESSION )]
unsafe fn catch_attack_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
}
#[acmd_script( agent = "samus", script = "sound_catchattack", category = ACMD_SOUND )]
unsafe fn catch_attack_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_special_n02"));
    }
}
#[acmd_script( agent = "samus", script = "effect_catchattack", category = ACMD_EFFECT )]
unsafe fn catch_attack_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samus_cshot_shot"), Hash40::new("armr"), 7, 1, 0, 180, 90, 15, 0.2, true);
    }
}
//grab-release
#[acmd_script( agent = "samus", script = "expression_catchcut", category = ACMD_EXPRESSION )]
unsafe fn catch_cut_exp(fighter: &mut L2CAgentBase) {
    // methodlua2cpp::L2CFighterAnimcmdExpressionCommon::expression_CatchCutRumble()();
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}


//forward-throw
#[acmd_script( agent = "samus", script = "game_throwf", category = ACMD_GAME )]
unsafe fn throw_f_game(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 40, 80, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneel"), 3.0, 361, 50, 0, 45, 3.0, 3.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneel"), 3.0, 361, 50, 0, 45, 3.0, 8.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    FT_MOTION_RATE(fighter, 0.3);
    frame(fighter.lua_state_agent, 37.0);
    FT_MOTION_RATE(fighter, 1.1);

    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 25, 15);
        ATTACK(fighter, 0, 0, Hash40::new("footr"), 3.0, 361, 50, 0, 45, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }

    frame(fighter.lua_state_agent, 41.0);
    FT_MOTION_RATE(fighter, 2.5);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 46.0);
    FT_MOTION_RATE(fighter, 1.3);
}
#[acmd_script( agent = "samus", script = "expression_throwf", category = ACMD_EXPRESSION )]
unsafe fn throw_f_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        // QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, 0);
        
    }
}
#[acmd_script( agent = "samus", script = "sound_throwf", category = ACMD_SOUND )]
unsafe fn throw_f_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_swing_m"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        // PLAY_SE(fighter, Hash40::new("se_common_kick_hit_m"));
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
    }
}
#[acmd_script( agent = "samus", script = "effect_throwf", category = ACMD_EFFECT )]
unsafe fn throw_f_eff(fighter: &mut L2CAgentBase) {
frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -5.5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -17, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
    }
}
//back-throw
#[acmd_script( agent = "samus", script = "game_throwb", category = ACMD_GAME )]
unsafe fn throw_b_game(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 40, 55, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        CHECK_FINISH_CAMERA(fighter, 25, 15);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 10.0, y: 3.0, z: 0.0});
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}
#[acmd_script( agent = "samus", script = "expression_throwb", category = ACMD_EXPRESSION )]
unsafe fn throw_b_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
}
#[acmd_script( agent = "samus", script = "sound_throwb", category = ACMD_SOUND )]
unsafe fn throw_b_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}
#[acmd_script( agent = "samus", script = "effect_throwb", category = ACMD_EFFECT )]
unsafe fn throw_b_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), -3, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
//up-throw
#[acmd_script( agent = "samus", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn throw_hi_game(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 90, 80, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 80, 100, 0, 10, 4.0, 0.0, 30.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        CHECK_FINISH_CAMERA(fighter, 1, 32);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "samus", script = "expression_throwhi", category = ACMD_EXPRESSION )]
unsafe fn throw_hi_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        // let link_no = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
        // let capture_boma = sv_battle_object::module_accessor(link_no);
        // MotionModule::set_rate(capture_boma, 1.0);
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
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
}
#[acmd_script( agent = "samus", script = "sound_throwhi", category = ACMD_SOUND )]
unsafe fn throw_hi_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_kick_hit_s"));
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_smash_h01"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
    }
}
#[acmd_script( agent = "samus", script = "effect_throwhi", category = ACMD_EFFECT )]
unsafe fn throw_hi_eff(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("samus_throw_hi"), Hash40::new("armr"), 8, 0, 0, 0, 0, 0, 0.65, true);
        LAST_EFFECT_SET_RATE(fighter, 1);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
//down-throw
#[acmd_script( agent = "samus", script = "game_throwlw", category = ACMD_GAME )]
unsafe fn throw_lw_game(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 45, 80, 0, 100, 1.5, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 1, 32);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 50, 0, 45, 4.0, 0.0, 4.0, 12.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "samus", script = "expression_throwlw", category = ACMD_EXPRESSION )]
unsafe fn throw_lw_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        // slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        // let link_no = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
        // let capture_boma = sv_battle_object::module_accessor(link_no);
        // MotionModule::change_motion(capture_boma, Hash40::new("capture_wait_hi"), 0.0, 1.0, false, 0.0, false, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, -1);
        ArticleModule::set_float(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, 0.0, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);

        let link_no = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
        let capture_boma = sv_battle_object::module_accessor(link_no);
        LinkModule::set_model_constraint_joint(capture_boma, Hash40::new("trans"));
        // LinkModule::set_model_constraint_target_joint(capture_boma, Hash40::new("trans"));
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
    // frame(fighter.lua_state_agent, 61.0);
    // if is_excute(fighter) {
    //     QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    // }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST));
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beams"), 0, false, 0);
        
        let link_no = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
        let capture_boma = sv_battle_object::module_accessor(link_no);
        MotionModule::change_motion(capture_boma, Hash40::new("down_damage_d"), 0.0, 0.0, false, 0.0, false, false);
    }
    // frame(fighter.lua_state_agent, 50.0);
    // if is_excute(fighter) {
    //     slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    // }
    // frame(fighter.lua_state_agent, 120.0);
    // if is_excute(fighter) {
    //     slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
    // }
}
#[acmd_script( agent = "samus", script = "sound_throwlw", category = ACMD_SOUND )]
unsafe fn throw_lw_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_special_n01"));
    }
    // frame(fighter.lua_state_agent, 28.0);
    // if is_excute(fighter) {
    //     PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_01"));
    //     PLAY_SE(fighter, Hash40::new("se_common_kick_hit_m"));
    // }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_samus_special_n01"));
        PLAY_SE(fighter, Hash40::new("se_samus_special_n03"));
    }
}
#[acmd_script( agent = "samus", script = "effect_throwlw", category = ACMD_EFFECT )]
unsafe fn throw_lw_eff(fighter: &mut L2CAgentBase) {
frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 15, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samus_cshot_shot"), Hash40::new("armr"), 7, -1, 0, 180, 90, 10, 0.2, true);
    }
}



pub fn install() {
    smashline::install_status_scripts!(
        throw_status_end
    );
    smashline::install_acmd_scripts!(
        catch_game,
        catch_exp,
        catch_snd,
        catch_eff,

        catch_dash_game,
        catch_dash_exp,
        catch_dash_snd,
        catch_dash_eff,

        catch_turn_game,
        catch_turn_exp,
        catch_turn_snd,
        catch_turn_eff,

        // catch_pull_game,
        catch_pull_exp,
        // catch_pull_snd,
        // catch_pull_eff,

        // catch_wait_game,
        catch_wait_exp,
        // catch_wait_snd,
        // catch_wait_eff,

        catch_attack_game,
        catch_attack_exp,
        catch_attack_snd,
        catch_attack_eff,

        // catch_cut_game,
        catch_cut_exp,
        // catch_cut_snd,
        // catch_cut_eff,

        throw_f_game,
        throw_f_exp,
        throw_f_snd,
        throw_f_eff,

        throw_b_game,
        throw_b_exp,
        throw_b_snd,
        throw_b_eff,

        throw_hi_game,
        throw_hi_exp,
        throw_hi_snd,
        throw_hi_eff,

        throw_lw_game,
        throw_lw_exp,
        throw_lw_snd,
        throw_lw_eff
    );
}