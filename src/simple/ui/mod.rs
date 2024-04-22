mod class_select;
mod healthbar;
mod hud;
mod in_game;
mod upgrade_select;

use bevy_replicon::core::common_conditions::has_authority;
use bevy_replicon_renet::renet::RenetClient;
pub use healthbar::HealthBar;
pub use hud::InfoText;

use bevy::prelude::*;
use super::state;

pub struct UIPlugin;

impl Plugin for UIPlugin
{
    fn build(&self, app: &mut App) {
        let host_or_client_cond = has_authority.or_else(resource_exists::<RenetClient>);
        app
            // Class select v
            .add_systems(OnEnter(state::GameState::ChoosingClass), class_select::setup_class_select_ui)
            .add_systems(Update, (class_select::handle_class_select_ui,).in_set(state::ChoosingClassSystems))
            .add_systems(OnExit(state::GameState::ChoosingClass), class_select::teardown_class_select_ui)
            .add_systems(OnEnter(state::GameState::InGame), in_game::setup_uis.run_if(host_or_client_cond.clone()))
            .add_systems(OnExit(state::GameState::InGame), in_game::cleanup_uis.run_if(host_or_client_cond.clone()))
            .add_systems(OnEnter(state::InGameState::Paused), in_game::on_pause.run_if(host_or_client_cond.clone()))
            .add_systems(OnExit(state::InGameState::Paused), in_game::on_resume.run_if(host_or_client_cond.clone()))
            .add_systems(OnEnter(state::InGameState::Break), in_game::on_enter_upgrade_select.run_if(host_or_client_cond.clone()))
            .add_systems(OnTransition { from: state::InGameState::Break, to: state::InGameState::Fighting }, in_game::on_upgrade_select_to_fighting.run_if(host_or_client_cond.clone()))
            .add_systems(FixedUpdate, (in_game::handle_resume_button, in_game::s_handle_next_wave_button).run_if(has_authority))

            .add_systems(FixedUpdate, (
                class_select::s_handle_go_in_game_ui,
            ).in_set(state::AuthoritySystems).in_set(state::ChoosingClassSystems))
            .add_systems(FixedUpdate, (
                hud::c_update_info_text,
                healthbar::c_add_healthbars,
                healthbar::c_update_healthbars,
                upgrade_select::c_create_upgrade_ui,
                upgrade_select::c_handle_upgrade_clicked,
            ).in_set(state::HostAndClientSystems))
            ;
    }
}
