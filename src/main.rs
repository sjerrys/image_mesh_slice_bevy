use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,                                                                         
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let texture_handle = asset_server.load("textures/sprites_array.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let mut uvs = Vec::new();
    uvs.push([0.0, 1.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([1.0, 0.0]);
    uvs.push([1.0, 1.0]);

    let mut mesh = Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    // use only the upper left part of the texture atlas
    let mut uvs1 = Vec::new();
    uvs1.push([0.0, 0.5]);
    uvs1.push([0.0, 0.0]);
    uvs1.push([0.5, 0.0]);
    uvs1.push([0.5, 0.5]);

    let mut mesh1 = Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)));
    mesh1.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs1);

    // use only the lower right part of the texture atlas
    let mut uvs2 = Vec::new();
    uvs2.push([0.5, 1.0]);
    uvs2.push([0.5, 0.5]);
    uvs2.push([1.0, 0.5]);
    uvs2.push([1.0, 1.0]);

    let mut mesh2 = Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)));
    mesh2.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs2);

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: material_handle.clone(),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh1),
        material: material_handle.clone(),
        transform: Transform::from_translation(Vec3::new(2.0, 0.0, 0.0)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh2),
        material: material_handle.clone(),
        transform: Transform::from_translation(Vec3::new(-2.0, 0.0, 0.0)),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..default()
    });


}