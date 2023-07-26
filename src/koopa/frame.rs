use crate::imports::imports_agent::*;

// HDR
pub unsafe fn gimmick_flash(boma: &mut BattleObjectModuleAccessor) {
	if !app::sv_information::is_ready_go() {
		return
	}
    let fighter = get_fighter_common_from_accessor(boma);
    let lr = PostureModule::lr(fighter.module_accessor);
    let flash_y_offset = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);

    app::FighterUtil::flash_eye_info(fighter.module_accessor);

    if WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, flash_y_offset, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
    }
    else {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, flash_y_offset, 2, 0, 0, 0, 1.0, true);
    }
    LAST_EFFECT_SET_COLOR(fighter, 0.831, 0.686, 0.216);	
}

unsafe fn fireball_cooldown(fighter: &mut L2CFighterCommon) {
    //Ignore cooldown during respawn,death,entry and nspecial
    if (&[
        *FIGHTER_STATUS_KIND_ENTRY,*FIGHTER_STATUS_KIND_DEAD,*FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,*FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_SPECIAL_N
    ]).contains(&StatusModule::status_kind(fighter.module_accessor)) {
        return;
    }

    let cooleddown = VarModule::countdown_int(fighter.battle_object, &mut koopa::instance::int::FIREBALL_COOLDOWN_FRAME, 0);
    let charged_effect =  VarModule::get_int(fighter.battle_object, &mut koopa::instance::int::FIREBALL_EFFECT_ID);
    if !cooleddown {
        if charged_effect > 0 {
            VarModule::set_int(fighter.battle_object, &mut koopa::instance::int::FIREBALL_EFFECT_ID,0);
            if EffectModule::is_exist_effect(fighter.module_accessor, charged_effect as u32) {
                EffectModule::kill(fighter.module_accessor, charged_effect as u32, false,false);
            }
        }
        return;
    }

    if (charged_effect <= 0 
    || !EffectModule::is_exist_effect(fighter.module_accessor, charged_effect as u32))
    {
        if (charged_effect <= 0){
            gimmick_flash(fighter);
        }
        let pos = &Vector3f{x: 0.0, y: 1.0, z: 0.0};
        let rot = &Vector3f{x: 180.0, y: 0.0, z: 50.0};
        let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("koopa_breath_m_fire"), Hash40::new("jaw"), pos, rot, 1.0, true, 0, 0, 0, 0, 0, false, false) as u32;
        VarModule::set_int(fighter.battle_object, &mut koopa::instance::int::FIREBALL_EFFECT_ID,handle as i32);
    }
}

#[fighter_frame( agent = FIGHTER_KIND_KOOPA )]
fn koopa_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        fireball_cooldown(fighter);
    }
}


pub fn install() {
    smashline::install_agent_frames!(
        koopa_update
    );
}