use bevy::{ecs::query, prelude::*, time};

fn hello_world() {
    println!("hello world");
}
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
            .add_systems(Startup, add_peoples)
            .add_systems(Update, greet_the_people);
    }
}
fn add_peoples(mut commands: Commands) {
    commands.spawn((Person, Name("julie".to_string())));
    commands.spawn((Person, Name("racks".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_the_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}", name.0);
        }
    }
}
fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
