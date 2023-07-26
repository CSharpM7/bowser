use crate::imports::imports_acmd::*;

#[acmd_script( agent = "koopa", scripts = ["effect_specialnstart","effect_specialairnstart"], category = ACMD_EFFECT)]
unsafe fn effect_specialnstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 0.961, 0.569, 0.569, 0.392);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 1, 0.537, 0.537, 0.588);
        macros::FLASH_FRM(fighter, 20, 0, 0, 0, 0);
    }
    wait(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::COL_NORMAL(fighter);
    }
}


#[acmd_script( agent = "koopa", scripts = ["game_specialnend","game_specialairnend"], category = ACMD_EFFECT)]
unsafe fn game_specialnend(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(fighter,1.0,31.0,16.0);
}

pub fn install() {
    install_acmd_scripts!(
        effect_specialnstart,
        game_specialnend,
    );
}