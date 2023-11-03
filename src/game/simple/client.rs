use bevy::prelude::*;
use bevy_replicon::prelude::*;

use crate::game::simple::plugin::*;


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
    if input.pressed(KeyCode::Right)
    {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::Left)
    {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::Up)
    {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::Down)
    {
        direction.y -= 1.0;
    }
    if direction != Vec2::ZERO
    {
        move_events.send(MoveDirection(direction.normalize_or_zero()));
    }
}

pub fn ability_input_system(mut ability_events: EventWriter<AbilityActivation>, input: Res<Input<KeyCode>>)
{
    
}

pub fn client_movement_predict(
    mut move_events: EventReader<MoveDirection>, 
    mut players: Query<&mut PlayerPosition, With<LocalPlayer>>,
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
    query: Query<(Entity, &Player, &PlayerPosition, &PlayerColor), Added<Replication>>,
    local_player: Res<LocalPlayerId>
) {
    for (entity, player, pos, color) in &query
    {
        let mut coms = commands.entity(entity);
        coms.insert(PlayerClientBundle::new(color.0, pos.0));
        if player.0 != local_player.0
        {
            continue;
        }
        
        coms.insert(LocalPlayer);
    }
}
