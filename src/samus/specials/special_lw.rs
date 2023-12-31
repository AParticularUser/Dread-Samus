use crate::imports::*;
use crate::samus::vars::*;
use crate::samus::consts::*;
use crate::samus::other::speedbooster::*;



////down special now can be toggled
//start
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn down_special_start_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,/*FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,*/
        0,/*FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,*/
        0,/*FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,*/
        *FS_SUCCEEDS_KEEP_VISIBILITY
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_SWIM as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn down_special_start_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
        let lr = PostureModule::lr(fighter.module_accessor);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED *lr,
            0.0
        );
    }
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_ENABLE);
    VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_HOP);
    fighter.sub_shift_status_main(L2CValue::Ptr(down_special_start_status_loop as *const () as _))
}
pub unsafe fn down_special_start_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW.into(), true.into()); //loop
        return true.into()
    }else {
        //bomb-jump
        if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_HOP) {
            VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_HOP);
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0);
                GroundModule::set_attach_ground(fighter.module_accessor, false);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                sv_kinetic_energy!(
                    set_brake,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    param::SAMUS_FLOAT_SPECIAL_LW_AIR_BRAKE,
                    0.0
                );
                sv_kinetic_energy!(
                    set_accel_x_mul,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    param::SAMUS_FLOAT_SPECIAL_LW_AIR_ACCEL_X_MUL
                );
            }
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                param::SAMUS_FLOAT_SPECIAL_LW_JUMP_HIGHT,
                0.0
            );
        }
        // air/ground transition
        else if fighter.global_table[0x16].get_i32() != fighter.global_table[0x17].get_i32() {
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0);
                // fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                sv_kinetic_energy!(
                    set_brake,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    param::SAMUS_FLOAT_SPECIAL_LW_GROUND_BRAKE,
                    0.0
                );
                sv_kinetic_energy!(
                    set_accel_x_mul,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    param::SAMUS_FLOAT_SPECIAL_LW_GROUND_ACCEL_X_MUL
                );
            }else {
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0);
                // fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                sv_kinetic_energy!(
                    set_brake,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    param::SAMUS_FLOAT_SPECIAL_LW_AIR_BRAKE,
                    0.0
                );
                sv_kinetic_energy!(
                    set_accel_x_mul,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    param::SAMUS_FLOAT_SPECIAL_LW_AIR_ACCEL_X_MUL
                );
            }
        }
        return false.into()
    }
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn down_special_start_status_end(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
//loop
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn down_special_loop_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,/*FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,*/
        0,/*FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,*/
        0,/*FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,*/
        *FS_SUCCEEDS_KEEP_VISIBILITY
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_SWIM as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn down_special_loop_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
    }
    let speed_x;
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
        speed_x = param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED *PostureModule::lr(fighter.module_accessor); 
        //speed = VarModule::get_float(fighter.battle_object, instance::SAMUS_FLOAT_SPEEDBOOSTER_CURR_SPEED_X);
    }else {
        speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) 
            - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) 
            - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
    }
    let motion_rate = speed_x *PostureModule::lr(fighter.module_accessor) * param::SAMUS_FLOAT_SPECIAL_LW_GROUND_SPIN_SPEED_ADJUST;
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_loop"), 0.0, motion_rate, false, 0.0, false, false);


    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
        0.0,
        0.0,
        0.0,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPECIAL_LW_GROUND_BRAKE,
            0.0
        );
        sv_kinetic_energy!(
            set_accel_x_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPECIAL_LW_GROUND_ACCEL_X_MUL
        );
    }else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPECIAL_LW_AIR_BRAKE,
            0.0
        );
        sv_kinetic_energy!(
            set_accel_x_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPECIAL_LW_AIR_ACCEL_X_MUL
        );
    }
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
            0.0
        );
    }else {
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPECIAL_LW_STABLE_SPEED,
            0.0
        );
    }
    sv_kinetic_energy!(
        set_accel_x_add,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        param::SAMUS_FLOAT_SPECIAL_LW_ACCEL_X_ADD
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        param::SAMUS_FLOAT_SPECIAL_LW_ACCEL_GRAVITY
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        speed_x,
        0.0
    );
    VarModule::on_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_ENABLE);
    VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_HOP);
    fighter.sub_shift_status_main(L2CValue::Ptr(down_special_loop_status_loop as *const () as _))
}
pub unsafe fn down_special_loop_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    //rotation

    let lr = PostureModule::lr(fighter.module_accessor);
    let motion_rate_prev = MotionModule::rate(fighter.module_accessor);
    let motion_frame_end = MotionModule::end_frame(fighter.module_accessor);
    let motion_frame_prev = MotionModule::prev_frame(fighter.module_accessor);
    let motion_frame_curr = MotionModule::frame(fighter.module_accessor);

    println!("motion_rate_prev :{:?}", motion_rate_prev);
    println!("motion_frame_end :{:?}", motion_frame_end);
    println!("motion_frame_prev :{:?}", motion_frame_prev);
    println!("motion_frame_curr :{:?}", motion_frame_curr);

    let mut motion_rate;
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        motion_rate = lr
            * param::SAMUS_FLOAT_SPECIAL_LW_GROUND_SPIN_SPEED_ADJUST 
            * KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }else {
        let motion_rate_target = ControlModule::get_stick_x(fighter.module_accessor) * param::SAMUS_FLOAT_SPECIAL_LW_AIR_SPIN_SPEED_MAX * lr;

        if motion_rate_target > motion_rate_prev {
            if motion_rate_prev > 0.0 {
                motion_rate = motion_rate_prev + param::SAMUS_FLOAT_SPECIAL_LW_AIR_SPIN_SPEED_ACCEL;
            }else {
                motion_rate = motion_rate_prev + param::SAMUS_FLOAT_SPECIAL_LW_AIR_SPIN_SPEED_BRAKE;
            }
            if motion_rate_target < motion_rate {
                motion_rate = motion_rate_target;
            }
        }else if motion_rate_target < motion_rate_prev {
            if motion_rate_prev < 0.0 {
                motion_rate = motion_rate_prev - param::SAMUS_FLOAT_SPECIAL_LW_AIR_SPIN_SPEED_ACCEL;
            }else {
                motion_rate = motion_rate_prev - param::SAMUS_FLOAT_SPECIAL_LW_AIR_SPIN_SPEED_BRAKE;
            }
            if motion_rate_target > motion_rate {
                motion_rate = motion_rate_target;
            }
        }else {
            motion_rate = motion_rate_prev;
        }
    }
    MotionModule::set_rate(fighter.module_accessor, motion_rate);
    println!("motion_rate_new :{:?}", MotionModule::rate(fighter.module_accessor));

    if motion_rate > 0.0
    && motion_frame_curr >= motion_frame_end {
        let frame = motion_frame_prev + motion_rate_prev + -motion_frame_end;
        MotionModule::set_frame(fighter.module_accessor, frame, false);
    }else if motion_rate < 0.0
    && motion_frame_curr <= 0.0 {
        let frame = motion_frame_prev + motion_rate_prev + motion_frame_end;
        MotionModule::set_frame(fighter.module_accessor, frame, false);
    }
    println!("motion_frame_new :{:?}", MotionModule::frame(fighter.module_accessor));



    //bomb-jump
    if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_HOP) {
        VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_HOP);
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            GroundModule::set_attach_ground(fighter.module_accessor, false);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            // fighter.set_situation(SITUATION_KIND_AIR.into());
            sv_kinetic_energy!( 
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                param::SAMUS_FLOAT_SPECIAL_LW_AIR_BRAKE,
                0.0
            );
            sv_kinetic_energy!(
                set_accel_x_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                param::SAMUS_FLOAT_SPECIAL_LW_AIR_ACCEL_X_MUL
            );
        }
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            param::SAMUS_FLOAT_SPECIAL_LW_JUMP_HIGHT,
            0.0
        );
    }
    //jump
    else if (fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND
    || fighter.global_table[0x17].get_i32() == *SITUATION_KIND_GROUND)
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        LANDING_EFFECT(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        GroundModule::set_attach_ground(fighter.module_accessor, false);
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            param::SAMUS_FLOAT_SPECIAL_LW_JUMP_HIGHT,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPECIAL_LW_AIR_BRAKE,
            0.0
        );
        sv_kinetic_energy!(
            set_accel_x_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            param::SAMUS_FLOAT_SPECIAL_LW_AIR_ACCEL_X_MUL
        );
    }
    //ground/air transition
    else if fighter.global_table[0x16].get_i32() != fighter.global_table[0x17].get_i32() {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            PLAY_LANDING_SE(fighter, Hash40::new("se_samus_landing01"));
            LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 5, 0, 0, 0, false);
            // fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                param::SAMUS_FLOAT_SPECIAL_LW_GROUND_BRAKE,
                0.0
            );
            sv_kinetic_energy!(
                set_accel_x_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                param::SAMUS_FLOAT_SPECIAL_LW_GROUND_ACCEL_X_MUL
            );
        }else {
            // fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                param::SAMUS_FLOAT_SPECIAL_LW_AIR_BRAKE,
                0.0
            );
            sv_kinetic_energy!(
                set_accel_x_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                param::SAMUS_FLOAT_SPECIAL_LW_AIR_ACCEL_X_MUL
            );
        }
    } 
    //bomb
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        if ArticleModule::get_active_num(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB)
        < WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("bomb_max_req")) as u64 {
            ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, false, -1);
            ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    //end
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW.into(), true.into());
        return true.into()
    }else {
        return false.into()
    }
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn down_special_loop_status_end(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}
//end
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn down_special_end_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,/*FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,*/
        0,/*FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,*/
        0,/*FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,*/
        *FS_SUCCEEDS_KEEP_VISIBILITY
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW) as u64,
        *FIGHTER_STATUS_ATTR_DISABLE_SWIM as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    VarModule::set_int(fighter.battle_object, status::SAMUS_INT_SPECIAL_LW_JUMP_COUNT_FIX, jump_count);
    0.into()
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn down_special_end_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("special_lw_end_main :{:?}", WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT));
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) == false {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        let jump_count = VarModule::get_int(fighter.battle_object, status::SAMUS_INT_SPECIAL_LW_JUMP_COUNT_FIX);
        WorkModule::set_int(fighter.module_accessor, jump_count, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_end"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        let control_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let control_speed_x = KineticEnergy::get_speed_x(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(control_energy));
        let gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let gravity_speed_y = KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(gravity_energy));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
                0.0
            );
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                control_speed_x,
                0.0
            );
        }
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            gravity_speed_y
        );

        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < 0.0 {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
            speedbooster_off(fighter);
        }
    }
    ControlModule::clear_command(fighter.module_accessor, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(down_special_end_status_loop as *const () as _))
}
pub unsafe fn down_special_end_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    //ground/air transition
    if fighter.global_table[0x16].get_i32() != fighter.global_table[0x17].get_i32() {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            // fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_lw_end"), -1.0, 1.0, 0.0);
        }else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            // fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_lw_end"), -1.0, 1.0, 0.0);
        }
    }
    //bomb-jump
    if VarModule::is_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_HOP) {
        VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_HOP);
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::set_attach_ground(fighter.module_accessor, false);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            // fighter.set_situation(SITUATION_KIND_AIR.into());
            sv_kinetic_energy!( 
                set_brake,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                param::SAMUS_FLOAT_SPECIAL_LW_AIR_BRAKE,
                0.0
            );
            sv_kinetic_energy!(
                set_accel_x_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                param::SAMUS_FLOAT_SPECIAL_LW_AIR_ACCEL_X_MUL
            );
        }
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            param::SAMUS_FLOAT_SPECIAL_LW_JUMP_HIGHT,
            0.0
        );
    }
    //end
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), true.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        }
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    return false.into()
}
#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn down_special_end_status_end(_fighter: &mut L2CFighterCommon) -> L2CValue {0.into()}

//dissables bomb-jump status change
#[status_script(agent = "samus_bomb", status = WEAPON_SAMUS_BOMB_STATUS_KIND_BURST_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn bomb_explode_status_init(weapon: &mut L2CFighterCommon) -> L2CValue {
    original!(weapon);
    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_SAMUS_BOMB_INSTANCE_WORK_ID_INT_BOMBJUMP);//disables vanilla bomb-jump
    let fighter_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let fighter_battle_object = get_battle_object_from_id(fighter_id);
    let fighter_boma = sv_battle_object::module_accessor(fighter_id);
    let fighter_pos_x = PostureModule::pos_x(fighter_boma);
    let fighter_pos_y = PostureModule::pos_y(fighter_boma);
    let weapon_pos_x = PostureModule::pos_x(weapon.module_accessor);
    let weapon_pos_y = PostureModule::pos_y(weapon.module_accessor);
    let distance = sv_math::vec2_distance(fighter_pos_x, fighter_pos_y, weapon_pos_x, weapon_pos_y);
    let param_bomb_jump_hit_size = WorkModule::get_param_float(fighter_boma, hash40("param_special_lw"), hash40("sp_lw_bj_hit_size"));
    if distance <= param_bomb_jump_hit_size
    && (StatusModule::status_kind(fighter_boma) ==  *FIGHTER_STATUS_KIND_SPECIAL_LW
        || StatusModule::status_kind(fighter_boma) ==  *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW
        || StatusModule::status_kind(fighter_boma) ==  *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW)
    && VarModule::is_flag(fighter_battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_ENABLE) {
        VarModule::on_flag(fighter_battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_HOP);
    }
    0.into()
}

//start scripts
#[acmd_script( agent = "samus", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME )]
unsafe fn down_special_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_ENABLE)
    }
}
#[acmd_script( agent = "samus", scripts = ["expression_speciallw", "expression_specialairlw"], category = ACMD_EXPRESSION )]
unsafe fn down_special_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_TOP);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
    }
}
#[acmd_script( agent = "samus", scripts = ["sound_speciallw", "sound_specialairlw"], category = ACMD_SOUND )]
unsafe fn down_special_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_escape_ex"));
        // PLAY_SE(fighter, Hash40::new("se_samus_landing01"));
    }
}
#[acmd_script( agent = "samus", scripts = ["effect_speciallw", "effect_specialairlw"], category = ACMD_EFFECT )]
unsafe fn down_special_eff(_fighter : &mut L2CAgentBase) {
}
//end scripts
#[acmd_script( agent = "samus", scripts = ["game_speciallwend", "game_specialairlwend"], category = ACMD_GAME )]
unsafe fn down_special_end_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, status::SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_ENABLE)
    }
}
#[acmd_script( agent = "samus", scripts = ["expression_speciallwend", "expression_specialairlwend"], category = ACMD_EXPRESSION )]
unsafe fn down_special_end_exp(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
    }
}
#[acmd_script( agent = "samus", scripts = ["sound_speciallwend", "sound_specialairlwend"], category = ACMD_SOUND )]
unsafe fn down_special_end_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        // PLAY_SE(fighter, Hash40::new("se_samus_escape_ex"));
        PLAY_SE(fighter, Hash40::new("se_samus_escape_ex"));
    }
}
#[acmd_script( agent = "samus", scripts = ["effect_speciallwend", "effect_specialairlwend"], category = ACMD_EFFECT )]
unsafe fn down_special_end_eff(_fighter : &mut L2CAgentBase) {}
//bomb
#[acmd_script( agent = "samus_bomb", script = "game_burstattack", category = ACMD_GAME )]
unsafe fn bomb_explode_game(weapon : &mut L2CAgentBase) {
    if is_excute(weapon) {
        //increased hitstop 0.3 -> 1.0
        ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, 60, 30, 0, 40, 7.38, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 6, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        ControlModule::set_rumble(weapon.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(weapon.lua_state_agent, 3.0);
    if is_excute(weapon) {
        AREA_WIND_2ND_RAD(weapon, 0, 0.5, 0.02, 1000, 1, 0, 0, 16);
    }
    wait(weapon.lua_state_agent, 4.0);
    if is_excute(weapon) {
        AreaModule::erase_wind(weapon.module_accessor, 0);
    }
    wait(weapon.lua_state_agent, 5.0);
    if is_excute(weapon) {
        AttackModule::set_size(weapon.module_accessor, 0, 4.9);
    }
    wait(weapon.lua_state_agent, 3.0);
    if is_excute(weapon) {
        AttackModule::set_size(weapon.module_accessor, 0, 2.5);
    }
    wait(weapon.lua_state_agent, 3.0);
    if is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
}

////




pub fn install() {
    smashline::install_status_scripts!(
        down_special_start_status_pre,
        down_special_start_status_main,
        down_special_start_status_end,

        down_special_loop_status_pre,
        down_special_loop_status_main,
        down_special_loop_status_end,

        down_special_end_status_pre,
        down_special_end_status_main,
        down_special_end_status_end,

        bomb_explode_status_init,
    );
    smashline::install_acmd_scripts!(
        down_special_game,
        down_special_exp,
        down_special_snd,
        down_special_eff,
        down_special_end_game,
        down_special_end_exp,
        down_special_end_snd,
        down_special_end_eff,

        bomb_explode_game
    );
}
