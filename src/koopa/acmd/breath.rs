use crate::imports::imports_acmd::*;

#[acmd_script( agent = "koopa_breath", script = "game_max", category = ACMD_GAME)]
unsafe fn game_max(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 10.8, 55, 60, 0, 80, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(weapon.module_accessor);
    }
    frame(weapon.lua_state_agent, 14.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 7.5, 55, 60, 0, 80, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(weapon.module_accessor);
    }
}

#[acmd_script( agent = "koopa_breath", script = "effect_max", category = ACMD_EFFECT)]
unsafe fn effect_max(weapon: &mut L2CAgentBase) {
    for _ in 0..100 {
        if macros::is_excute(weapon) {
            EFFECT_FOLLOW(weapon, Hash40::new("koopa_breath_m_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW(weapon, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
        }
        wait(weapon.lua_state_agent, 15.0);
        if macros::is_excute(weapon) {
            EFFECT_FOLLOW(weapon, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
        }
        wait(weapon.lua_state_agent, 15.0);
    }
}

#[acmd_script( agent = "koopa_breath", script = "game_maxmidbus", category = ACMD_GAME)]
unsafe fn game_max_midbus(weapon: &mut L2CAgentBase) {
    let mut halflife=30.0;
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 10.8, 55, 60, 0, 80, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(weapon.module_accessor);
        halflife = WorkModule::get_int(weapon.module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE) as f32 /2.0;
    }
    frame(weapon.lua_state_agent, halflife);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 7.5, 55, 60, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(weapon.module_accessor);
    }
}

#[acmd_script( agent = "koopa_breath", script = "effect_maxmidbus", category = ACMD_EFFECT)]
unsafe fn effect_max_midbus(weapon: &mut L2CAgentBase) {
    for _ in 0..100 {
        if macros::is_excute(weapon) {
            EFFECT_FOLLOW(weapon, Hash40::new("koopa_breath_m_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        for _ in 0..2 {
            if macros::is_excute(weapon) {
                EFFECT_FOLLOW(weapon, Hash40::new("sys_starrod_splash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, true);
                LAST_EFFECT_SET_COLOR(weapon,0.375, 1.0,1.0);
            }
            wait(weapon.lua_state_agent, 5.0);
        }
    }
}

#[acmd_script( agent = "koopa_breath", scripts = ["game_end","game_endmidbus"], category = ACMD_GAME)]
unsafe fn game_end(weapon: &mut L2CAgentBase) {
    frame(weapon.lua_state_agent, 1.0);
    if macros::is_excute(weapon) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
}

#[acmd_script( agent = "koopa_breath", script = "effect_end", category = ACMD_EFFECT)]
unsafe fn effect_end(weapon: &mut L2CAgentBase) {
    let lr = PostureModule::lr(weapon.module_accessor);
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_damage_fire"),
        &Vector3f{x: pos.x + 4.0*lr, y: pos.y, z:pos.z},
        &VECTOR_ZERO,
        2.0,
        0,
        -1,
        false,
        0
    );
}
#[acmd_script( agent = "koopa_breath", script = "effect_endmidbus", category = ACMD_EFFECT)]
unsafe fn effect_end_midbus(weapon: &mut L2CAgentBase) {
    let lr = PostureModule::lr(weapon.module_accessor);
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_hit_ice"),
        &Vector3f{x: pos.x + 4.0*lr, y: pos.y, z:pos.z},
        &VECTOR_ZERO,
        1.0,
        0,
        -1,
        false,
        0
    );
}


pub fn install() {
    install_acmd_scripts!(
        game_max,
        game_max_midbus,
        effect_max,
        effect_max_midbus,
        game_end,
        effect_end,
        effect_end_midbus,
    );
}