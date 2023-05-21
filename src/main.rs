use bevy::{prelude::*, DefaultPlugins, log::LogPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use kayak_ui::{prelude::*, widgets::*};


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(LogPlugin {
                level: bevy::log::Level::INFO,
                filter: "wgpu_core=error,wgpu_hal=error,naga=warn,kayak_ui=info".into(),
            }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(KayakContextPlugin)
        .add_plugin(KayakWidgets)
        .add_startup_system(startup)
        .run();
}

#[derive(States, PartialEq, Eq, Debug, Default, Clone, Hash)]
pub enum GameState {
    #[default]
    Overworld,
    Combat,
}

fn startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    let camera_entity = commands
        .spawn((Camera2dBundle::default(), CameraUIKayak))
        .id();

    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    let mut widget_context = KayakRootContext::new(camera_entity);
    widget_context.add_plugin(KayakWidgetsContextPlugin);

    widget_context.add_widget_data::<GameStateProps, EmptyState>();
    widget_context.add_widget_system(
        GameStateProps::default().get_name(),
        update_game_state::<GameStateProps, EmptyState>,
        game_state_render,
    );
    let parent_id: Option<Entity> = None;

    rsx! {
        <KayakAppBundle>
            <GameStateBundle />
        </KayakAppBundle>
    };

    commands.spawn((widget_context, EventDispatcher::default()));
}

#[derive(Component, Default, Clone, PartialEq)]
pub struct GameStateProps;

impl Widget for GameStateProps {}

#[derive(Bundle)]
pub struct GameStateBundle {
    pub name: Name,
    pub widget: GameStateProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub widget_name: WidgetName,
}

impl Default for GameStateBundle {
    fn default() -> Self {
        Self {
            name: Name::new("GameStateWidget"),
            widget_name: GameStateProps::default().get_name(),
            widget: GameStateProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles(KStyle {
                render_command: StyleProp::Value(RenderCommand::Layout),
                height: StyleProp::Value(Units::Stretch(1.0)),
                width: StyleProp::Value(Units::Stretch(1.0)),
                ..KStyle::default()
            }),
        }
    }
}

pub fn update_game_state<
    Props: PartialEq + Component + Clone,
    UIState: PartialEq + Component + Clone,
>(
    In((entity, previous_entity)): In<(Entity, Entity)>,
    game_state: Res<State<GameState>>,
    mut local_state: Local<GameState>,
    widget_context: Res<KayakWidgetContext>,
    widget_param: WidgetParam<Props, UIState>,
) -> bool {
    if widget_param.has_changed(&widget_context, entity, previous_entity) {
        return true;
    }
    let changed = game_state.0 != *local_state;
    if changed {
        *local_state = game_state.0.clone();
    }
    changed
}

pub fn game_state_render(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    game_state: Res<State<GameState>>,
) -> bool {
    let parent_id = Some(entity);

    rsx!{
        <ElementBundle>
        {
            match game_state.0 {
                GameState::Overworld => {
                    constructor!{
                        <ElementBundle>
                            <ElementBundle>
                                <TextWidgetBundle
                                    text={TextProps {
                                        content: "OverWorld".into(),
                                        size: 20.0,
                                        ..Default::default()
                                    }}
                                />
                                <KButtonBundle
                                    styles={KStyle {
                                        top: Units::Stretch(1.0).into(),
                                        bottom: Units::Stretch(1.0).into(),
                                        left: Units::Stretch(1.0).into(),
                                        right: Units::Stretch(1.0).into(),
                                        ..Default::default()
                                    }}
                                    button={KButton { text: "Enter Combat".into() }}
                                    on_event={OnEvent::new(
                                        move |In(_entity): In<Entity>,
                                        mut event: ResMut<KEvent>,
                                        mut next_state: ResMut<NextState<GameState>>| {
                                            if let EventType::Click(..) = event.event_type {
                                                event.prevent_default();
                                                event.stop_propagation();
                                                info!("Entering combat");
                                                next_state.set(GameState::Combat);
                                            }
                                        },
                                    )}
                                />
                            </ElementBundle>
                        </ElementBundle>
                    }
                },
                GameState::Combat => {
                    constructor!{
                        <ElementBundle>
                            <TextWidgetBundle
                                text={TextProps {
                                    content: "Combat".into(),
                                    size: 20.0,
                                    ..Default::default()
                                }}
                            />
                            <KButtonBundle
                                styles={KStyle {
                                    top: Units::Stretch(1.0).into(),
                                    bottom: Units::Stretch(1.0).into(),
                                    left: Units::Stretch(1.0).into(),
                                    right: Units::Stretch(1.0).into(),
                                    ..Default::default()
                                }}
                                button={KButton { text: "Leave Combat".into() }}
                                on_event={OnEvent::new(
                                    move |In(_entity): In<Entity>,
                                    mut event: ResMut<KEvent>,
                                    mut next_state: ResMut<NextState<GameState>>| {
                                        if let EventType::Click(..) = event.event_type {
                                            event.prevent_default();
                                            event.stop_propagation();
                                            info!("Exiting combat");
                                            next_state.set(GameState::Overworld);
                                        }
                                    },
                                )}
                            />
                        </ElementBundle>
                    }
                }
            }
        }
        </ElementBundle>
    };

    true
}