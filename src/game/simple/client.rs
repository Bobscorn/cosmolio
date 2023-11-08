use bevy::prelude::*;
use bevy_replicon::prelude::*;

use crate::game::simple::{
    plugin::*,
    common::*
};


#[derive(Component)]
pub struct LocalPlayer;

#[derive(Bundle)]
pub struct PlayerClientBundle
{
    sprite_bundle: SpriteBundle
}

impl PlayerClientBundle
{
    pub fn new(color: Color, position: Vec2) -> Self
    {
        Self
        {
            sprite_bundle: SpriteBundle { sprite: Sprite { color, custom_size: Some(Vec2::new(25.0, 25.0)), ..default() }, transform: Transform::from_translation(position.extend(0.0)), ..default() }
        }
    }
}


pub fn movement_input_system(mut move_events: EventWriter<MoveDirection>, input: Res<Input<KeyCode>>)
{
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::D)
    {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::A)
    {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::W)
    {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::S)
    {
        direction.y -= 1.0;
    }
    if direction != Vec2::ZERO
    {
        move_events.send(MoveDirection(direction.normalize_or_zero()));
    }
}

pub fn client_movement_predict(
    mut move_events: EventReader<MoveDirection>, 
    mut players: Query<&mut Position, With<LocalPlayer>>,
    time: Res<Time>
) {
    for dir in &mut move_events
    {
        for mut player_pos in &mut players
        {
            player_pos.0 += dir.0 * time.delta_seconds() * MOVE_SPEED;
        }
    }
}

// Adds other non-replicated components to a Player entity when it has been replicated
pub fn client_player_spawn_system(
    mut commands: Commands, 
    query: Query<(Entity, &Player, &Position, &PlayerColor), Added<Replication>>,
    mut local_player: ResMut<LocalPlayerId>
) {
    for (entity, player, pos, color) in &query
    {
        let mut coms = commands.entity(entity);
        coms.insert(PlayerClientBundle::new(color.0, pos.0));
        let player_id = player.0;
        if player_id != local_player.id
        {
            continue;
        }
        
        info!("Inserting Local Player '{player_id}'");
        local_player.entity = coms.insert(LocalPlayer).id();
    }
}
