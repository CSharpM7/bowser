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
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 51, 66, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 23, 16);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_S_FLAG_HIT);
    }
}
#[acmd_script( agent = "koopa", scripts = ["effect_specialsthrowf","effect_specialairsthrowf"], category = ACMD_EFFECT)]
unsafe extern "C" fn effect_specialsthrowf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 2, 0, -6, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "koopa", scripts = ["sound_specialsthrowf","sound_specialairsthrowf"], category = ACMD_SOUND)]
unsafe extern "C" fn sound_specialsthrowf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
}

#[acmd_script( agent = "koopa", scripts = ["expression_specialsthrowf","expression_specialairsthrowf"], category = ACMD_EXPRESSION)]
unsafe extern "C" fn expression_specialsthrowf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
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

#[acmd_script( agent = "koopa", scripts = ["effect_specialsthrowb","effect_specialairsthrowb"], category = ACMD_EFFECT)]
unsafe extern "C" fn effect_specialsthrowb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 3, 0, 3, 0, 180, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

#[acmd_script( agent = "koopa", scripts = ["sound_specialsthrowb","sound_specialairsthrowb"], category = ACMD_SOUND)]
unsafe extern "C" fn sound_specialsthrowb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
}

#[acmd_script( agent = "koopa", scripts = ["expression_specialsthrowb","expression_specialairsthrowb"], category = ACMD_EXPRESSION)]
unsafe extern "C" fn expression_specialsthrowb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
}

pub fn install() {
    install_acmd_scripts!(
        effect_specialssquat,

        game_specialsthrowf,
        effect_specialsthrowf,
        sound_specialsthrowf,
        expression_specialsthrowf,

        game_specialsthrowb,
        effect_specialsthrowb,
        sound_specialsthrowb,
        expression_specialsthrowb,
    );
}