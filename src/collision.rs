use bevy::{math::bounding::Aabb2d, prelude::*};

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Collision>();
        app.add_systems(FixedUpdate, check_for_collisions.in_set(CollisionSystems));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CollisionSystems;

pub(super) struct CollisionPlugin;

#[derive(Clone, Copy)]
pub(super) enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

#[derive(Clone)]
pub(super) struct Contact {
    pub(super) normal: Vec2,
    pub(super) penetration: f32,
}

impl Contact {
    fn flipped(self) -> Self {
        Self {
            normal: -self.normal,
            ..self
        }
    }
}

#[derive(Clone)]
pub(super) struct CollisionMember {
    pub(super) entity: Entity,
    pub(super) position: Vec2,
    pub(super) shape: Shape,
}

#[derive(Component)]
pub(super) struct Collider {
    pub(super) shape: Shape,
}

#[derive(Message)]
pub(super) struct Collision {
    pub(super) contact: Contact,
    pub(super) collisioner: CollisionMember,
    pub(super) collisionee: CollisionMember,
}

fn check_for_collisions(
    query: Query<(Entity, &Transform, &Collider)>,
    mut collisions: MessageWriter<Collision>,
) {
    for [
        (entity1, transform1, collider1),
        (entity2, transform2, collider2),
    ] in query.iter_combinations()
    {
        if let Some(contact) = collide(
            transform1.translation.xy(),
            transform2.translation.xy(),
            collider1,
            collider2,
        ) {
            let contact2 = contact.clone().flipped();
            let collision_member1 = CollisionMember {
                entity: entity1,
                position: transform1.translation.xy(),
                shape: collider1.shape,
            };
            let collision_member2 = CollisionMember {
                entity: entity2,
                position: transform2.translation.xy(),
                shape: collider2.shape,
            };

            collisions.write(Collision {
                contact,
                collisioner: collision_member1.clone(),
                collisionee: collision_member2.clone(),
            });
            collisions.write(Collision {
                contact: contact2,
                collisioner: collision_member2,
                collisionee: collision_member1,
            });
        }
    }
}

fn collide(pos1: Vec2, pos2: Vec2, collider1: &Collider, collider2: &Collider) -> Option<Contact> {
    match (&collider1.shape, &collider2.shape) {
        (Shape::Circle(circle1), Shape::Circle(circle2)) => {
            circle_circle_collide(pos1, pos2, circle1, circle2)
        }
        (Shape::Circle(circle), Shape::Rectangle(rectangle)) => {
            circle_rectangle_collide(pos1, pos2, circle, rectangle)
        }
        (Shape::Rectangle(_), Shape::Circle(_)) => {
            collide(pos2, pos1, collider2, collider1).map(Contact::flipped)
        }
        (Shape::Rectangle(rectangle1), Shape::Rectangle(rectangle2)) => {
            rectangle_rectangle_collide(pos1, pos2, rectangle1, rectangle2)
        }
    }
}

fn circle_rectangle_collide(
    pos1: Vec2,
    pos2: Vec2,
    circle: &Circle,
    rectangle: &Rectangle,
) -> Option<Contact> {
    let bounding_rectangle = Aabb2d::new(pos2, rectangle.half_size);
    let closest_point = bounding_rectangle.closest_point(pos1);
    let offset = pos1 - closest_point;
    let distance = closest_point.distance(pos1);
    if distance > circle.radius {
        return None;
    }

    if distance > 0. {
        return Some(Contact {
            normal: offset.normalize(),
            penetration: circle.radius - distance,
        });
    }

    let delta = pos1 - pos2;
    let overlap = rectangle.half_size - delta.abs();
    if overlap.x < overlap.y {
        Some(Contact {
            normal: Vec2::new(delta.x.signum(), 0.),
            penetration: overlap.x + circle.radius,
        })
    } else {
        Some(Contact {
            normal: Vec2::new(0., delta.y.signum()),
            penetration: overlap.y + circle.radius,
        })
    }
}

fn rectangle_rectangle_collide(
    pos1: Vec2,
    pos2: Vec2,
    rectangle1: &Rectangle,
    rectangle2: &Rectangle,
) -> Option<Contact> {
    let delta = pos1 - pos2;
    let overlap = (rectangle1.half_size + rectangle2.half_size) - delta.abs();

    if overlap.x <= 0. || overlap.y <= 0. {
        return None;
    }

    if overlap.x < overlap.y {
        Some(Contact {
            normal: Vec2::new(delta.x.signum(), 0.),
            penetration: overlap.x,
        })
    } else {
        Some(Contact {
            normal: Vec2::new(0., delta.y.signum()),
            penetration: overlap.y,
        })
    }
}

fn circle_circle_collide(
    pos1: Vec2,
    pos2: Vec2,
    circle1: &Circle,
    circle2: &Circle,
) -> Option<Contact> {
    if pos1.distance(pos2) > circle1.radius + circle2.radius {
        return None;
    }
    Some(Contact {
        normal: pos1 - pos2,
        penetration: circle1.radius + circle2.radius - pos1.distance(pos2),
    })
}
