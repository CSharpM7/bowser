use crate::imports::imports_agent::*;
use {
    once_cell::sync::Lazy,
};


// AGENT INIT AND CALLBACKS
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Remove fireball ready effect
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ENTRY,*FIGHTER_STATUS_KIND_DEAD,*FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,*FIGHTER_STATUS_KIND_LOSE]) || !sv_information::is_ready_go() {
        EFFECT_OFF_KIND(fighter,Hash40::new("koopa_breath_m_fire"),false,false);
        VarModule::set_int(fighter.battle_object, koopa::instance::int::FIREBALL_EFFECT_ID,0);
        VarModule::set_int(fighter.battle_object, koopa::instance::int::FIREBALL_COOLDOWN_FRAME,MAX_COOLDOWN);
    }
    true.into()
}

pub unsafe fn agent_rebirth(fighter: &mut L2CFighterCommon){

    let charged_effect =  VarModule::get_int(fighter.battle_object, koopa::instance::int::FIREBALL_EFFECT_ID);
    if (charged_effect <= 0
    || EffectModule::is_exist_effect(fighter.module_accessor, charged_effect as u32)){
        EffectModule::kill(fighter.module_accessor, charged_effect as u32, false,false);
        VarModule::set_int(fighter.battle_object, koopa::instance::int::FIREBALL_EFFECT_ID,0);
    }
    VarModule::set_int(fighter.battle_object, koopa::instance::int::FIREBALL_COOLDOWN_FRAME,MAX_COOLDOWN);
    
}
#[status_script(agent = "koopa", status = FIGHTER_STATUS_KIND_REBIRTH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn rebirth_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    agent_rebirth(fighter);
    return original!(fighter);
}

unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_KOOPA {
        return;
    }
    agent_rebirth(fighter);
    reset_vars(fighter.battle_object);
    if is_training_mode() {
        VarModule::set_int(fighter.battle_object, koopa::instance::int::FIREBALL_COOLDOWN_FRAME,0);
    }
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   

    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let midbus = crate::data::gamemode::is_Midbus(color);
    VarModule::set_flag(fighter.battle_object, koopa::instance::flag::IS_MIDBUS, midbus);
}


#[smashline::fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}
#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(
        agent_init
    );
    install_agent_resets!(
        agent_reset
    );
    install_status_scripts!(
        rebirth_pre,
    );
}