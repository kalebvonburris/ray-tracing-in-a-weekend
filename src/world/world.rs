use std::sync::Arc;

use crate::objects::hittable::Hittable;

/// The world state of the application.
pub struct World {
    /// All objects in the world.
    pub objects: Arc<Vec<Box<dyn Hittable>>>,
}

impl World {
    #[must_use]
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self {
            objects: Arc::new(objects),
        }
    }
}

unsafe impl Send for World {}
unsafe impl Sync for World {}
