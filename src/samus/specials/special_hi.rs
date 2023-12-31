use crate::imports::*;
use crate::samus::vars::*;
use crate::samus::consts::*;

////up-special is now Graple-Beam
/// note: see opff for additional use of the "SAMUS_FLAG_SPECIAL_HI_HOP_DISABLED" flag

//status
unsafe extern "C" fn special_hi_status_init(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
unsafe extern "C" fn special_hi_status_exit(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
unsafe extern "C" fn special_hi_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn special_hi_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_hi_2nd"), 0.0, 1.0, false, 0.0);
    }else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_air_hi_2nd"), 0.0, 1.0, false, 0.0);
    }

    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_HI_HOP_DISABLED) == false {
            VarModule::on_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_HI_HOP_DISABLED);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                param::SAMUS_FLOAT_SPECIAL_HI_HOP
            );
        }
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    }
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, *FIGHTER_STATUS_AIR_LASSO_WORK_INT_ARTICLE_ID);
    WorkModule::set_int(fighter.module_accessor, *ARTICLE_ID_NONE, *FIGHTER_STATUS_AIR_LASSO_WORK_INT_ARTICLE2_ID);

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    //default angle is set to halfway between min and max 
    let min = param::SAMUS_FLOAT_SPECIAL_HI_GBEAM_ANGLE_MIN;
    let max = param::SAMUS_FLOAT_SPECIAL_HI_GBEAM_ANGLE_MAX;
    let weight = (((max-min)/2.0)+min)/90.0;
    MotionModule::set_weight(fighter.module_accessor, weight, false);
    VarModule::off_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_HI_LOCK_ANGLE);
    //if first time using durring current airtime, add upward momentum
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_HI_HOP_DISABLED) == false {
            VarModule::on_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_HI_HOP_DISABLED);
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                param::SAMUS_FLOAT_SPECIAL_HI_HOP
            );
        }
    }
    VarModule::off_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_HI_FIX_GBEAM_POS);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi_status_loop as *const () as _))
}
pub unsafe fn special_hi_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    //status ending stuff
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    //air-ground transition
    if fighter.global_table[0x16].get_i32() != fighter.global_table[0x17].get_i32() {
        let frame = MotionModule::frame(fighter.module_accessor);
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            macros::PLAY_LANDING_SE(fighter, Hash40::new("se_samus_landing01"));
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_hi"), -1.0, 1.0, 0.0);
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_hi_2nd"), frame, 1.0, false, 0.0);
        }else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0);
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new("special_air_hi_2nd"), frame, 1.0, false, 0.0);
        }
        let angle = VarModule::get_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_HI_ANGLE);
        MotionModule::set_weight(fighter.module_accessor, angle/90.0, false);
    }
    //attempt tether recovery
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND 
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK) 
    && ControlModule::get_stick_y(fighter.module_accessor) > -0.4 
    && GroundModule::is_status_cliff(fighter.module_accessor)
    && GroundModule::can_entry_cliff(fighter.module_accessor) == 1
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME) <= 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_AIR_LASSO_REACH.into(), true.into());
        return true.into()
    }
    //blend motions to match angle
    if VarModule::is_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_HI_LOCK_ANGLE) == false {
        let angle;
        let min = param::SAMUS_FLOAT_SPECIAL_HI_GBEAM_ANGLE_MIN;
        let max = param::SAMUS_FLOAT_SPECIAL_HI_GBEAM_ANGLE_MAX;
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        if stick_x == 0.0 && stick_y == 0.0 {
            //if no direction is held, angle is set to halfway between min and max 
            angle = ((max-min)/2.0)+min;
        }else {
            let lr = PostureModule::lr(fighter.module_accessor);
            let mut stick_deg = ControlModule::get_stick_angle(fighter.module_accessor).to_degrees();
            if lr < 0.0 {
                if stick_deg < 0.0 {
                    stick_deg = -180.0 -stick_deg;
                }else {
                    stick_deg = 180.0 -stick_deg;
                }
            }
            angle = stick_deg.clamp(min, max);
        }
        VarModule::set_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_HI_ANGLE, angle);
        MotionModule::set_weight(fighter.module_accessor, angle/90.0, false);
    }
    return false.into()
}
unsafe extern "C" fn special_hi_status_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    //change tether's starting position to match angle
    if VarModule::is_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_HI_FIX_GBEAM_POS) {
        VarModule::off_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_HI_FIX_GBEAM_POS);
        let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM);
        let object_id = Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(object_id);

        LinkModule::set_model_constraint_target_joint(article_boma, Hash40::new("haver"));

        let lr = PostureModule::lr(fighter.module_accessor);
        let angle = VarModule::get_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_HI_ANGLE);
        let offset = 3.0;

        let pos_x = angle.to_radians().cos()*offset*lr;
        let pos_y = angle.to_radians().sin()*offset;
        let end_joint = PhysicsModule::get_2nd_node_num(article_boma) as i32;
        let vec3 = &mut Vector3f{x:0.0, y:3.0, z:0.0};
        ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("haver"), vec3, false);
        PhysicsModule::set_2nd_pos(article_boma, end_joint, &Vector3f{x:vec3.x+pos_x, y:vec3.y+pos_y, z:0.0});
    }
    0.into()
}
unsafe extern "C" fn special_hi_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_STATUS_KIND_AIR_LASSO_REACH {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}
//motion
unsafe extern "C" fn special_hi_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, false, -1);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_SAMUS_CLIFF_HANG_DATA_AIR_LASSO as u32);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.0, 361, 30, 0, 90, 2.5, 1.2, 0.0, 0.0, Some(7.2), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_CLIFF_HANG_DATA_DEFAULT as u32);
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, *WEAPON_SAMUS_GBEAM_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VarModule::on_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_HI_LOCK_ANGLE);
        VarModule::on_flag(fighter.module_accessor, status::SAMUS_FLAG_SPECIAL_HI_FIX_GBEAM_POS);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        let angle = VarModule::get_float(fighter.module_accessor, status::SAMUS_FLOAT_SPECIAL_HI_ANGLE);
        macros::ATTACK(fighter, 0, 1, Hash40::new("throw"), 3.0, angle as u64, 0, 15, 100, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 1, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);

        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        //reset if tether collides with wall on first frame
        let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM);
        let object_id = Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(object_id);
        let pos = &mut Vector3f{x:0.0, y:0.0, z:0.0};
        ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("armr"), pos, false);
        if StatusModule::status_kind(article_boma) != *WEAPON_SAMUS_GBEAM_STATUS_KIND_SHOOT
        && GroundModule::get_distance_to_floor(fighter.module_accessor, pos, 0.0, false) < 6.0
        && angle > 70.0 {
            ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, *WEAPON_SAMUS_GBEAM_STATUS_KIND_SHOOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 2, Hash40::new("throw"), 7.0, 55, 60, 0, 85, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, *WEAPON_SAMUS_GBEAM_STATUS_KIND_REWIND, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}
unsafe extern "C" fn special_hi_exp(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, true, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GUN, Hash40::new("air_catch"), false, -1.0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 9, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 14, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
unsafe extern "C" fn special_hi_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samus_catch"));
    }
}
unsafe extern "C" fn special_hi_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) 
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    if is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_gbeam_flash"), Hash40::new("armr"), 7.2, 0, 0, 0, 0, 0, 1.3, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_gbeam_shot"), Hash40::new("armr"), 7, 0, -0.5, 0, 0, 0, 0.75, true);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samus_gbeam_vanish"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 0.7, true);
    }
}
//tether turnaround stuff
unsafe extern "C" fn air_lasso_hang_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG);
    let ret = original(fighter);
    let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM);
    let object_id = Article::get_battle_object_id(article) as u32;
    let article_boma = sv_battle_object::module_accessor(object_id);
    if PostureModule::lr(fighter.module_accessor) != PostureModule::lr(article_boma) {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_BODY_FLIP_X, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_INT_BODY_FLIP);
    }
    ret
}
unsafe extern "C" fn air_lasso_rewind_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_INT_BODY_FLIP) == *FIGHTER_STATUS_AIR_LASSO_BODY_FLIP_X 
    && PostureModule::lr(fighter.module_accessor) == -1.0 {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
    let original = smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND);
    original(fighter)
}
unsafe extern "C" fn air_lasso_rewind_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let article = ArticleModule::get_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_GBEAM);
    let object_id = Article::get_battle_object_id(article) as u32;
    let article_boma = sv_battle_object::module_accessor(object_id);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_HANG_WORK_INT_BODY_FLIP) == *FIGHTER_STATUS_AIR_LASSO_BODY_FLIP_X 
    && PostureModule::lr(fighter.module_accessor) == PostureModule::lr(article_boma) {
        PostureModule::reverse_lr(fighter.module_accessor);
    }
    let original = smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND);
    original(fighter)
}
//graple-beam
unsafe extern "C" fn gbeam_shoot_status_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    ////poorly simulates vanilla tether behavior
    let fighter_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let fighter_boma = sv_battle_object::module_accessor(fighter_id);
    let lr = PostureModule::lr(fighter_boma);
    let angle = VarModule::get_float(fighter_boma, status::SAMUS_FLOAT_SPECIAL_HI_ANGLE);
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_gbeam"), hash40("shoot_air_speed"));
    let chain_max = PhysicsModule::get_2nd_node_num(weapon.module_accessor) as i32;
    let x_speed_max = angle.to_radians().cos() * speed * lr;
    let y_speed_max = angle.to_radians().sin() * speed;
    let constraint_pos = &mut Vector3f{x:0.0, y:0.0, z:0.0};
    ModelModule::joint_global_position(fighter_boma, Hash40::new("haver"), constraint_pos, false);
    //itterate through each joint of tether
    for chain_curr in 0..chain_max {
        let joint_hash = WorkModule::get_param_int64(weapon.module_accessor, hash40("joint_id_gbeam"), chain_curr as u64);
        let joint_pos = &mut Vector3f{x:0.0, y:0.0, z:0.0};
        ModelModule::joint_global_position(weapon.module_accessor, Hash40::new_raw(joint_hash), joint_pos, false);
        let x_speed = constraint_pos.x -joint_pos.x;
        let y_speed = constraint_pos.y -joint_pos.y;
        PhysicsModule::set_2nd_speed(weapon.module_accessor, chain_curr, &Vector3f{x:x_speed, y:y_speed, z:0.0});
    }
    PhysicsModule::set_2nd_speed(weapon.module_accessor, chain_max, &Vector3f{x:x_speed_max, y:y_speed_max, z:0.0});
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    //status
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_status_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_status_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_status_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_status_exec);
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_status_exit);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_status_end);
    //motion-ground
    agent.game_acmd("game_specialhi", special_hi_game);
    agent.expression_acmd("expression_specialhi", special_hi_exp);
    agent.sound_acmd("sound_specialhi", special_hi_snd);
    agent.effect_acmd("effect_specialhi", special_hi_eff);
    //motion-air
    agent.game_acmd("game_specialairhi", special_hi_game);
    agent.expression_acmd("expression_specialairhi", special_hi_exp);
    agent.sound_acmd("sound_specialairhi", special_hi_snd);
    agent.effect_acmd("effect_specialairhi", special_hi_eff);
    //thethering statuses
    agent.status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO_HANG, air_lasso_hang_status_main);
    agent.status(Pre, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND, air_lasso_rewind_status_pre);
    agent.status(End, *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND, air_lasso_rewind_status_end);
    //graple-beam
    Agent::new("samus_gbeam")
    .status(Exec, *WEAPON_SAMUS_GBEAM_STATUS_KIND_SHOOT, gbeam_shoot_status_exec)
    .install();
}
