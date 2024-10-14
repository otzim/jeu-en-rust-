use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_player)
        .add_systems(Update, move_player)
        .run();
}

// Structure pour le joueur
#[derive(Component)]
struct Player {
    direction: Vec2,  // Direction du joueur pour l'animation
    is_attacking: bool, // Indique si le joueur attaque
    is_running: bool,   // Indique si le joueur court
}

// Structure pour contrôler l'animation
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

// Fonction de setup pour charger le sprite sheet et les arbres
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Ajoute une caméra 2D
    commands.spawn(Camera2dBundle::default());

    // Charge le sprite sheet du joueur
    let texture_handle = asset_server.load("Player.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(32.0, 32.0),
        9,
        8,
        None,
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Ajoute le joueur
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::splat(2.0),
            ..default()
        },
        ..default()
    })
    .insert(Player {
        direction: Vec2::new(0.0, -1.0),
        is_attacking: false,
        is_running: false,
    })
    .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));

    // Ajoute des arbres à la scène
    spawn_trees(&mut commands, &asset_server);
}

// Fonction pour ajouter des arbres à la scène
fn spawn_trees(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let tree_texture_handle = asset_server.load("Oak_Tree.png");

    // Vous pouvez ajouter plusieurs arbres à différentes positions
    for i in 0..5 {
        commands.spawn(SpriteBundle {
            texture: tree_texture_handle.clone(),
            transform: Transform {
                translation: Vec3::new(-100.0 + (i * 50) as f32, -50.0, 0.0), // Position des arbres
                scale: Vec3::splat(1.0), // Échelle des arbres
                ..default()
            },
            ..default()
        });
    }
}

// Système pour animer le joueur
fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Player)>,
) {
    for (mut timer, mut sprite, player) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if player.is_attacking {
                sprite.index = 8; // Index pour l'animation d'attaque
            } else {
                // Met à jour le sprite en fonction de la direction du joueur
                let row = if player.direction.x == 0.0 && player.direction.y == 1.0 {
                    0 // Haut
                } else if player.direction.x == 0.0 && player.direction.y == -1.0 {
                    1 // Bas
                } else if player.direction.x == -1.0 && player.direction.y == 0.0 {
                    2 // Gauche
                } else if player.direction.x == 1.0 && player.direction.y == 0.0 {
                    3 // Droite
                } else {
                    1 // Par défaut : bas
                };

                sprite.index = (sprite.index + 1) % 9 + row * 9; // 9 images par ligne
            }
        }
    }
}

// Système pour déplacer le joueur avec les touches du clavier
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in &mut query {
        let mut direction = Vec3::ZERO;
        let mut player_direction = Vec2::ZERO;

        // Déplacement du joueur et mise à jour de la direction pour l'animation
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= if player.is_running { 4.0 } else { 2.0 };
            player_direction = Vec2::new(-1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += if player.is_running { 4.0 } else { 2.0 };
            player_direction = Vec2::new(1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += if player.is_running { 4.0 } else { 2.0 };
            player_direction = Vec2::new(0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= if player.is_running { 4.0 } else { 2.0 };
            player_direction = Vec2::new(0.0, -1.0);
        }

        // Vérifie si le joueur attaque
        if keyboard_input.just_pressed(KeyCode::Space) {
            player.is_attacking = true;
        } else if keyboard_input.just_released(KeyCode::Space) {
            player.is_attacking = false;
        }

        // Vérifie si le joueur court
        player.is_running = keyboard_input.pressed(KeyCode::ControlLeft);

        // Met à jour la position du joueur
        if direction != Vec3::ZERO {
            transform.translation += direction;
            player.direction = player_direction;
        }
    }
}
