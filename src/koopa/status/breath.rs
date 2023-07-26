use crate::imports::imports_agent::*;

#[status_script(agent = "koopa_breath", status = WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let mut is_fireball = false;
    let mut is_midbus = false;
    WorkModule::set_int(weapon.module_accessor,0,*WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_INT_KIND);

    if utility::get_category(&mut *owner_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    {
        let owner_kind = utility::get_kind(&mut *owner_boma);
        if owner_kind == *FIGHTER_KIND_KOOPA {
            is_fireball = WorkModule::get_float(owner_boma,*FIGHTER_KOOPA_STATUS_BREATH_WORK_FLOAT_GENE_ANGLE) > 360.0;

            let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let owner_object = util::get_battle_object_from_id(owner_id);
            let entry_id = WorkModule::get_int(weapon.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
            if VarModule::is_flag(owner_object, koopa::instance::flag::IS_MIDBUS) {
                is_midbus = true;
                WorkModule::set_int(weapon.module_accessor,1,*WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_INT_KIND);
            }
        }
        //Doesn't even matter
        else if owner_kind == *FIGHTER_KIND_MURABITO || owner_kind == *FIGHTER_KIND_SHIZUE {
            let kind = WorkModule::get_int(weapon.module_accessor,*WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_INT_KIND);
            is_fireball = kind > 1;
        }
        //Kirby can't do fireball
        else if owner_kind == *FIGHTER_KIND_KIRBY {
            is_fireball = false;
        }
    }

    if (is_fireball){
        WorkModule::set_int(weapon.module_accessor,1,*WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_INT_KIND);
        WorkModule::set_float(weapon.module_accessor, 361.0, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_FLOAT_ANGLE);

        WorkModule::set_customize_no(weapon.module_accessor, 1, 0);
        PostureModule::set_scale(weapon.module_accessor, 1.0, false);

        let mot = if is_midbus {Hash40::new("max_midbus")} else {Hash40::new("max")};
        MotionModule::change_motion(
            weapon.module_accessor,
            mot,
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        let param_life =  WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("life")) as i32;
        WorkModule::set_int(weapon.module_accessor, param_life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        WorkModule::set_int(weapon.module_accessor, param_life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        
        let param_hit_decrease = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("hit_frames")) as i32;
        WorkModule::set_int(weapon.module_accessor, param_hit_decrease, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_INT_HIT_FRAME);

        KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        let lr = PostureModule::lr(weapon.module_accessor);
        let param_speed =  WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("max_speed"));
        WorkModule::set_float(weapon.module_accessor, param_speed, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_FLOAT_SPEED_MUL);
        
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            param_speed * lr,
            0.0
        );
        sv_kinetic_energy!(
            set_stable_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            -1.0,
            -1.0
        );
        sv_kinetic_energy!(
            set_accel,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            0.0
        );

        weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(movemax_substatus as *const () as _));
        weapon.fastshift(L2CValue::Ptr(movemax_main_loop as *const () as _));

        return 0.into()
    }
    else{
        return original!(weapon);
    }
}

unsafe extern "C" fn movemax_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    if param_3.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}
unsafe extern "C" fn movemax_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT)
    {
        let param_hit_decrease = WorkModule::get_int(weapon.module_accessor, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_INT_HIT_FRAME);
        WorkModule::sub_int(weapon.module_accessor, param_hit_decrease,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    let life = WorkModule::get_int(weapon.module_accessor,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 {
        EFFECT_OFF_KIND(weapon,Hash40::new("koopa_breath_m_fire"),false,false);
        AttackModule::clear_all(weapon.module_accessor);
        
        let is_midbus= MotionModule::motion_kind(weapon.module_accessor) == Hash40::new("max_midbus").hash;
        let mot = if is_midbus {Hash40::new("end_midbus")} else {Hash40::new("end")};
        MotionModule::change_motion(
            weapon.module_accessor,
            mot,
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        StatusModule::change_status_force(weapon.module_accessor, WEAPON_KOOPA_BREATH_STATUS_KIND_NONE.into(), false.into());
        return 0.into();
    }
    
    0.into()
}


pub fn install() {
    install_status_scripts!(
        move_main,
    );
}