use bevy::{prelude::*, sprite::Anchor};

use crate::simple::common::Health;


/// A tag component that will add a healthbar above an entity with health
///
/// Add this component to an entity with a health to give it a health bar, 
/// the c_add_healthbars system will handle the rest
#[derive(Component)]
pub struct HealthBar
{
    pub height: f32 // Height above the entity to place the health bar
}

impl Default for HealthBar
{
    fn default() -> Self {
        Self { height: 30.0 }
    }
}

// A tag component for the sprite entity that changes as the health changes
// Do not add yourself, used for implementing the HealthBar tag component
#[derive(Component)]
pub struct HealthBarEntity
{
    pub parent: Entity
}

fn new_health_bar(commands: &mut Commands, health_bar: &HealthBar, parent: Entity) -> Entity
{
    let background_entity = commands.spawn(NodeBundle {
        // sprite: Sprite { color: Color::GRAY, custom_size: Some(Vec2::new(29.0, 14.5)), ..default() },
        transform: Transform::from_translation(Vec3::new(0.0, health_bar.height, 1.0)),
        ..default()
    }).with_children(|c| {
        c.spawn((SpriteBundle{
            sprite: Sprite { color: Color::GREEN, custom_size: Some(Vec2::new(25.0, 9.0)), anchor: Anchor::CenterLeft, ..default() },
            transform: Transform::from_translation(Vec3::new(-12.5, 0.0, 1.0)),
            ..default()
        }, HealthBarEntity { parent }));
    }).id();
    background_entity
}

pub fn c_add_healthbars(
    mut commands: Commands,
    new_health_entities: Query<(Entity, &HealthBar), (With<Health>, Added<HealthBar>)>,
    new_health_bad_entities: Query<Entity, (Without<Health>, Added<HealthBar>)>
) {
    for (entity, health_bar) in &new_health_entities
    {
        info!("Adding a health bar entity to {:?}", entity);
        let health_bar_entity = new_health_bar(&mut commands, health_bar, entity);
        commands.entity(entity).add_child(health_bar_entity);
    }
    for entity in &new_health_bad_entities
    {
        warn!("Entity {:?} was given a HealthBar tag but does not have a Health component!", entity);
    }
}

pub fn c_update_healthbars(
    parent_healths: Query<&Health, With<HealthBar>>,
    mut healthbars: Query<(&mut Sprite, &HealthBarEntity), Without<HealthBar>>,
) {
    for (mut sprite, health_bar) in &mut healthbars
    {
        let Ok(parent_health) = parent_healths.get(health_bar.parent) else { continue; };
        let percent_health;
        if parent_health.max_health <= 0.0
        {
            percent_health = 0.0;
        } 
        else
        {
            percent_health = parent_health.health / parent_health.max_health;
        }
        sprite.custom_size = Some(Vec2::new(25.0 * percent_health, 9.0));
    }
}
