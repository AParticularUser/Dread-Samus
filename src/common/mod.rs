use crate::imports::*;

#[skyline::from_offset(0x3ac540)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;
