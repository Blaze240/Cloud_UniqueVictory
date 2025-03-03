use {
    skyline_smash::app::utility::get_kind,
    smash::{
        app::{lua_bind::*, sv_animcmd::*, sv_battle_object::*,utility::*},
        hash40,
        lib::{lua_const::*, L2CAgent, L2CValue},
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::{Priority::*, *},
    crate::{
        EDGE_EXIST
    }
};

unsafe extern "C" fn cloud_sound_win2(agent: &mut L2CAgentBase) {
    if EDGE_EXIST {
        frame(agent.lua_state_agent, 28.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_cloud_win04"));
        }
    } else {
        frame(agent.lua_state_agent, 30.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_cloud_win02"));
        }
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_cloud_win02"));
    }
}

pub fn install() {
    Agent::new("cloud")
        .sound_acmd("sound_win2", cloud_sound_win2, Default)
        .install();
}
