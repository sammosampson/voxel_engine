use crate::prelude::*;

#[derive(Debug)]
pub struct Mesh {
    pub data: Vec<Vertex>,
}


impl Mesh {
    pub fn new(data: Vec<Vertex>) -> Self {
        Self {
            data
        }
    }
}
