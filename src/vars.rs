//use super::*;
//use crate::imports::imports_acmd::*;
use smash::phx::Vector3f;

pub const MAX_COOLDOWN : i32 = 600;
pub mod koopa {
    pub mod instance {
        pub mod flag {
            pub const IS_MIDBUS: i32 = 0x0100;
        }
        pub mod int {
            pub const FIREBALL_COOLDOWN_FRAME: i32 = 0x0100;
            pub const FIREBALL_EFFECT_ID: i32 = 0x0101;
        }
        pub mod float {
        }
    }
}
/* 
pub mod koopa {
    pub mod instance {
        pub mod flag {
            pub static mut IS_MIDBUS:[bool;8] = [false; 8];
        }
        pub mod int {
            pub static mut FIREBALL_COOLDOWN_FRAME:[i32;8] = [0; 8];
            pub static mut FIREBALL_EFFECT_ID:[i32;8] = [0; 8];
        }
        pub mod float {
        }
    }
}
*/
use {
    smash::{
        phx::*,
        app::*,
    },
    sharpsmashlinesuite::{
        getvar::*,
    }
};
pub unsafe fn reset_vars(object: *mut BattleObject){
    GetVarManager::reset_var_module(object,false);
    /* 
    let entry = get_entry_from_object(object);
    koopa::instance::flag::IS_MIDBUS[entry] = false;
    koopa::instance::int::FIREBALL_COOLDOWN_FRAME[entry] = 0;
    koopa::instance::int::FIREBALL_EFFECT_ID[entry] = 0;
    */
}