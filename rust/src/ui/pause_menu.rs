use bevy::prelude::*;
use godot::classes::Input;
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::state::GameState;
use crate::state::state_manager::{LoadSceneMessage, SceneState};

#[derive(Resource, Default)]
pub struct PauseMenuAssets {
    pub resume_button: Option<GodotNodeHandle>,
    pub main_menu_button: Option<GodotNodeHandle>,
    pub initialized: bool,
    pub signal_connected: bool,
}

#[derive(NodeTreeView)]
pub struct PauseMenuUi {
    // Todo
    #[node("/root/Pause_menu/Button_manager/Resume")]
    pub resume_button: GodotNodeHandle,

    // Todo
    #[node("/root/Pause_menu/Button_manager/Main_menu")]
    pub main_menu_button: GodotNodeHandle,
}

fn toggle_pause_menu(
    mut godot: GodotAccess,
    state: Res<State<GameState>>,
    mut app_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    let input = godot.singleton::<Input>();

    if input.is_action_pressed("pause") {
        if *state.get() != GameState::InGame {
            godot_print!("Press but not Ingame");
            return;
        }

        app_state.set(GameState::PauseMenu);
        commands.trigger(LoadSceneMessage {
            scene_state: SceneState::Pause,
        });
    }
}

fn reset_menu_assets(mut menu_assets: ResMut<PauseMenuAssets>) {
    menu_assets.resume_button = None;
    menu_assets.main_menu_button = None;
    menu_assets.initialized = false;
    menu_assets.signal_connected = false;
}

fn initialized_pause_menu(mut menu_assets: ResMut<PauseMenuAssets>, mut scene_tree: SceneTreeRef) {
    if let Some(root) = scene_tree.get().get_root() {
        match PauseMenuUi::from_node(root) {
            Ok(menu_ui) => {
                godot_print!("Found menu node");
                menu_assets.resume_button = Some(menu_ui.resume_button);
                menu_assets.main_menu_button = Some(menu_ui.main_menu_button);
                menu_assets.initialized = true;
            }
            Err(_) => {}
        }
    } else {
        godot_print!("Main Menu scene not avaible");
    }
}

fn pause_menu_not_initialized(menu_assets: Res<PauseMenuAssets>) -> bool {
    !menu_assets.initialized
}

fn pause_menu_initialized_but_signals_not_connected(menu_assets: Res<PauseMenuAssets>) -> bool {
    menu_assets.initialized && !menu_assets.signal_connected
}

#[derive(Event, Debug, Clone)]
struct ResumeGameEvent;

#[derive(Event, Debug, Clone)]
struct MainMenuGameEvent;

fn connect_button(
    mut menu_assets: ResMut<PauseMenuAssets>,
    signals_start: GodotSignals<ResumeGameEvent>,
    signals_quit: GodotSignals<MainMenuGameEvent>,
) {
    if menu_assets.resume_button.is_some()
        && menu_assets.main_menu_button.is_some()
        && !menu_assets.signal_connected
    {
        if let Some(resume_handle) = menu_assets.resume_button {
            godot_print!("Connect start");
            signals_start.connect(
                resume_handle,
                BaseButtonSignals::PRESSED,
                None,
                |_args, _node_handle, _ent| Some(ResumeGameEvent),
            );
        }
        if let Some(main_menu_handle) = menu_assets.main_menu_button {
            godot_print!("Connect quit");
            signals_quit.connect(
                main_menu_handle,
                BaseButtonSignals::PRESSED,
                None,
                |_args, _node_handle, _ent| Some(MainMenuGameEvent),
            );
        }
        menu_assets.signal_connected = true;
    }
}

fn on_resume_game(
    _trigger: On<ResumeGameEvent>,
    state: Res<State<GameState>>,
    mut app_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    if *state.get() != GameState::PauseMenu {
        godot_print!("Press but not Pause");
        return;
    }

    godot_print!("Press start in Pause menu");

    app_state.set(GameState::InGame);
    commands.trigger(LoadSceneMessage {
        scene_state: SceneState::InGame,
    });
}

fn on_return_main_menu(
    _trigger: On<MainMenuGameEvent>,
    state: Res<State<GameState>>,
    mut app_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    if *state.get() != GameState::PauseMenu {
        godot_print!("Press but not PauseMenu");
        return;
    }

    godot_print!("Press quit in PauseMenu");

    app_state.set(GameState::MainMenu);
    commands.trigger(LoadSceneMessage {
        scene_state: SceneState::MainMenu,
    });
}

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PauseMenuAssets>()
            .add_plugins(GodotSignalsPlugin::<ResumeGameEvent>::default())
            .add_plugins(GodotSignalsPlugin::<MainMenuGameEvent>::default())
            .add_systems(OnEnter(GameState::PauseMenu), reset_menu_assets)
            .add_systems(
                Update,
                (
                    initialized_pause_menu.run_if(pause_menu_not_initialized),
                    connect_button.run_if(pause_menu_initialized_but_signals_not_connected),
                )
                    .run_if(in_state(GameState::PauseMenu)),
            )
            .add_systems(
                Update,
                toggle_pause_menu.run_if(in_state(GameState::InGame)),
            )
            .add_observer(on_resume_game)
            .add_observer(on_return_main_menu);
    }
}
