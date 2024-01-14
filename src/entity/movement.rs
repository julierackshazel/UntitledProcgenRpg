// Movement that works for enemy or player?
// with collision
fn move_right(time: Res<time>, key: Res<Input<KeyCode>>, mut query: Query<&mut Transform, with<Player>>) {
    sprite_bundle: SpriteBundle;
    if key.pressed(KeyCode::Right) {
        transform.translation.y += 150. * time.delta_seconds();
    }
}
