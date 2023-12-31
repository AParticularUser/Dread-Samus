use crate::imports::*;
use crate::samus::vars::*;
use crate::samus::specials::special_s::*;
use crate::samus::other::shinespark::*;
use crate::samus::other::speedbooster::*;

unsafe extern "C" fn samus_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[0x39].assign(&L2CValue::Ptr(special_s_enabled_check as *const () as _));
}

unsafe extern "C" fn samus_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        // let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        // let lr = PostureModule::lr(fighter.module_accessor);

        //up-special
        if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_HI_HOP_DISABLED) {
            if situation_kind != *SITUATION_KIND_AIR {
                VarModule::off_flag(fighter.module_accessor, instance::SAMUS_FLAG_SPECIAL_HI_HOP_DISABLED);
            }
        }
        //side-special
        special_s_recharge_check(fighter);
        special_s_effect_trail_check(fighter);
        //speedbooster
        speedbooster_opff(fighter);
        //shinespark timer
        if VarModule::is_flag(fighter.module_accessor, instance::SAMUS_FLAG_SHINESPARK_ON) {
            if VarModule::get_int(fighter.module_accessor, instance::SAMUS_INT_SHINESPARK_CHARGE_TIMER) <= 0 
            || status_kind == *FIGHTER_STATUS_KIND_DEAD {
                shinespark_off(fighter);
            }else {
                VarModule::dec_int(fighter.module_accessor, instance::SAMUS_INT_SHINESPARK_CHARGE_TIMER);
                //summon effects
                shinespark_effect(fighter);
                //shinespark jump input
                if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) != 0 
                && ControlModule::get_stick_x(fighter.module_accessor).abs() < 0.4 {
                    if (WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) >= WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)
                    && (CancelModule::is_enable_cancel(fighter.module_accessor)
                    || [*FIGHTER_STATUS_KIND_JUMP,
                        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
                        *FIGHTER_STATUS_KIND_FALL,
                        *FIGHTER_STATUS_KIND_FALL_AERIAL
                    ].contains(&status_kind)))
                    || status_kind == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW { //down-special loop status
                        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A, true);//shinespark-start
                        shinespark_off(fighter);
                    }
                }
            }
        }
    }
}

mod normals;
mod specials;
mod other;
mod vars;
mod consts;

pub fn install() {
    let agent = &mut smashline::Agent::new("samus");
    agent.on_start(samus_init);
    agent.on_line(Main, samus_frame);
    normals::install(agent);
    specials::install(agent);
    other::install(agent);
    vars::install();
    agent.install();
}
