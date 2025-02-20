use ::macroquad::prelude::*;

/// Création du trait "StellarObject" qui va s'occuper des fonctions "get_position", "move_object"
/// et "collision" qui sont les fonctions communes aux trois structures "Asteroid", "Bullet" et "Spaceship".
pub trait StellarObject {
    type ReturnType;
    fn get_position(&self) -> Vec2;
    fn move_object(&mut self) -> Vec2;
    fn collision<T: StellarObject>(&mut self, elem: &T, size: f32) -> Self::ReturnType;
}
