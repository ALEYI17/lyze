use bevy::{ecs::resource::Resource, prelude::*};
use godot::classes::Button;
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::state::GameState;
use crate::state::state_manager::{LoadSceneMessage, SceneState};

#[derive(Resource, Default)]
pub struct MenuAssets {
    pub start_button: Option<GodotNodeHandle>,
    pub quit_button: Option<GodotNodeHandle>,
    pub initialized: bool,
    pub signal_connected: bool,
}

#[derive(NodeTreeView)]
pub struct MainMenuUi {
    // Todo
    #[node("/root/Main_menu/Button_manager/Start")]
    pub start_button: GodotNodeHandle,

    // Todo
    #[node("/root/Main_menu/Button_manager/Exit")]
    pub quit_button: GodotNodeHandle,
}

fn reset_menu_assets(mut menu_assets: ResMut<MenuAssets>) {
    menu_assets.start_button = None;
    menu_assets.quit_button = None;
    menu_assets.initialized = false;
    menu_assets.signal_connected = false;
}

fn initialized_main_menu(mut menu_assets: ResMut<MenuAssets>, mut scene_tree: SceneTreeRef) {
    if let Some(root) = scene_tree.get().get_root() {
        match MainMenuUi::from_node(root) {
            Ok(menu_ui) => {
                godot_print!("Found menu node");
                menu_assets.start_button = Some(menu_ui.start_button);
                menu_assets.quit_button = Some(menu_ui.quit_button);
                menu_assets.initialized = true;
            }
            Err(_) => {}
        }
    } else {
        godot_print!("Main Menu scene not avaible");
    }
}

fn menu_not_initialized(menu_assets: Res<MenuAssets>) -> bool {
    !menu_assets.initialized
}

fn menu_initialized_but_signals_not_connected(menu_assets: Res<MenuAssets>) -> bool {
    menu_assets.initialized && !menu_assets.signal_connected
}

#[derive(Event, Debug, Clone)]
struct StartGameEvent;

#[derive(Event, Debug, Clone)]
struct QuitGameEvent {
    source: GodotNodeHandle,
}

fn connect_button(
    mut menu_assets: ResMut<MenuAssets>,
    signals_start: GodotSignals<StartGameEvent>,
    signals_quit: GodotSignals<QuitGameEvent>,
) {
    if menu_assets.start_button.is_some()
        && menu_assets.quit_button.is_some()
        && !menu_assets.signal_connected
    {
        if let Some(start_handle) = menu_assets.start_button {
            godot_print!("Connect start");
            signals_start.connect(
                start_handle,
                BaseButtonSignals::PRESSED,
                None,
                |_args, _node_handle, _ent| Some(StartGameEvent),
            );
        }
        if let Some(quit_handle) = menu_assets.quit_button {
            godot_print!("Connect quit");
            signals_quit.connect(
                quit_handle,
                BaseButtonSignals::PRESSED,
                None,
                |_args, node_handle, _ent| {
                    Some(QuitGameEvent {
                        source: node_handle,
                    })
                },
            );
        }
        menu_assets.signal_connected = true;
    }
}

fn on_start_game(
    _trigger: On<StartGameEvent>,
    state: Res<State<GameState>>,
    mut app_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    if *state.get() != GameState::MainMenu {
        godot_print!("Press but not MainMenu");
        return;
    }

    godot_print!("Press start in MainMenu");

    app_state.set(GameState::InGame);
    commands.trigger(LoadSceneMessage {
        scene_state: SceneState::InGame,
    });
}

fn on_quit(trigger: On<QuitGameEvent>, state: Res<State<GameState>>, mut godot: GodotAccess) {
    if *state.get() != GameState::MainMenu {
        godot_print!("Press but not MainMenu");
        return;
    }

    godot_print!("Press quit in MainMenu");
    if let Some(button) = godot.try_get::<Button>(trigger.event().source)
        && let Some(mut tree) = button.get_tree()
    {
        tree.quit();
    }
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuAssets>()
            .add_plugins(GodotSignalsPlugin::<StartGameEvent>::default())
            .add_plugins(GodotSignalsPlugin::<QuitGameEvent>::default())
            .add_systems(OnEnter(GameState::MainMenu), reset_menu_assets)
            .add_systems(
                Update,
                (
                    initialized_main_menu.run_if(menu_not_initialized),
                    connect_button.run_if(menu_initialized_but_signals_not_connected),
                )
                    .run_if(in_state(GameState::MainMenu)),
            )
            .add_observer(on_start_game)
            .add_observer(on_quit);
    }
}
