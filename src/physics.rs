use crate::math;

#[derive(Default)]
pub struct Velocity {
    pub value: f32
}

impl Velocity {
    pub fn increase_by(&mut self, to_increase_by: f32) {
        self.value = self.value + to_increase_by;
    }
}

#[derive(Default)]
pub struct Heading {
    pub value: math::Vector4
}

impl Heading {
    pub fn rotate_y_by(&mut self, to_rotate_by: f32) {
        self.value = math::Matrix4x4::y_rotation(to_rotate_by) * self.value;
    }
}

impl From<math::Vector4> for Heading {
    fn from(value: math::Vector4) -> Self {
        Heading { value }
    }
}