use bevy::{prelude::*, utils::HashMap};

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entites: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entites: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entites: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // first phase detection -- detect collisions
    for (entity_alpha, transform_alpha, collider_alpha) in query.iter() {
        for (entity_bravo, transform_bravo, collider_bravo) in query.iter() {
            if entity_alpha != entity_bravo {
                let distance = transform_alpha
                    .translation()
                    .distance(transform_bravo.translation());

                if distance < collider_alpha.radius + collider_bravo.radius {
                    colliding_entites
                        .entry(entity_alpha)
                        .or_insert_with(Vec::new)
                        .push(entity_bravo);
                }
            }
        }
    }

    // second phase -- update colliders
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entites.clear();
        if let Some(collisions) = colliding_entites.get(&entity) {
            collider
                .colliding_entites
                .extend(collisions.iter().copied());
        }
    }
}
