#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(unused_macros)]

pub mod imports {
    pub use {
        std::f32::{
            consts::PI,
            *
        },
        // libm::*,
        smash::{
            lua2cpp::*,
            hash40,
            phx::{
                Hash40,
                Vector2f,
                Vector3f//,
                // Vector4f
            },
            app::{
                sv_animcmd::frame,
                sv_animcmd::wait,
                lua_bind::{
                    Article,
                    KineticEnergy,
                    *
                },
                *
            },
            lib::{
                lua_const::*,
                *
            }
        },
        smashline::*,
        smash_script::{
            macros::*,
            *
        },
        custom_var::*
    };
    #[skyline::from_offset(0x3ac540)]
    pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;
}

// mod common;
mod samus;


#[skyline::main(name = "dread_samus")]
pub fn main() {
    // common::install();
    samus::install();
}