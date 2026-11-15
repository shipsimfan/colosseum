use crate::physics::{Collider, Ray, RaycastResult};
use alexandria::math::Vector3f;

impl Collider {
    /// Perform a raycast against the collider
    pub fn raycast(&self, ray: &Ray) -> Option<RaycastResult> {
        match self {
            &Collider::AABB { min, size } => raycast_aabb(ray, min, size),
        }
    }
}

/// Perform a raycast against an axis-aligned bounding box (AABB)
fn raycast_aabb(ray: &Ray, min: Vector3f, size: Vector3f) -> Option<RaycastResult> {
    let max = min + size;
    let (min, max) = (min.min_v(max), min.max_v(max));

    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;

    for (origin, direction, box_min, box_max) in [
        (ray.origin.x, ray.direction.x, min.x, max.x),
        (ray.origin.y, ray.direction.y, min.y, max.y),
        (ray.origin.z, ray.direction.z, min.z, max.z),
    ] {
        if direction.abs() < 1e-8 {
            if origin < box_min || origin > box_max {
                return None;
            }
        } else {
            let t1 = (box_min - origin) / direction;
            let t2 = (box_max - origin) / direction;
            let (t_near, t_far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };

            t_min = t_min.max(t_near);
            t_max = t_max.min(t_far);

            if t_min > t_max {
                return None;
            }
        }
    }

    if t_max < 0.0 {
        return None;
    }

    let t = if t_min >= 0.0 { t_min } else { t_max };
    Some(RaycastResult {
        position: ray.origin + ray.direction * t,
        distance: t,
    })
}
