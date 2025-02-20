use ::macroquad::prelude::*;
use std::f32::consts::PI;

use crate::stellarobject::StellarObject;

/// Création de la structure "Bullet" avec en valeur une "position" représenté par un vecteur de taille 2,
/// une "speed" représenté par un f32 et un "angle" par des i32.
pub struct Bullet {
    pub position: Vec2,
    pub speed: f32,
    pub angle: i32,
}

/// Implémentation des méthodes pour la structure "Bullet".
impl Bullet {
    /// Méthode qui permet d'initialisé un nouvel objet de la structure "Bullet" en mettant la
    /// "position" à "position_ship" qui est la position d'un objet de la structure "Spaceship", la "speed" à 8.0
    /// et l'"angle" à "angle_ship" qui est l'"angle" du même objet de la structure "Spaceship".
    pub fn new(position_ship: Vec2, angle_ship: i32) -> Self {
        Self {
            position: position_ship,
            speed: 8.0,
            angle: angle_ship,
        }
    }
}

/// Ajout du trait "StellarObject" pour la structure "Bullet".
impl StellarObject for Bullet {
    /// Initialisation du type que la méthode collision va renvoyer.
    type ReturnType = ();

    /// Accesseur qui renvoie la "position" d'un objet de la structure "Bullet".
    fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Permet de faire bouger un objet de la structrue "Bullet".
    fn move_object(&mut self) -> macroquad::math::Vec2 {
        let angle_rad = self.angle as f32 * (PI / 180.0);
        let new_x = self.position.x + angle_rad.cos() * self.speed;
        let new_y = self.position.y + angle_rad.sin() * self.speed;
        self.position = vec2(new_x, new_y);
        self.position
    }

    /// La collison avec les objets de la structure "Bullet" a déjà été implémenté dans le module "asteroid.rs".
    fn collision<T: StellarObject>(&mut self, _elem: &T, _size: f32) -> Self::ReturnType {}
}

mod tests {
    use super::*;
    #[test]
    fn check_get_position() {
        let bullet = Bullet {
            position: vec2(60.0, 50.0),
            speed: 8.0,
            angle: 20,
        };
        assert_eq!(bullet.get_position(), vec2(60.0, 50.0))
    }
}
