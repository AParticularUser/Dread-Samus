#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros,
    ambiguous_glob_reexports
)]


pub mod imports {
    pub use {
        std::f32::consts::PI,
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
                sv_animcmd::{
                    frame,
                    wait
                },
                lua_bind::{
                    Article,
                    KineticEnergy,
                    *
                },
                utility,
                *
            },
            lib::{
                lua_const::*,
                *
            }
        },
        smashline::*,
        smash_script::{
            macros::is_excute,
            macros::*,
            *
        },
        custom_var::*
    };
}


mod common;
mod samus;

#[skyline::main(name = "dread_samus")]
pub fn main() {
    samus::install();
}