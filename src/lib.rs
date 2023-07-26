#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![deny(
    deprecated
)]

#[macro_use]
extern crate lazy_static;

mod koopa;

mod imports;
mod custom_vars;
pub mod vars;
pub mod data;
use data::gamemode::*;

pub use skyline::libc::c_char;

extern "C"{
    /// gets whether we are in training mode
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;
}


#[skyline::main(name = "smashline_bowsersbigblast")]
pub fn main() {
    println!("[smashline_bowsersbigblast::main] Loading...");
    custom_vars::install();
    //data::install();
    data::gamemode::set_gamemode();
    koopa::install();
    println!("[smashline_bowsersbigblast::main] Loaded!");
}