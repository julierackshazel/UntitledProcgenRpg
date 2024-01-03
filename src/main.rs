use bevy::prelude::*;

fn hello_world() {
    println!("hello world");
}
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_peoples(mut commands: Commands) {
    commands.spawn((Person, Name("julie".to_string())));
    commands.spawn((Person, Name("racks".to_string())));
}

fn greet_the_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}", name.0);
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (hello_world, add_peoples, greet_the_people))
        .run();
    let at_the_disco = 3;
    panic!("{}", at_the_disco);
}
