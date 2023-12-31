pub mod param {
    pub const SAMUS_FLOAT_SPECIAL_HI_HOP : f32 = 1.3;
    pub const SAMUS_FLOAT_SPECIAL_HI_GBEAM_ANGLE_MIN: f32 = 0.0;
    pub const SAMUS_FLOAT_SPECIAL_HI_GBEAM_ANGLE_MAX: f32 = 90.0;
    
    pub const SAMUS_FLOAT_SPECIAL_LW_STABLE_SPEED : f32 = 1.3;
    pub const SAMUS_FLOAT_SPECIAL_LW_ACCEL_X_ADD : f32 = 0.04;
    pub const SAMUS_FLOAT_SPECIAL_LW_GROUND_ACCEL_X_MUL : f32 = 0.05;
    pub const SAMUS_FLOAT_SPECIAL_LW_AIR_ACCEL_X_MUL : f32 = 0.005;
    pub const SAMUS_FLOAT_SPECIAL_LW_ACCEL_GRAVITY : f32 = -0.07;
    pub const SAMUS_FLOAT_SPECIAL_LW_GROUND_BRAKE : f32 = 0.03;
    pub const SAMUS_FLOAT_SPECIAL_LW_AIR_BRAKE : f32 = 0.007;
    pub const SAMUS_FLOAT_SPECIAL_LW_JUMP_HIGHT : f32 = 1.47;
    pub const SAMUS_FLOAT_SPECIAL_LW_GROUND_SPIN_SPEED_ADJUST : f32 = 15.0;
    pub const SAMUS_FLOAT_SPECIAL_LW_AIR_SPIN_SPEED_MAX : f32 = 15.0;
    pub const SAMUS_FLOAT_SPECIAL_LW_AIR_SPIN_SPEED_ACCEL : f32 = 0.5;
    pub const SAMUS_FLOAT_SPECIAL_LW_AIR_SPIN_SPEED_BRAKE : f32 = 0.3;

    pub const SAMUS_FLOAT_SPECIAL_N_RECOIL_FRAME : f32 = 10.0;
    pub const SAMUS_INT_SPECIAL_N_HOMINGMISSILE_CHARGE_FRAME : i32 = 30;
    pub const SAMUS_INT_SPECIAL_N_HOMINGMISSILE_MAX : i32 = 3;
    pub const SAMUS_INT_SPECIAL_N_HOMINGMISSILE_DELAY_FRAME : i32 = 5;
    pub const SAMUS_FLOAT_SPECIAL_N_HOMINGMISSILE_ANGLE_OFFSET : f32 = 15.0;
    pub const SAMUS_INT_SPECIAL_N_FIRE_CSHOT_FRAME : i32 = 0;
    pub const SAMUS_INT_SPECIAL_N_FIRE_MISSILE_FRAME : i32 = 30;

    pub const SAMUS_INT_SPECIAL_S_CHAIN_MAX : i32 = 2;
    pub const SAMUS_INT_SPECIAL_S_LOOP_FRAME : i32 = 6;
    pub const SAMUS_FLOAT_SPECIAL_S_SPEED : f32 = 4.0;
    pub const SAMUS_INT_SPECIAL_S_CHAIN_FRAME : i32 = 40;
    pub const SAMUS_FLOAT_SPECIAL_S_EFFECT_TRAIL_FRAME : f32 = 6.0;
    pub const SAMUS_INT_SPECIAL_S_RECHARGE_FRAME : i32 = 150;

    pub const SAMUS_INT_SPEEDBOOSTER_START_FRAME : i32 = 60;
    pub const SAMUS_INT_SPEEDBOOSTER_EFFECT_FRAME : i32 = 8;
    pub const SAMUS_FLOAT_SPEEDBOOSTER_MAX_SPEED : f32 = 3.0;
    pub const SAMUS_INT_SPEEDBOOSTER_STICK_FRAME : i32 = 10;
    pub const SAMUS_INT_SPEEDBOOSTER_FALL_FRAME : i32 = 120;
    pub const SAMUS_INT_SPEEDBOOSTER_WALL_JUMP_FRAME : i32 = 30;
    
    pub const SAMUS_INT_SHINESPARK_CHARGE_FRAME : i32 = 300;
    pub const SAMUS_INT_SHINESPARK_EFFECT_FRAME : i32 = 8;
    pub const SAMUS_FLOAT_SHINESPARK_SPECIAL_LW_SPIN_SPEED_STABLE : f32 = 40.0;
    pub const SAMUS_INT_SHINESPARK_AIM_FRAME : i32 = 50;
    pub const SAMUS_FLOAT_SHINESPARK_SPEED : f32 = 6.0;
    pub const SAMUS_INT_SHINESPARK_LOOP_FRAME : i32 = 15;
    pub const SAMUS_FLOAT_SHINESPARK_GROUND_BRAKE : f32 = 0.3;
    pub const SAMUS_FLOAT_SHINESPARK_AIR_BRAKE : f32 = 0.2;
}
