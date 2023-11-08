use bevy::prelude::*;


pub enum ProjectileGroup
{
    Players,
    Enemies,
}

#[derive(Component)]
pub struct Projectile
{
    pub owner: ProjectileGroup
}

#[derive(Component)]
pub enum ProjectileHitbox
{
    Circular(f32),
    Square(Vec2), // Contains x-half-distance, y-half-distance
    Polygon(Vec<Vec2>)
}
