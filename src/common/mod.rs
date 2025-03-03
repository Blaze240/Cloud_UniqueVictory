use {
    crate::EDGE_EXIST,
    smash::{
        app::{lua_bind::*, sv_system, utility},
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
    },
    smashline::*,
};

unsafe extern "C" fn edge_check_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = utility::get_kind(module_accessor);

        if fighter_kind == *FIGHTER_KIND_EDGE {
            EDGE_EXIST = true;
        }
        else{
            EDGE_EXIST = false;
        }
    }
}
pub fn install() {
    Agent::new("fighter")
        .on_line(Main, edge_check_frame)
        .install();
}
