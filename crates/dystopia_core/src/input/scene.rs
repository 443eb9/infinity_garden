use avian2d::prelude::{Collider, Position, Rotation};
use bevy::{
    math::Vec2,
    prelude::{
        Camera, Commands, Component, Entity, EventReader, GlobalTransform, ParallelCommands, Query,
        Res, With, Without,
    },
};

use crate::{
    input::{MouseHovering, MouseInput, RayTransparent, SceneCursorPosition, SceneMouseInput},
    simulation::MainCamera,
};

#[derive(Component)]
pub struct EntityOnDrag {
    pub initial_cursor_pos: Vec2,
    pub initial_elem_world_pos: Vec2,
}

pub fn scene_mouse_hover(
    commands: ParallelCommands,
    cursor_pos: Res<SceneCursorPosition>,
    colliders_query: Query<(Entity, &Collider, &Position, &Rotation), Without<RayTransparent>>,
    main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, transform) = main_camera.single();
    let Some(cursor_pos) = (**cursor_pos).and_then(|p| camera.viewport_to_world_2d(transform, p))
    else {
        return;
    };

    colliders_query
        .par_iter()
        .for_each(|(entity, collider, position, rotation)| {
            if collider.contains_point(*position, *rotation, cursor_pos) {
                commands.command_scope(|mut c| {
                    c.entity(entity).insert(MouseHovering);
                });
            }
        });
}

pub fn scene_mouse_click(
    mut commands: Commands,
    colliders_query: Query<Entity, With<MouseHovering>>,
    mut event: EventReader<SceneMouseInput>,
) {
    for click in event.read() {
        colliders_query.iter().for_each(|entity| {
            commands.entity(entity).insert(MouseInput {
                button: click.button,
                state: click.state,
            });
        });
    }
}
