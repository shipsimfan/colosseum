use crate::physics::RaycastResult;

impl RaycastResult {
    /// Compare two raycast results based on their distance
    pub fn compare(this: Option<Self>, other: Option<Self>) -> Option<Self> {
        match (this, other) {
            (Some(a), Some(b)) => {
                if a.distance < b.distance {
                    Some(a)
                } else {
                    Some(b)
                }
            }
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }
}
