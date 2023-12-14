use crate::imports::imports_acmd::*;

#[acmd_script( agent = "koopa", scripts = ["effect_specialssquat","effect_specialairssquat"], category = ACMD_EFFECT)]
unsafe fn effect_specialssquat(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_catch"), Hash40::new("haver"), 0, 1, 2, 0, 0, 0, 0.8, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        //macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 20, 27, 0, 0, 0, 2, false);
    }
}

#[acmd_script( agent = "koopa", scripts = ["game_specialsthrowf","game_specialairsthrowf"], category = ACMD_GAME)]
unsafe fn game_specialsthrowf(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 45, 66, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 23, 16);
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
    }
}
#[acmd_script( agent = "koopa", scripts = ["game_specialsthrowb","game_specialairsthrowb"], category = ACMD_GAME)]
unsafe fn game_specialsthrowb(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 45, 66, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        macros::CHECK_FINISH_CAMERA(fighter, 20, 15);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
    }
}

pub fn install() {
    install_acmd_scripts!(
        effect_specialssquat,
        game_specialsthrowf,
        game_specialsthrowb,
    );
}