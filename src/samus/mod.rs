use crate::imports::*;
use crate::samus::vars::*;
use crate::samus::consts::*;
use crate::samus::other::shinespark::*;
use crate::samus::other::speedbooster::*;



#[smashline::fighter_init]
fn samus_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_SAMUS {
            fighter.global_table[0x39].assign(&L2CValue::Ptr(special_s_enabled_check as *const () as _));
        }
    }
}
unsafe extern "C" fn special_s_enabled_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_USED)
    && (VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_CHAIN_TIMER) <= 0
    || VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_CHAIN_COUNT) >= param::SAMUS_INT_SPECIAL_S_CHAIN_MAX) {
        return false.into()
    }
    return true.into()
}

#[fighter_frame( agent = FIGHTER_KIND_SAMUS )]
fn samus_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        println!("=======NEW_FRAME=======");
        // let control_energy_x = KineticEnergy::get_speed_x(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL)));
        // let control_energy_y = KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL)));
        // let stop_energy_x = KineticEnergy::get_speed_x(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP)));
        // let stop_energy_y = KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP)));
        // let motion_energy_x = KineticEnergy::get_speed_x(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION)));
        // let motion_energy_y = KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION)));
        // let ground_energy = KineticEnergy::get_speed_x(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GROUND_MOVEMENT)));
        // let damage_energy = KineticEnergy::get_speed_x(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE)));
        // let jostle_energy = KineticEnergy::get_speed_x(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE)));
        // let gravity_energy_x = KineticEnergy::get_speed_x(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY)));
        // let gravity_energy_y = KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY)));
        // let wind_energy_x = KineticEnergy::get_speed_x(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND)));
        // let wind_energy_y = KineticEnergy::get_speed_y(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND)));
        // let no_stop_energy = KineticEnergy::get_speed_x(std::mem::transmute::<u64, &mut smash::app::KineticEnergy>(KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP_NO_STOP)));

        // println!("control_energy:{:?}, {:?}", control_energy_x, control_energy_y);
        // println!("stop_energy   :{:?}, {:?}", stop_energy_x, stop_energy_y);
        // println!("motion_energy :{:?}, {:?}", motion_energy_x, motion_energy_y);
        // println!("ground_energy :{:?}", ground_energy);
        // println!("damage_energy :{:?}", damage_energy);
        // println!("jostle_energy :{:?}", jostle_energy);
        // println!("gravity_energy:{:?}, {:?}", gravity_energy_x, gravity_energy_y);
        // println!("wind_energy   :{:?}, {:?}", wind_energy_x, wind_energy_y);
        // println!("no_stop_energy:{:?}", no_stop_energy);
        // println!("----------------------------------");

        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        // let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);


        if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_HI_HOP_DISABLED) {
            if situation_kind != *SITUATION_KIND_AIR {
            // || status_kind == *FIGHTER_STATUS_KIND_DEAD
            // || status_kind == *FIGHTER_STATUS_KIND_CLIFF_CATCH {
                VarModule::off_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_HI_HOP_DISABLED);
            }
        }

        if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_USED)
        && status_kind != *FIGHTER_STATUS_KIND_SPECIAL_S
        && status_kind != *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A {
        // && status_kind != *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A {
            if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_RECHARGE_TIMER) > 0 {
                VarModule::dec_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_RECHARGE_TIMER);
                if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_RECHARGE_TIMER) == 7 {//<-- adjust flash/glow length
                    EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_staff_hold"), Hash40::new("waist"), 0, 1, 0, 0, 0, 0, 1.2, true, 0.9);
                    EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_item_get"), Hash40::new("waist"), 0, 1, 0, 0, 0, 0, 1.2, true, 0.9);
                    FLASH(fighter, 0.2, 0.4, 10.0, 0.5);
                    BURN_COLOR(fighter, 1, 1, 1, 0.9);
                    PLAY_SE(fighter, Hash40::new("se_samus_attackair_h01"));
                }
            }else {
                COL_NORMAL(fighter);
                BURN_COLOR_NORMAL(fighter);
                VarModule::off_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_USED);
            }
            if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_CHAIN_TIMER) > 0 {
                VarModule::dec_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_CHAIN_TIMER);
            }
        }
        //added effect removal to fix AFTER_IMAGE() bugging out for a single frame when persisting through a status
        if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF) {
            VarModule::dec_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_EFFECT_TRAIL_TIMER);
            if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_EFFECT_TRAIL_TIMER) <= 1 {
                VarModule::off_flag(fighter.battle_object, instance::SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF);
                VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPECIAL_S_EFFECT_TRAIL_TIMER, param::SAMUS_FLOAT_SPECIAL_S_EFFECT_TRAIL_FRAME as i32);
                EffectModule::clear_all_after_image(fighter.module_accessor, 0);
            }
        }


        if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) {
            //summon effects
            speedbooster_effect(fighter);

            //store shinespark
            if ControlModule::get_flick_y(fighter.module_accessor) <= 5 
            && ControlModule::get_stick_y(fighter.module_accessor) < 0.0 
            && ControlModule::get_stick_x(fighter.module_accessor)*lr < 0.25 {
                if status_kind == *FIGHTER_STATUS_KIND_SQUAT 
                || status_kind == *FIGHTER_STATUS_KIND_SQUAT_WAIT 
                || status_kind == *FIGHTER_STATUS_KIND_LANDING
                || status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT
                || status_kind == *FIGHTER_STATUS_KIND_DASH
                || status_kind == *FIGHTER_STATUS_KIND_RUN
                || status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE {
                    println!("-------flick_off-------");
                    speedbooster_off(fighter);
                    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, true);//shinespark-charge
                }else if (status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
                || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW) //end
                && CancelModule::is_enable_cancel(fighter.module_accessor) {
                    println!("-------flick+cancel_off-------");
                    speedbooster_off(fighter);
                    StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP, true);//shinespark-charge
                }
            }
            //kinetic stuff
            if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_PREV_STATUS_KIND) != status_kind {
                if WorkModule::get_param_float(fighter.module_accessor, smash::hash40("common"), smash::hash40("air_speed_x_limit")) < param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED 
                && situation_kind != *SITUATION_KIND_GROUND {
                    sv_kinetic_energy!(
                        set_limit_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                        param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
                        0.0
                    );
                }
                JostleModule::set_status(fighter.module_accessor, false);
                
                // if status_kind == *FIGHTER_STATUS_KIND_DASH {
                //     // MotionModule::set_rate(fighter.module_accessor, param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED);
                //     // WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_DASH_WORK_INT_COUNT);
                //     KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                //     KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                //     sv_kinetic_energy!(
                //         clear_speed,
                //         fighter,
                //         FIGHTER_KINETIC_ENERGY_ID_MOTION
                //     );
                //     sv_kinetic_energy!(
                //         set_speed,
                //         fighter,
                //         FIGHTER_KINETIC_ENERGY_ID_STOP,
                //         param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED*lr,
                //         0.0
                //     );
                // }else 
                if 
                // status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 || 
                status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {///////////////////////////////////////////////////////////////////////////////////
                    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                    sv_kinetic_energy!(
                        clear_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_MOTION
                    );
                    sv_kinetic_energy!(
                        set_stable_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_STOP,
                        param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
                        0.0
                    );
                    sv_kinetic_energy!(
                        set_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_STOP,
                        param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED*lr,
                        0.0
                    );
                }else 
                //footstool
                if status_kind == *FIGHTER_STATUS_KIND_TREAD_JUMP {
                    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_TREAD_JUMP_COUNT);//footstools performed
                    VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_FALL_TIMER, param::SAMUS_INT_SPEEDBOOSTER_FALL_FRAME);
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP, true);
                }else 
                //jump/fall 
                if status_kind == *FIGHTER_STATUS_KIND_JUMP 
                || status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL
                || status_kind == *FIGHTER_STATUS_KIND_FALL
                || status_kind == *FIGHTER_STATUS_KIND_FALL_AERIAL {
                    sv_kinetic_energy!(
                        set_stable_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                        param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
                        0.0
                    );
                    let speed_x = VarModule::get_float(fighter.battle_object, instance::SAMUS_FLOAT_SPEEDBOOSTER_PREV_SPEED_X);
                    sv_kinetic_energy!(
                        set_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                        speed_x,
                        0.0
                    );
                }//else 
                // //down-special
                // if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW //loop
                // || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW { //end
                //     sv_kinetic_energy!(
                //         set_stable_speed,
                //         fighter,
                //         FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                //         param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
                //         0.0
                //     );
                // }else if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW { //start
                //     KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                //     KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                //     sv_kinetic_energy!(
                //         clear_speed,
                //         fighter,
                //         FIGHTER_KINETIC_ENERGY_ID_MOTION
                //     );
                //     sv_kinetic_energy!(
                //         set_stable_speed,
                //         fighter,
                //         FIGHTER_KINETIC_ENERGY_ID_STOP,
                //         param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED,
                //         0.0
                //     );
                //     sv_kinetic_energy!(
                //         set_speed,
                //         fighter,
                //         FIGHTER_KINETIC_ENERGY_ID_STOP,
                //         param::SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED*lr,
                //         0.0
                //     );
                // }
                VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_PREV_STATUS_KIND, status_kind);
            }
            //update speed vars after kinetic stuff
            let curr = VarModule::get_float(fighter.battle_object, instance::SAMUS_FLOAT_SPEEDBOOSTER_CURR_SPEED_X);
            VarModule::set_float(fighter.battle_object, instance::SAMUS_FLOAT_SPEEDBOOSTER_PREV_SPEED_X, curr);
            let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) 
                - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) 
                - KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
            VarModule::set_float(fighter.battle_object, instance::SAMUS_FLOAT_SPEEDBOOSTER_CURR_SPEED_X, speed_x);
            

            ////ending conditions
            //conditional statuses
            if status_kind == *FIGHTER_STATUS_KIND_DASH
            || status_kind == *FIGHTER_STATUS_KIND_RUN
            || status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE
            || status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
            || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
            || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW //start
            || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW //loop
            || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW //end
            || status_kind == *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP //shinespark-charge
            || status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT
            || status_kind == *FIGHTER_STATUS_KIND_TREAD_JUMP
            || status_kind == *FIGHTER_STATUS_KIND_JUMP
            || status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL
            || status_kind == *FIGHTER_STATUS_KIND_WALL_JUMP
            || status_kind == *FIGHTER_STATUS_KIND_FALL
            || status_kind == *FIGHTER_STATUS_KIND_FALL_AERIAL
            || status_kind == *FIGHTER_STATUS_KIND_LANDING
            || status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT {
                //wall condition
                if (GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32) && lr > 0.0)
                || (GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32) && lr < 0.0) {
                    if situation_kind != *SITUATION_KIND_GROUND {
                        if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_TIMER) > 0 {
                            VarModule::dec_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_TIMER);
                        }else {
                            println!("-------wall_time_off-------");
                            speedbooster_off(fighter);
                        }
                    }else {
                        println!("-------wall_ground_off-------");
                        speedbooster_off(fighter);
                    }
                }else {
                    VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_TIMER, param::SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_FRAME);
                }
                //stick condition
                if ControlModule::get_stick_x(fighter.module_accessor)*lr >= 0.75 {
                    VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_STICK_TIMER, param::SAMUS_INT_SPEEDBOOSTER_STICK_FRAME);
                }else if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_STICK_TIMER) > 0 {
                    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3 
                    || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW { //start
                        if ControlModule::get_stick_x(fighter.module_accessor)*lr < -0.4 {
                            VarModule::dec_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_STICK_TIMER);
                        }
                    }else {
                        VarModule::dec_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_STICK_TIMER);
                    }
                }else {
                    println!("-------stick_off-------");
                    speedbooster_off(fighter);
                }
                //air condition
                if situation_kind != *SITUATION_KIND_GROUND {
                    if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_FALL_TIMER) > 0 {
                        VarModule::dec_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_FALL_TIMER);
                    }else {
                        println!("-------air_off-------");
                        speedbooster_off(fighter);
                    }
                }else {
                    VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_FALL_TIMER, param::SAMUS_INT_SPEEDBOOSTER_FALL_FRAME);
                }
                //dash-attack
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH 
                && MotionModule::frame(fighter.module_accessor) >= 30.0 {
                    println!("-------dash_attack_off-------");
                    speedbooster_off(fighter);
                }
            }else {
                println!("-------status_off-------");
                speedbooster_off(fighter);
            }

            // //end speedbooster
            // if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SPEEDBOOSTER_ON) == false {
            //     if status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW { //loop
            //         sv_kinetic_energy!(
            //             set_stable_speed,
            //             fighter,
            //             FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            //             param::SAMUS_FLOAT_SPECIAL_LW_STABLE_SPEED,
            //             0.0
            //         );
            //     }else if status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
            //     || status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
            //     || status_kind == *FIGHTER_STATUS_KIND_DASH {
            //         KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RUN_STOP);
            //     }else if situation_kind != *SITUATION_KIND_GROUND {
            //         let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
            //         sv_kinetic_energy!(
            //             set_stable_speed,
            //             fighter,
            //             FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            //             air_speed_x_stable,
            //             0.0
            //         );
            //     }
            //     JostleModule::set_status(fighter.module_accessor, true);
            //     // WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
            //     //clear effects
            //     COL_NORMAL(fighter);
            //     BURN_COLOR_NORMAL(fighter);
            //     STOP_SE(fighter, Hash40::new("se_item_specialflag_raise"));
            //     // STOP_SE(fighter, Hash40::new("se_samus_special_H01"));
            // }

        }else {
            if status_kind == *FIGHTER_STATUS_KIND_RUN {
                VarModule::dec_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_START_TIMER);
                if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_START_TIMER) <= 0 {
                    speedbooster_on(fighter);
                }
            }else {
                VarModule::set_int(fighter.battle_object, instance::SAMUS_INT_SPEEDBOOSTER_START_TIMER, param::SAMUS_INT_SPEEDBOOSTER_START_FRAME);
            }
        }
        //shinespark timer
        if VarModule::is_flag(fighter.battle_object, instance::SAMUS_FLAG_SHINESPARK_ON) {
            if VarModule::get_int(fighter.battle_object, instance::SAMUS_INT_SHINESPARK_CHARGE_TIMER) <= 0 
            || status_kind == *FIGHTER_STATUS_KIND_DEAD {
                shinespark_off(fighter);
            }else {
                VarModule::dec_int(fighter.battle_object, instance::SAMUS_INT_SHINESPARK_CHARGE_TIMER);
                //summon effects
                shinespark_effect(fighter);
                //shinespark jump input
                if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 
                && ControlModule::get_stick_x(fighter.module_accessor).abs() < 0.4 {
                    if CancelModule::is_enable_cancel(fighter.module_accessor) 
                    || status_kind == *FIGHTER_STATUS_KIND_JUMP
                    || status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL
                    || status_kind == *FIGHTER_STATUS_KIND_FALL 
                    || status_kind == *FIGHTER_STATUS_KIND_FALL_AERIAL 
                    || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW { //loop
                        shinespark_off(fighter);
                        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, true);//shinespark-start
                    }
                }
            }
        }
    }
}



pub mod normals;
pub mod specials;
pub mod other;
pub mod vars;
pub mod consts;

pub fn install() {
    normals::install();
    specials::install();
    other::install();
    vars::install();

    smashline::install_agent_init_callbacks!(
        samus_init
    );
    smashline::install_agent_frames!(
        samus_frame
    );
}