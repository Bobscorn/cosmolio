mod bullet;
mod explosion;
mod missile;
mod laser;
mod melee;

pub use bullet::{Bullet, BulletReplicationBundle};
pub use explosion::{Explosion, ExplosionReplicationBundle};
pub use missile::{Missile, MissileReplicationBundle};
pub use laser::{Laser, LaserReplicationBundle};
pub use melee::{MeleeAttack, MeleeAttackData, MeleeAttackType, MeleeReplicationBundle};

use bevy::prelude::*;
use bevy_replicon::prelude::*;

use crate::simple::state;

pub struct GameplayObjectPlugin;

impl Plugin for GameplayObjectPlugin
{
    fn build(&self, app: &mut App) {
        app
            .replicate::<Bullet>()
            .replicate::<Missile>()
            .replicate::<Explosion>()
            .replicate::<MeleeAttack>()
            .add_systems(FixedUpdate, (
                bullet::s_bullet_authority,
                missile::s_missile_authority,
                melee::s_melee_authority,
                explosion::s_explosion_authority,
                laser::s_laser_authority,
            ).in_set(state::AuthoritySystems))
            .add_systems(FixedUpdate, (
                missile::s_move_missiles,
            ).in_set(state::AuthoritySystems).in_set(state::FightingSystems))
            .add_systems(FixedUpdate, (
                bullet::c_bullet_extras,
                melee::c_melee_extras,
                laser::c_laser_extras,
                explosion::c_explosion_extras,
                missile::c_missile_extras,
            ).in_set(state::HostAndClientSystems))
            ;
    }
}
