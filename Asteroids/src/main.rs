use ::rand::{thread_rng, Rng};
use ::std::cmp::min;
use macroquad::audio::PlaySoundParams;
use macroquad::{
    audio::{load_sound, play_sound, Sound},
    prelude::*,
};
use std::f32::consts::PI;

use asteroid::Asteroid;
use bullet::Bullet;
use spaceship::Spaceship;
use stellarobject::StellarObject;

mod asteroid;
mod bullet;
mod spaceship;
mod stellarobject;

/// La fonction "draw_starting_background" prends en arguments une texture et permet de l'afficher ainsi
/// que d'écrire le texte de tutoriel pour jouer.
fn draw_starting_background(texture: &Texture2D) {
    let minimum: i32 = min(screen_height() as i32, screen_height() as i32);
    draw_texture_ex(
        texture,
        (screen_width() - minimum as f32) / 2.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(minimum as f32, minimum as f32)),
            ..Default::default()
        },
    );
    draw_text("CHOOSE YOUR LEVEL:", 10.0, 60.0, 30.0, GREEN);
    draw_text("PRESS B FOR A BASIC LEVEL", 60.0, 120.0, 30.0, GREEN);
    draw_text(
        "PRESS I FOR AN INTERMEDIATE LEVEL",
        60.0,
        180.0,
        30.0,
        GREEN,
    );
    draw_text("PRESS H FOR A HARD LEVEL", 60.0, 240.0, 30.0, GREEN);
    draw_text(
        "TO START",
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0 - 60.0,
        60.0,
        BLACK,
    );
    draw_text(
        "PRESS",
        screen_width() / 2.0 - 63.0,
        screen_height() / 2.0 + 10.0,
        60.0,
        BLACK,
    );
    draw_text(
        "'ENTER'",
        screen_width() / 2.0 - 85.0,
        screen_height() / 2.0 + 80.0,
        60.0,
        BLACK,
    );
    draw_text(
        "INSTRUCTIONS:",
        3.0 * screen_width() / 4.0,
        3.0 * screen_height() / 4.0,
        30.0,
        GREEN,
    );
    draw_text(
        "Up: to speed up",
        3.0 * screen_width() / 4.0,
        3.0 * screen_height() / 4.0 + 30.0,
        30.0,
        GREEN,
    );
    draw_text(
        "Down: to slow down",
        3.0 * screen_width() / 4.0,
        3.0 * screen_height() / 4.0 + 60.0,
        30.0,
        GREEN,
    );
    draw_text(
        "Right and left: to turn",
        3.0 * screen_width() / 4.0,
        3.0 * screen_height() / 4.0 + 90.0,
        30.0,
        GREEN,
    );
}

/// La fonction "draw_win_background" dessine l'image de victoire prise en argument ainsi que le texte
/// "CONGRATS YOU WON" et les instructions du jeu.
fn draw_win_backroung(texture: &Texture2D) {
    let coor_x = screen_width() / 4.0;
    let coor_y = screen_height() / 8.0;
    draw_texture_ex(
        texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        },
    );
    draw_text(
        "CONGRATS YOU WON!",
        coor_x,
        coor_y,
        screen_height() / 8.0,
        GREEN,
    );
}
/// La fonction "draw_game_over" dessine l'image de défaite prise en argument ainsi que le texte "GAME OVER".
fn draw_game_over(texture: &Texture2D) {
    let coor_x = screen_width() / 8.0 * 3.0;
    let coor_y = screen_height() / 8.0;
    draw_texture_ex(
        texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            rotation: 0.0,
            ..Default::default()
        },
    );
    draw_text("GAME OVER", coor_x, coor_y, 80.0, GREEN);
}

/// La fonction "draw_level_shiel" prend en argument un entier qui représente le nombre de vie restant au "shield"
/// du "spaceship". Elle permet d'afficher le nombre de point de vie restant au "shield".
fn draw_level_shield(shield: &i32) {
    draw_text("Level of shield", 0.0, screen_height() - 30.0, 20.0, WHITE);
    if shield == &3 {
        draw_rectangle(0.0, screen_height() - 20.0, 50., 12.0, RED);
        draw_rectangle(50.0, screen_height() - 20.0, 50.0, 12.0, ORANGE);
        draw_rectangle(100.0, screen_height() - 20.0, 50.0, 12.0, GREEN);
    } else if shield == &2 {
        draw_rectangle(0.0, screen_height() - 20.0, 50., 10.0, RED);
        draw_rectangle(50.0, screen_height() - 20.0, 50.0, 10.0, ORANGE);
    } else if shield == &1 {
        draw_rectangle(0.0, screen_height() - 20.0, 50.0, 10.0, RED);
    }
}
/// La fonction "handle_input" vérifie si la touche "Echap" est appuyez ou non. Si oui alors elle renvoie "True",
/// "False" sinon.
fn handle_input() -> bool {
    if is_key_down(KeyCode::Escape) {
        return true;
    }
    false
}

/// La fonction "update_model" va permettre à chaque quantum de temps de faire bouger tout les objets qui ont
/// le trait "StellarObject". Elle prends en arguments 5 éléments : une liste de grand "asteroid", une liste
/// de moyen "asteroid", une liste de petit "asteroid", le "spaceship" et une liste de "bullet".
fn update_model(
    asteroids_grand: &mut Vec<Asteroid>,
    asteroids_moyen: &mut Vec<Asteroid>,
    asteroids_petit: &mut Vec<Asteroid>,
    ship: &mut Spaceship,
    vec_bullet: &mut Vec<Bullet>,
) {
    for asteroid in asteroids_grand {
        asteroid.move_object();
    }
    for asteroid in asteroids_moyen {
        asteroid.move_object();
    }
    for asteroid in asteroids_petit {
        asteroid.move_object();
    }
    ship.move_object();
    for bullet in vec_bullet {
        bullet.move_object();
    }
}

/// La fonction "check_shield" vérifie si le "shield" est supérieur à 0. Elle prends en paramètre le "shield"
/// du "spaceship".
fn check_shield(shield: &i32) -> bool {
    shield < &0
}

/// La fonction "window_conf" configure la fenêtre qui va être lancée avec certains critères.
fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Project Asteroid"),
        high_dpi: true,
        sample_count: 100,
        fullscreen: true,
        ..Default::default()
    }
}
/// La fonction "main" est la fonction principale du script, elle lance la simulation du jeu asteroid.
#[macroquad::main(window_conf)]
async fn main() {
    //Charge toutes les textures et les sons nécesaire pour le bon fonctionnement du jeu.
    let texture_start: Texture2D = load_texture("annex/Planet.png").await.unwrap();
    let texture_big_asteroid: Texture2D =
        load_texture("annex/Model_Big_Asteroid.png").await.unwrap();
    let texture_mid_asteroid: Texture2D =
        load_texture("annex/Model_Mid_Asteroid.png").await.unwrap();
    let texture_lit_asteroid: Texture2D =
        load_texture("annex/Model_Lit_Asteroid.png").await.unwrap();
    let texture_spaceship: Texture2D = load_texture("annex/Model_Spaceship.png").await.unwrap();
    let texture_bullet = load_texture("annex/Texture_Bullet.png").await.unwrap();
    let sound_bullet: Sound = load_sound("annex/bullet_sound.ogg").await.unwrap();
    let sound_collision: Sound = load_sound("annex/sound_explosion.ogg").await.unwrap();
    let sound_spaceship: Sound = load_sound("annex/spaceship_collision.ogg").await.unwrap();

    // Initialise les vecteurs qui contiendront les "asteroid" de taille différentes.
    let mut big_asteroid: Vec<Asteroid> = Vec::new();
    let mut mid_asteroid: Vec<Asteroid> = Vec::new();
    let mut lit_asteroid: Vec<Asteroid> = Vec::new();

    // Création du "spaceship", du vecteur qui contiendra les "bullet", de la variable
    // "last_bullet" qui permet de gérer l'intervalle entre deux "bullet" envoyés et de la variable
    // qui confirme si le mode de jeu est choisi ou pas.
    let mut ship = Spaceship::new();
    let mut vec_bullet: Vec<Bullet> = Vec::new();
    let mut last_bullet = 0.0;
    let mut mode = false;

    // Création de l'arrière plan.
    let mut vec_background = Vec::new();
    for _ in 1..500 {
        let x = thread_rng().gen_range(0.0..=2000.0);
        let y = thread_rng().gen_range(0.0..=2000.0);
        vec_background.push((x, y))
    }

    // Initialisation du nombre d'"asteroid" en fonction de la difficultée.
    let mut nb_ast = 0;
    loop {
        draw_starting_background(&texture_start);
        if is_key_pressed(KeyCode::B) {
            nb_ast = 5;
            mode = true
        };
        if is_key_pressed(KeyCode::I) {
            nb_ast = 10;
            mode = true
        };
        if is_key_pressed(KeyCode::H) {
            nb_ast = 15;
            mode = true
        };

        // Création des "asteroid" par rapport au nombre d'"asteroid" initialsé plutôt.
        if is_key_pressed(KeyCode::Enter) && mode {
            for _ in 1..nb_ast {
                let asteroid = asteroid::Asteroid::new();
                big_asteroid.push(asteroid)
            }
            for _ in 1..nb_ast / 2 {
                let asteroid = asteroid::Asteroid::new();
                mid_asteroid.push(asteroid)
            }
            for _ in 1..(nb_ast / 3) + 1 {
                let asteroid = asteroid::Asteroid::new();
                lit_asteroid.push(asteroid)
            }

            loop {
                // Placement de l'arrière plan.
                for pos in &vec_background {
                    draw_circle(pos.0, pos.1, 1.0, WHITE);
                }

                // Affiche le "spaceship" dans le jeu
                draw_texture_ex(
                    &texture_spaceship,
                    ship.get_position().x - 30.0,
                    ship.get_position().y - 30.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(60.0, 60.0)),
                        rotation: ship.angle as f32 * PI / 180.0,
                        ..Default::default()
                    },
                );

                // Affiche les "asteroid" de grande taille dans le jeu.
                for elem in &big_asteroid {
                    draw_texture_ex(
                        &texture_big_asteroid,
                        elem.get_position().x - 70.0,
                        elem.get_position().y - 60.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(140.0, 120.0)),
                            ..Default::default()
                        },
                    );
                }

                // Affiche les "asteroid" de taille moyenne dans le jeu.
                for elem in &mid_asteroid {
                    draw_texture_ex(
                        &texture_mid_asteroid,
                        elem.get_position().x - 61.0,
                        elem.get_position().y - 61.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(122.0, 122.0)),
                            ..Default::default()
                        },
                    );
                }

                // Affiche les "asteroid" de petite taille dans le jeu.
                for elem in &lit_asteroid {
                    draw_texture_ex(
                        &texture_lit_asteroid,
                        elem.get_position().x - 35.0,
                        elem.get_position().y - 35.0,
                        GRAY,
                        DrawTextureParams {
                            dest_size: Some(vec2(70.0, 70.0)),
                            ..Default::default()
                        },
                    );
                }

                // Placement des "bullet" dans le jeu.
                for elem in &vec_bullet {
                    draw_texture_ex(
                        &texture_bullet,
                        elem.get_position().x - 10.0,
                        elem.get_position().y - 10.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(20.0, 20.0)),
                            rotation: elem.angle as f32 * PI / 180.0 + 180.0,
                            ..Default::default()
                        },
                    );
                }
                // Affiche la barre du niveau du "shield" du "spaceship".
                draw_level_shield(&ship.shield);

                // Vérifie les conditions d'arrêts : soit la touche "echap" est appuyé, soit le "shield" est à
                // zéro, soit il n'y a plus d'"asteroid" dans le jeu.
                if handle_input()
                    || check_shield(&ship.shield)
                    || (big_asteroid.len() + mid_asteroid.len() + lit_asteroid.len() == 0)
                {
                    break;
                }

                // Crée un "bullet" lorsque la touche "space" est appuyé.
                if is_key_down(KeyCode::Space) {
                    let now = get_time() as f32;
                    if (now - last_bullet) > 0.2 {
                        vec_bullet.push(Bullet::new(ship.get_position(), ship.get_angle()));
                        last_bullet = now;
                        play_sound(
                            &sound_bullet,
                            PlaySoundParams {
                                looped: false,
                                volume: 0.5,
                            },
                        )
                    }
                }

                // Garde uniquement les "bullet" présents dans la fenêtre.
                vec_bullet.retain(|bullet| {
                    bullet.get_position().x < screen_width()
                        && bullet.get_position().x > 0.0
                        && bullet.get_position().y < screen_height()
                        && bullet.get_position().y > 0.0
                });

                // Vérifie les collisions entre chaque "asteroid" de grande taille avec chaque "bullet". Si il y a une collision alors supprime
                // le "bullet" et l'"asteroid" et crée deux "asteroid" de taille moyenne.
                let size_big = 65.0;
                let mut temp_bull: Vec<Vec2> = Vec::new();
                for bullet in &vec_bullet {
                    let mut temp_ast = Vec::new();
                    for asteroid in &mut big_asteroid {
                        let temp = asteroid.collision(bullet, size_big).unwrap();
                        for elem in temp.0 {
                            temp_ast.push(elem)
                        }
                        for elem in temp.1 {
                            temp_bull.push(elem);
                            play_sound(
                                &sound_collision,
                                PlaySoundParams {
                                    looped: false,
                                    volume: 0.5,
                                },
                            )
                        }
                    }
                    big_asteroid.retain(|asteroid| !temp_ast.contains(&asteroid.get_position()));
                    for asteroid in temp_ast {
                        let mut nv_ast_1 = Asteroid::new();
                        nv_ast_1.position = vec2(asteroid.x - 30.0, asteroid.y - 30.0);
                        let mut nv_ast_2 = Asteroid::new();
                        nv_ast_2.position = vec2(asteroid.x + 30.0, asteroid.y + 30.0);
                        mid_asteroid.push(nv_ast_1);
                        mid_asteroid.push(nv_ast_2);
                    }
                }
                vec_bullet.retain(|bullet| !temp_bull.contains(&bullet.get_position()));

                // Vérifie les collisions entre chaque "asteroid" de taille moyenne avec chaque "bullet". Si il y a une collision alors supprime
                // le "bullet" et l'"asteroid" et crée deux "asteroid" de petite taille.
                let size_mid = 40.0;
                for bullet in &vec_bullet {
                    let mut temp_ast: Vec<Vec2> = Vec::new();
                    for asteroid in &mut mid_asteroid {
                        let temp = asteroid.collision(bullet, size_mid).unwrap();
                        for elem in temp.0 {
                            temp_ast.push(elem)
                        }
                        for elem in temp.1 {
                            temp_bull.push(elem);
                            play_sound(
                                &sound_collision,
                                PlaySoundParams {
                                    looped: false,
                                    volume: 0.5,
                                },
                            )
                        }
                    }
                    mid_asteroid.retain(|asteroid| !temp_ast.contains(&asteroid.get_position()));
                    for asteroid in temp_ast {
                        let mut nv_ast_1 = Asteroid::new();
                        nv_ast_1.position = vec2(asteroid.x - 20.0, asteroid.y - 20.0);
                        let mut nv_ast_2 = Asteroid::new();
                        nv_ast_2.position = vec2(asteroid.x + 20.0, asteroid.y + 20.0);
                        lit_asteroid.push(nv_ast_1);
                        lit_asteroid.push(nv_ast_2);
                    }
                }
                vec_bullet.retain(|bullet| !temp_bull.contains(&bullet.get_position()));

                // Vérifie les collisions entre chaque "asteroid" de petite taille avec chaque "bullet". Si il y a une collision alors supprime
                // le "bullet" et l'"asteroid".
                let size_lit = 25.0;
                for bullet in &vec_bullet {
                    let mut temp_ast: Vec<Vec2> = Vec::new();
                    for asteroid in &mut lit_asteroid {
                        if (bullet.get_position() - asteroid.get_position()).length() <= 25.0 {
                            let temp = asteroid.collision(bullet, size_lit).unwrap();
                            for elem in temp.0 {
                                temp_ast.push(elem)
                            }
                            for elem in temp.1 {
                                temp_bull.push(elem);
                                play_sound(
                                    &sound_collision,
                                    PlaySoundParams {
                                        looped: false,
                                        volume: 0.5,
                                    },
                                )
                            }
                        }
                    }
                    lit_asteroid.retain(|asteroid| !temp_ast.contains(&asteroid.get_position()));
                }
                vec_bullet.retain(|bullet| !temp_bull.contains(&bullet.get_position()));

                // Appel de la fonction qui va permettre à tous les élements visibles de bouger si nécessaire.
                update_model(
                    &mut big_asteroid,
                    &mut mid_asteroid,
                    &mut lit_asteroid,
                    &mut ship,
                    &mut vec_bullet,
                );

                // Vérifie les collisions entre le "spaceship" et chaque "asteroid" de grande taille. Si il y a une collision
                // alors le vaiseau doit reculer et perdre un point de "shield".
                let size = Asteroid::ASTEROID_INIT_SIZE + Spaceship::SPACESHIP_INIT_SIZE + 3.0;
                for asteroid in &big_asteroid {
                    ship.collision(asteroid, size);
                    if ship.collision {
                        play_sound(
                            &sound_spaceship,
                            PlaySoundParams {
                                looped: false,
                                volume: 0.5,
                            },
                        );
                        ship.collision = false
                    }
                }

                // Vérifie les collisions entre le "spaceship" et chaque "asteroid" de taille moyenne. Si il y a une collision
                // alors le vaiseau doit reculer et perdre un point de "shield".
                let size =
                    Asteroid::ASTEROID_INIT_SIZE / 2.0 + Spaceship::SPACESHIP_INIT_SIZE + 3.0;
                for asteroid in &mid_asteroid {
                    ship.collision(asteroid, size);
                }

                // Vérifie les collisions entre le "spaceship" et chaque "asteroid" de petite taille. Si il y a une collision
                // alors le vaiseau doit reculer et perdre un point de "shield".
                let size =
                    Asteroid::ASTEROID_INIT_SIZE / 3.0 + Spaceship::SPACESHIP_INIT_SIZE + 3.0;
                for asteroid in &lit_asteroid {
                    ship.collision(asteroid, size);
                }
                next_frame().await
            }
            // Boucle qui s'occupe soit de la victoire soit de la defaite
            loop {
                // Verifie le niveau du "shield" si celui-ci est inférieure à 0 alors lance la fonction "draw_game_over".
                if check_shield(&ship.shield) {
                    let texture_loose: Texture2D = load_texture("annex/Loose.png").await.unwrap();
                    draw_game_over(&texture_loose);
                // Verifie si il n'y a plus d'"asteroid" et lance la fonction "draw_win_backroung".
                } else if lit_asteroid.len() + mid_asteroid.len() + big_asteroid.len() == 0 {
                    let texture_win: Texture2D = load_texture("annex/Win.png").await.unwrap();
                    draw_win_backroung(&texture_win);
                }
                // Si le jeu a été quitté par l'appui de la touche "echap" alors quitte la fenetre.
                if handle_input() {
                    break;
                }
                next_frame().await
            }
            break;
        }
        // Si la touche "echap" ait appuyé pendant l'écran de réglage alors quitte la fenêtre.
        if handle_input() {
            break;
        }
        next_frame().await
    }
}
