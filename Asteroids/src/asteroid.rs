use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::stellarobject::StellarObject;

/// Création de la structure "Asteroid" avec en valeur une "position" et une "speed" représenté
/// par des vecteur de taille 2.
pub struct Asteroid {
    pub position: Vec2,
    pub speed: Vec2,
}

/// Implémentation des méthodes pour la structure "Asteroid".
impl Asteroid {
    /// Taille de base de chaque objet de la stucture "Asteroid".
    pub const ASTEROID_INIT_SIZE: f32 = 60.0;

    /// Méthode qui permet de créer un nouvel objet de la structure "Asteroid" avec une
    /// "position" est une "speed"  aléatoire.
    pub fn new() -> Self {
        Self {
            position: Self::new_alea_pos(),
            speed: Self::new_alea_speed(),
        }
    }

    /// Génère une position aléatoire près de l'un des bords de la fenêtre.
    pub fn new_alea_pos() -> Vec2 {
        let mut rng = thread_rng();

        let nearpos: f32 = rng.gen_range(Self::ASTEROID_INIT_SIZE / 2.0..=Self::ASTEROID_INIT_SIZE);
        let nearside = rng.gen_range(1..=4); // 1 = top, 2 = right, 3 = down, 4 = left
        let xpos: f32 = match nearside {
            2 => screen_width() - nearpos,
            4 => nearpos,
            _ => rng.gen_range(0.0..=screen_width()),
        };
        let ypos: f32 = match nearside {
            1 => nearpos,
            3 => screen_height() - nearpos,
            _ => rng.gen_range(0.0..=screen_height()),
        };
        vec2(xpos, ypos)
    }

    /// Génère une "speed" aléatoire.
    pub fn new_alea_speed() -> Vec2 {
        let mut rng = thread_rng();

        let angle: f32 = rng.gen_range(0.0..=(2.0 * PI));
        Vec2::from_angle(angle)
    }

    /// Vérifie que l'objet de la structure "Asteroid" se trouve bien dans la fenêtre sinon le replace
    /// de l'autre coté de cette même fenêtre.
    pub fn bound_pos(mut pos: Vec2) -> Vec2 {
        pos.x = Self::bound_to(pos.x, screen_width());
        pos.y = Self::bound_to(pos.y, screen_height());
        pos
    }

    /// Vérifie qu'une coordonnée se situe bien dans la partie visible de la fenêtre.
    pub fn bound_to(coord: f32, max: f32) -> f32 {
        if coord < 0.0 {
            max - coord
        } else if coord > max {
            coord - max
        } else {
            coord
        }
    }
}

/// Ajout du trait "StellarObject" pour la structure "Asteroid".
impl StellarObject for Asteroid {
    /// Initialisation du type que la méthode collision va renvoyer.
    type ReturnType = Option<(Vec<Vec2>, Vec<Vec2>)>;

    /// Accesseur qui renvoie la "position" d'un objet de la structure "Asteroid".
    fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Permet de faire bouger un objet de la structrue "Asteroid".
    fn move_object(&mut self) -> Vec2 {
        self.position += self.speed;
        self.position = Self::bound_pos(self.position);
        self.position
    }

    /// Vérifie si l'objet de la structure "Asteroid" est en collision avec un objet de la structure "Bullet".
    fn collision<T: StellarObject>(&mut self, elem: &T, size: f32) -> Self::ReturnType {
        let mut temp_aster: Vec<Vec2> = Vec::new();
        let mut temp_bullet: Vec<Vec2> = Vec::new();
        if (elem.get_position() - self.get_position()).length() <= size {
            temp_aster.push(self.get_position());
            temp_bullet.push(elem.get_position());
        }
        Some((temp_aster, temp_bullet))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bullet::Bullet;
    #[test]
    fn check_get_position() {
        let asteroid = Asteroid {
            position: vec2(90.0, 60.0),
            speed: vec2(2.0, 2.0),
        };
        assert_eq!(asteroid.get_position(), vec2(90.0, 60.0))
    }
    #[test]
    fn check_collision() {
        let mut asteroid = Asteroid {
            position: vec2(90.0, 60.0),
            speed: vec2(2.0, 2.0),
        };
        let bullet = Bullet {
            position: vec2(60.0, 50.0),
            speed: 8.0,
            angle: 20,
        };
        let vec1 = vec![vec2(90.0, 60.0)];
        let vec2 = vec![vec2(60.0, 50.0)];
        assert_eq!(asteroid.collision(&bullet, 65.0), Some((vec1, vec2)))
    }
}
