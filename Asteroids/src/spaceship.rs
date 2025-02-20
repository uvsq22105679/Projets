use crate::stellarobject::StellarObject;
use ::macroquad::prelude::*;
use std::f32::consts::PI;

/// Création de la structure "Spaceship" avec en valeur une "position" et une "speed" représentés par des
/// vecteurs de taille 2, un "angle" et un "shield" représentés par des i32 et une "collision" qui est un
/// booleen : "False" quand l'objet de la structure "Spaceship" n'est pas en collions avec des objets de la
/// structure "Asteroid", "True" sinon.
pub struct Spaceship {
    pub position: Vec2,
    pub speed: Vec2,
    pub angle: i32,
    pub shield: i32,
    pub collision: bool,
}
/// Implémentation des méthodes pour la structure "Spaceship".
impl Spaceship {
    /// Taille de base de chaque objet de la structure "Spaceship".
    pub const SPACESHIP_INIT_SIZE: f32 = 28.0;

    /// Méthode qui permet d'initialisé un nouvel objet de la structure "Spaceship" en mettant la
    /// "position" au centre de l'écran, la "speed" à (0.0,0.0), l'angle à 0, le "shield" à 3 et la "collision"
    /// à False.
    pub fn new() -> Self {
        Self {
            position: vec2(screen_width() / 2.0, screen_height() / 2.0),
            speed: vec2(0.0, 0.0),
            angle: 0,
            shield: 3,
            collision: false,
        }
    }

    /// Accesseur qui permet de récupérer l'"angle" de l'objet de la structure "Spaceship".
    pub fn get_angle(&self) -> i32 {
        self.angle
    }

    /// Modifie la "position" de l'objet de la structure "Spaceship".
    pub fn go_to(&mut self) -> Vec2 {
        self.position += self.speed;
        self.position = Self::bound_pos(self.position);
        self.position
    }

    /// Vérifie que l'objet de la structure "Spaceship" se trouve bien dans la fenêtre sinon le replace
    /// de l'autre coté de cette  même fenêtre.
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

/// Ajout du trait "StellarObject" pour la structure "Spaceship".
impl StellarObject for Spaceship {
    /// Initialisation du type que la méthode collision va renvoyer.
    type ReturnType = bool;

    /// Accesseur qui renvoie la "position" d'un objet de la structure "Spaceship".
    fn get_position(&self) -> Vec2 {
        self.position
    }

    /// Permet de modifier certains paramètres de l'objet de la structure "Spachship" en fonction de la touche appuyée.
    fn move_object(&mut self) -> Vec2 {
        let norm = (self.speed.x * self.speed.x + self.speed.y * self.speed.y).sqrt();

        // Si c'est la touche "right", alors modifie l'"angle".
        if is_key_down(KeyCode::Right) {
            self.angle -= 3;
        }

        // Si c'est la touche "left", alors modifie l'"angle".
        if is_key_down(KeyCode::Left) {
            self.angle += 3;
        }

        // Si c'est la touche "up", alors modifie la "speed" et la "position".
        if is_key_down(KeyCode::Up) {
            let new_x = 0.1 * (self.angle as f32 * PI / 180.0).cos();
            let new_y = 0.1 * (self.angle as f32 * PI / 180.0).sin();
            if self.speed.x < 6.0 && self.speed.y < 6.0 {
                self.speed += Vec2::new(new_x, new_y);
            }
            if self.speed.x > 6.0 {
                self.speed = vec2(6.0, self.speed.y)
            }
            if self.speed.y > 6.0 {
                self.speed = vec2(self.speed.x, 6.0)
            }
            Self::go_to(self);
        }

        // Si c'est la touche "down", alors diminue la "speed".
        if is_key_down(KeyCode::Down) && norm > 0.0 {
            self.speed /= vec2(1.1, 1.1);
        }

        // Si la touche "up" n'est pas appuyé alors modifie la "speed" et la "position".
        if !is_key_down(KeyCode::Up) && norm > 0.0 {
            self.speed /= vec2(1.05, 1.05);
            let change = Self::go_to(self);
            self.position = Self::bound_pos(change);
        }
        self.position
    }

    /// Vérifie si l'objet de la structure "Spaceship" est en collision avec un objet de la structure
    /// "Asteroid".
    fn collision<T: StellarObject>(&mut self, ast: &T, size: f32) -> Self::ReturnType {
        if (ast.get_position() - self.get_position()).length() <= size {
            self.shield -= 1;
            let angle_rad = self.speed.length() * (PI / 180.0);
            self.speed = vec2(1.0, 1.0);
            let new_x = angle_rad.cos() * 50.0;
            let new_y = angle_rad.sin() * 50.0;
            if self.get_position().x > ast.get_position().x {
                self.position += vec2(new_x, new_y)
            } else {
                self.position -= vec2(new_x, new_y);
            }
            self.collision = true;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asteroid::Asteroid;
    #[test]
    fn check_get_position() {
        let ship = Spaceship {
            position: vec2(60.0, 50.0),
            speed: vec2(2.0, 2.0),
            angle: 20,
            shield: 3,
            collision: false,
        };
        assert_eq!(ship.get_position(), vec2(60.0, 50.0))
    }
    #[test]
    fn check_collision() {
        let asteroid = Asteroid {
            position: vec2(90.0, 60.0),
            speed: vec2(2.0, 2.0),
        };
        let mut ship = Spaceship {
            position: vec2(60.0, 50.0),
            speed: vec2(2.0, 2.0),
            angle: 20,
            shield: 3,
            collision: false,
        };
        assert_eq!(ship.collision(&asteroid, 65.0), true)
    }
}
