use crate::imports::imports_acmd::*;

#[acmd_script( agent = "koopa", scripts = ["game_specialnmax","game_specialairnmax"], category = ACMD_GAME)]
unsafe fn game_specialnmax(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        VarModule::set_int(fighter.battle_object, &mut koopa::instance::int::FIREBALL_COOLDOWN_FRAME,MAX_COOLDOWN);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_START);
    }
}

#[acmd_script( agent = "koopa", scripts = ["effect_specialnmax","effect_specialairnmax"], category = ACMD_EFFECT)]
unsafe fn effect_specialnmax(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_explosion_sign"), Hash40::new("jaw"), 0, 1.0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_RATE(fighter,1.5);

        if fighter.is_motion(Hash40::new("special_n_max")){
            macros::LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        macros::FLASH(fighter, 0.961, 0.569, 0.569, 0.392);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 1, 0.537, 0.537, 0.588);
        macros::FLASH_FRM(fighter, 20, 0, 0, 0, 0);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_explosion_sign"),false,false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        //macros::EFFECT_FOLLOW(fighter, Hash40::new("koopa_breath_m_fire"), Hash40::new("head"), -2, 5, 0, 180, 0, 50, 1, true);
        EFFECT_OFF_KIND(fighter,Hash40::new("koopa_breath_m_fire"),false,false);

        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire_fly"), Hash40::new("jaw"), 0, 0, 0, 180, 0, 50, 0.5, true);
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        macros::COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("koopa_appeal_s"), Hash40::new("mouth2"), 0, -1.3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter,2.0,0.5,0);
    }
}

#[acmd_script( agent = "koopa", scripts = ["sound_specialnmax","sound_specialairnmax"], category = ACMD_SOUND)]
unsafe fn sound_specialnmax(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_koopa_special_n01"));
    }
    wait(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        if fighter.is_motion(Hash40::new("special_n_max")){
            macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_koopa_step_left_m"));
        }
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_fire_m_damage"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack05"));
    }
}
#[acmd_script( agent = "koopa", scripts = ["expression_specialnmax","expression_specialairnmax"], category = ACMD_EXPRESSION)]
unsafe fn expression_specialnmax(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_specialnmax,
        sound_specialnmax,
        effect_specialnmax,
        expression_specialnmax
    );
}