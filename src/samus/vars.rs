#![allow(non_snake_case)]
use crate::imports::*;

pub mod instance {//0x0???
    pub const SAMUS_FLAG_SPECIAL_HI_HOP_DISABLED : i32 = 0x0000;

    pub const SAMUS_FLAG_SPECIAL_S_REVERSE : i32 = 0x0001;
    pub const SAMUS_INT_SPECIAL_S_CHAIN_COUNT : i32 = 0x0002;
    pub const SAMUS_FLAG_SPECIAL_S_USED : i32 = 0x0003;
    pub const SAMUS_INT_SPECIAL_S_CHAIN_TIMER : i32 = 0x0004;
    pub const SAMUS_FLAG_SPECIAL_S_EFFECT_TRAIL_OFF : i32 = 0x0005;
    pub const SAMUS_INT_SPECIAL_S_EFFECT_TRAIL_TIMER : i32 = 0x0006;
    pub const SAMUS_INT_SPECIAL_S_RECHARGE_TIMER : i32 = 0x0007;

    pub const SAMUS_INT_SPEEDBOOSTER_START_TIMER : i32 = 0x0008;
    pub const SAMUS_FLAG_SPEEDBOOSTER_ON : i32 = 0x0009;
    pub const SAMUS_INT_SPEEDBOOSTER_EFFECT_TIMER : i32 = 0x000A;
    pub const SAMUS_INT_SPEEDBOOSTER_STICK_TIMER : i32 = 0x000B;
    pub const SAMUS_INT_SPEEDBOOSTER_FALL_TIMER : i32 = 0x000C;
    pub const SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_TIMER : i32 = 0x000D;
    pub const SAMUS_INT_SPEEDBOOSTER_PREV_STATUS_KIND : i32 = 0x000E;
    pub const SAMUS_FLOAT_SPEEDBOOSTER_PREV_SPEED_X : i32 = 0x000F;
    pub const SAMUS_FLOAT_SPEEDBOOSTER_CURR_SPEED_X : i32 = 0x0013;
    
    pub const SAMUS_FLAG_SHINESPARK_ON : i32 = 0x0010;
    pub const SAMUS_INT_SHINESPARK_CHARGE_TIMER : i32 = 0x0011;
    pub const SAMUS_INT_SHINESPARK_EFFECT_TIMER : i32 = 0x0012;
    
    ////weapon
    pub const SAMUS_SUPERMISSILE_FLOAT_ANGLE : i32 = 0x1000;
}
pub mod status {//0x1???
    //normals
    pub const SAMUS_FLAG_ATTACK_LW3_CHECK_CEIL : i32 = 0x1000;
    //special-hi
    pub const SAMUS_FLOAT_SPECIAL_HI_ANGLE : i32 = 0x1000;
    pub const SAMUS_FLAG_SPECIAL_HI_LOCK_ANGLE : i32 = 0x1001;
    pub const SAMUS_FLAG_SPECIAL_HI_FIX_GBEAM_POS : i32 = 0x1002;
    //special-lw
    pub const SAMUS_INT_SPECIAL_LW_JUMP_COUNT_FIX : i32 = 0x1000;
    pub const SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_ENABLE : i32 = 0x1001;
    pub const SAMUS_FLAG_SPECIAL_LW_BOMB_JUMP_HOP : i32 = 0x1002;
    //special-n
    pub const SAMUS_FLOAT_SPECIAL_N_ANGLE : i32 = 0x1000;
    pub const SAMUS_FLAG_SPECIAL_N_MISSILE_MODE : i32 = 0x1001;
    pub const SAMUS_FLAG_SPECIAL_N_IS_CHARGE : i32 = 0x1002;
    pub const SAMUS_INT_SPECIAL_N_MISSILE_COUNT : i32 = 0x1003;
    pub const SAMUS_INT_SPECIAL_N_HOMINGMISSILE_DELAY_COUNT : i32 = 0x1004;
    pub const SAMUS_INT_SPECIAL_N_FIRE_COUNT : i32 = 0x1005;
    //special-s
    pub const SAMUS_FLOAT_SPECIAL_S_LR : i32 = 0x1000;
    pub const SAMUS_INT_SPECIAL_S_LOOP_FRAME_TIMER : i32 = 0x1001;
    pub const SAMUS_FLAG_SPECIAL_S_CHAIN_CANCEL_ENABLE : i32 = 0x1002;
    ////shinespark
    pub const SAMUS_FLAG_SHINESPARK_IS_SPECIAL_LW : i32 = 0x1000;
    //aim
    pub const SAMUS_INT_SHINESPARK_AIM_TIMER : i32 = 0x1001;
    pub const SAMUS_INT_SHINESPARK_AIM_EFFECT_TIMER : i32 = 0x1002;
    //loop
    pub const SAMUS_INT_SHINESPARK_LOOP_TIMER : i32 = 0x1001;
    //end
    pub const SAMUS_FLAG_SHINESPARK_ENABLE_GRAVITY : i32 = 0x1001;
    pub const SAMUS_FLAG_SHINESPARK_ENABLE_CONTROL : i32 = 0x1002;
}

//special thanks the Wuboy/Wubor Patch for porting HDR's VarModule
#[skyline::hook(offset = 0x3af2e0)]
pub unsafe fn battleobjectmoduleaccessor__initialize_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: *const u64) {
    original!()(module_accessor, param_1);
    // println!("[CustomVarManager] Initialize");
    let object_id = (*module_accessor).battle_object_id;
    // println!("[CustomVarManager] Initializing VarModule for {:#x}", object_id);
    // println!("[CustomVarManager] VarModule Count before adding: {}", CustomVarManager::count());
    if object_id != 0x50000000 {
        // println!("[CustomVarManager] Object ID is not invalid! Adding...");
        CustomVarManager::reset_var_module(module_accessor, false);
    }
    // println!("[CustomVarManager] VarModule Count after adding: {}", CustomVarManager::count());
}
#[skyline::hook(offset = 0x3af9f0)]
pub unsafe fn battleobjectmoduleaccessor__start_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    original!()(module_accessor, param_1);
    // let object_id = (*module_accessor).battle_object_id;
    // println!("[CustomVarManager] Start");
    // println!("[CustomVarManager] Starting VarModule for {:#x}", object_id);
    VarModule::start(module_accessor);
}
#[skyline::hook(offset = 0x3afde0)]
pub unsafe fn battleobjectmoduleaccessor__end_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    // println!("[CustomVarManager] End");
    // let object_id = (*module_accessor).battle_object_id;
    // println!("[CustomVarManager] Ending VarModule for {:#x} (not really)", object_id);
    CustomVarManager::reset_var_module(module_accessor, true);
    original!()(module_accessor, param_1)
}
#[skyline::hook(offset = 0x3af700)]
pub unsafe fn battleobjectmoduleaccessor__finalize_modules(module_accessor: *mut BattleObjectModuleAccessor) {
    // let object_id = (*module_accessor).battle_object_id;
    // println!("[CustomVarManager] Finalize");
    // println!("[CustomVarManager] Finalizing VarModule for {:#x}", object_id);
    // println!("[CustomVarManager] VarModule Count before removing: {}", CustomVarManager::count());
    CustomVarManager::remove_var_module(module_accessor);
    // println!("[CustomVarManager] VarModule Count after removing: {}", CustomVarManager::count());
    original!()(module_accessor)
}

pub fn install() {
    skyline::install_hooks!(
        battleobjectmoduleaccessor__initialize_modules,
        battleobjectmoduleaccessor__start_modules,
        battleobjectmoduleaccessor__end_modules,
        battleobjectmoduleaccessor__finalize_modules
    );
}