use std::ops::{Deref, DerefMut};

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Color(pub u8);

impl Deref for Color {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub type Pattern = Vec<Color>;
