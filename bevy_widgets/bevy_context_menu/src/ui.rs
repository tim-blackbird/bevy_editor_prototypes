use bevy::{prelude::*, window::SystemCursorIcon, winit::cursor::CursorIcon};
use bevy_editor_styles::Theme;

use crate::{ContextMenu, ContextMenuOption};

pub(crate) fn spawn_context_menu<'a>(
    commands: &'a mut Commands,
    theme: &Theme,
    menu: &ContextMenu,
    position: Vec2,
    target: Entity,
) -> EntityCommands<'a> {
    let root = commands
        .spawn(NodeBundle {
            background_color: theme.context_menu_background_color,
            border_radius: theme.border_radius,
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(position.y),
                left: Val::Px(position.x),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(3.)),
                width: Val::Px(300.),
                ..default()
            },
            ..default()
        })
        .id();

    for option in &menu.options {
        spawn_option(commands, theme, option, target).set_parent(root);
    }

    commands.entity(root)
}

pub(crate) fn spawn_option<'a>(
    commands: &'a mut Commands,
    theme: &Theme,
    option: &ContextMenuOption,
    target: Entity,
) -> EntityCommands<'a> {
    let callback = option.f.clone();
    let root = commands
        .spawn(NodeBundle {
            border_radius: theme.button_border_radius,
            style: Style {
                padding: UiRect::all(Val::Px(5.)),
                flex_grow: 1.,
                ..default()
            },
            ..default()
        })
        .observe(
            |trigger: Trigger<Pointer<Over>>,
             theme: Res<Theme>,
             mut query: Query<&mut BackgroundColor>| {
                *query.get_mut(trigger.entity()).unwrap() =
                    theme.context_menu_button_hover_background_color;
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Out>>, mut query: Query<&mut BackgroundColor>| {
                query.get_mut(trigger.entity()).unwrap().0 = Color::NONE;
            },
        )
        .observe(
            move |_trigger: Trigger<Pointer<Over>>,
                  window_query: Query<Entity, With<Window>>,
                  mut commands: Commands| {
                let window = window_query.single();
                commands
                    .entity(window)
                    .insert(CursorIcon::System(SystemCursorIcon::Pointer));
            },
        )
        .observe(
            |_trigger: Trigger<Pointer<Out>>,
             window_query: Query<Entity, With<Window>>,
             mut commands: Commands| {
                let window = window_query.single();
                commands
                    .entity(window)
                    .insert(CursorIcon::System(SystemCursorIcon::Default));
            },
        )
        .observe(
            move |trigger: Trigger<Pointer<Up>>,
                  mut commands: Commands,
                  parent_query: Query<&Parent>| {
                // Despawn context menu
                let root = parent_query
                    .iter_ancestors(trigger.entity())
                    .last()
                    .unwrap();
                commands.entity(root).despawn_recursive();
                callback.lock().unwrap()(commands.reborrow(), target);
            },
        )
        .id();

    commands
        .spawn((
            Text::new(&option.label),
            TextFont {
                font_size: 12.,
                ..default()
            },
            PickingBehavior::IGNORE,
        ))
        .set_parent(root);

    commands.entity(root)
}
