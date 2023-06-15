use std::collections::VecDeque;

use bevy::app::{AppLabel, SubApp};
use bevy::prelude::*;

use crate::MainMessageQue;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, AppLabel)]
struct MySubApp;

#[derive(Resource, Default)]
pub struct SubMessageQue(pub VecDeque<String>);

pub fn create_sub_app(app: &mut App) {
    // create a app with a resource and a single schedule
    let mut sub_app = App::empty();

    // add an outer schedule that runs the main schedule
    sub_app.add_simple_outer_schedule();

    // initialize main schedule
    sub_app.init_schedule(CoreSchedule::Main);

    sub_app.init_resource::<MainMessageQue>();
    sub_app.init_resource::<SubMessageQue>();

    // Add system
    sub_app.add_startup_system(sub_setup);
    sub_app.add_system(sub_system);

    // add the sub_app to the app
    app.insert_sub_app(MySubApp, SubApp::new(sub_app, extract_app));
}

fn extract_app(main_world: &mut World, sub_app: &mut App) {
    // Extract main MessageQue
    main_world
        .resource_mut::<MainMessageQue>()
        .0
        .append(&mut sub_app.world.resource_mut::<MainMessageQue>().0);

    // Extract sub MessageQue
    sub_app
        .world
        .resource_mut::<SubMessageQue>()
        .0
        .append(&mut main_world.resource_mut::<SubMessageQue>().0);
}

fn sub_setup(mut commands: Commands) {
    // Spawn a second window
    commands.spawn(Window {
        title: "Second window".to_owned(),
        ..default()
    });
}

fn sub_system(mut main_message: ResMut<MainMessageQue>, mut sub_message: ResMut<SubMessageQue>) {
    if let Some(message) = sub_message.0.pop_front() {
        println!("Sub app recieved this message: {}", message);
    }

    main_message.0.push_back(String::from("Hi"));
    main_message.0.push_back(String::from("There"));
}
