use crate::{
    system, system_with_extra_data,
    update::{
        PhysicsData,
        components::{Rigidbody, Transform},
        ecs::System,
    },
};
use alexandria::math::Vector3f;

impl Rigidbody {
    /// Create a system that moves rigidbodies based on their forces
    pub(in crate::update) fn system() -> System<PhysicsData> {
        let (type_ids, system) =
            system_with_extra_data!(|physics_data: PhysicsData,
                                     transforms: Transform,
                                     rigidbodies: Rigidbody| {
                let gravity = physics_data.gravity() * physics_data.delta_time();

                for (transform, rigidbody) in transforms.iter_mut().zip(rigidbodies.iter_mut()) {
                    let mut forces = rigidbody.forces * physics_data.delta_time();
                    if rigidbody.apply_gravity {
                        forces += gravity;
                    }

                    let acceleration = forces / rigidbody.mass;
                    rigidbody.velocity += acceleration;

                    *transform.position_mut() += rigidbody.velocity * physics_data.delta_time();
                }
            });
        System::new(type_ids, system)
    }

    /// Create a system that zeroes rigidbody forces each frame
    pub(in crate::update) fn zero_system() -> System<()> {
        let (type_ids, system) = system!(|rigidbodies: Rigidbody| {
            for rigidbody in rigidbodies.iter_mut() {
                rigidbody.forces = Vector3f::ZERO;
            }
        });
        System::new(type_ids, system)
    }
}
