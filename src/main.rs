use bevy::prelude::*;

use std::collections::VecDeque;

mod sub_app;

use sub_app::{create_sub_app, SubMessageQue};

#[derive(Resource, Default)]
pub struct MainMessageQue(pub VecDeque<String>);

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.init_resource::<MainMessageQue>();
    app.init_resource::<SubMessageQue>();

    create_sub_app(&mut app);

    app.add_startup_system(main_setup);
    app.add_system(main_system);

    // This will run the schedules once, since we're using the default runner
    app.run();
}

fn main_setup(mut commands: Commands) {
    commands.spawn(NodeBundle {
        background_color: Color::WHITE.into(),
        ..default()
    });
}

fn main_system(mut main_message: ResMut<MainMessageQue>, mut sub_message: ResMut<SubMessageQue>) {
    if let Some(message) = main_message.0.pop_front() {
        println!("Main app recieved this message: {}", message);
    }

    sub_message.0.push_back(String::from("Good"));
    sub_message.0.push_back(String::from("Day"));
}
