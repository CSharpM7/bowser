use crate::imports::imports_agent::*;


#[status_script(agent = "koopa", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let can_fireball = VarModule::get_int(fighter.battle_object, &mut koopa::instance::int::FIREBALL_COOLDOWN_FRAME) <= 0;
    if (!can_fireball){
        return original!(fighter);
    }
    else{
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_max").into(), Hash40::new("special_air_n_max").into(), false.into());
        fighter.sub_set_ground_correct_by_situation(true.into());

        fighter.sub_shift_status_main(L2CValue::Ptr(specialnmax_main_loop as *const () as _))
    }
}

unsafe extern "C" fn specialnmax_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_n_max").into(), Hash40::new("special_air_n_max").into(), true.into());
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
    }
    /*
    if fighter.is_situation(*SITUATION_KIND_GROUND){
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    } */

    WorkModule::set_float(fighter.module_accessor, 361.0, *FIGHTER_KOOPA_STATUS_BREATH_WORK_FLOAT_GENE_ANGLE);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_START){
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_START);
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH, false, -1);
        ArticleModule::set_float(fighter.module_accessor,*FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH, 361.0, *WEAPON_KOOPA_BREATH_INSTANCE_WORK_ID_FLOAT_ANGLE);
    }

    0.into()
}


#[status_script(agent = "koopa", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn specialn_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let can_fireball =  VarModule::get_int(fighter.battle_object, &mut koopa::instance::int::FIREBALL_COOLDOWN_FRAME) <= 0;
    if (!can_fireball){
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_CONTINUE_START)
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_BREATH_FLAG_CONTINUE_END)
        {
            if VarModule::get_int(fighter.battle_object, &mut koopa::instance::int::FIREBALL_COOLDOWN_FRAME) < MAX_COOLDOWN {
                VarModule::inc_int(fighter.battle_object, &mut koopa::instance::int::FIREBALL_COOLDOWN_FRAME);
            }
        }
        return original!(fighter);
    }
    else{
        return 0.into();
    }
}

#[status_script(agent = "koopa", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STOP)]
unsafe fn specialn_execstop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let can_fireball =  VarModule::get_int(fighter.battle_object, &mut koopa::instance::int::FIREBALL_COOLDOWN_FRAME) <= 0;
    if (!can_fireball){
        return original!(fighter);
    }
    else{
        return 0.into();
    }
}

pub fn install() {
    install_status_scripts!(
        specialn_main,
        specialn_exec,
        specialn_execstop,
    );
}