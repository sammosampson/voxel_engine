
use crate::math;


#[derive(Debug)]
pub enum EditorEvent {
    SetWindowVisibility(bool, String),
    CameraPositionChanged(math::Vector4),
    CameraDirectionChanged(math::Vector4),
    CameraUpChanged(math::Vector4)
}