use bevy::{input::keyboard::KeyboardInput, prelude::*};

use super::{
    despawn_entities_with_component, BASE_COLOR, FONT_SIZE, HIGH_COLOR, MENU_BUTTON_MARGIN,
    TEXT_COLOR,
};

pub fn settings_plugin(app: &mut App) {
    app.init_state::<SettingsState>()
        .add_systems(OnEnter(SettingsState::Main), setup_main_settings)
        .add_systems(
            OnExit(SettingsState::Main),
            despawn_entities_with_component::<OnMainSettingsScreen>,
        )
        .add_systems(
            Update,
            (
                escape_to_main_settings.run_if(in_state(SettingsState::Disabled)),
                exit_settings.run_if(not(in_state(SettingsState::Disabled))),
            ),
        );
}

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum SettingsState {
    Main,
    SettingsAudio,
    SettingsGraphics,
    #[default]
    Disabled,
}

#[derive(Component)]
struct OnMainSettingsScreen;

fn setup_main_settings(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            OnMainSettingsScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    OnMainSettingsScreen,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Main Settings"),
                        TextFont {
                            font_size: FONT_SIZE,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));
                });
        });
}

// When escape is pressed switch to main settings
fn escape_to_main_settings(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut settings_state: ResMut<NextState<SettingsState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        settings_state.set(SettingsState::Main);
    }
}

// When escpae is pressed exit settings
fn exit_settings(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut settings_state: ResMut<NextState<SettingsState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        settings_state.set(SettingsState::Disabled);
    }
}
