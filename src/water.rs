use crate::prelude::*;

#[derive(Component, Default, Copy, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct WaterState {
    pub level: WaterLevel,
    pub speed: f32,
}

#[derive(Default, Copy, Reflect, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum WaterLevel {
    #[default]
    None,
    Touching,
    Center,
}

#[derive(Reflect, Component, Default)]
#[require(Sensor, Transform, GlobalTransform)]
#[reflect(Component)]
pub struct Water {
    pub speed: f32,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        update_water.before(AhoySystems::MoveCharacters),
    );
}

fn update_water(
    mut objects: Query<(Entity, &Position, &mut WaterState)>,
    waters: Query<(&Collider, &Position, &Rotation, &Water)>,
    collisions: Collisions,
) {
    for (object, object_position, mut water_state) in &mut objects {
        water_state.level = WaterLevel::None;
        water_state.speed = f32::MAX;
        let waist = **object_position;
        for contact_pair in collisions.collisions_with(object) {
            if let Ok((collider, position, rotation, water)) = waters
                .get(contact_pair.collider1)
                .or(waters.get(contact_pair.collider2))
            {
                water_state.speed = water_state.speed.min(water.speed);
                let level = if collider.contains_point(*position, *rotation, waist) {
                    WaterLevel::Center
                } else {
                    WaterLevel::Touching
                };
                if level > water_state.level {
                    water_state.level = level;
                }
            }
        }
    }
}
