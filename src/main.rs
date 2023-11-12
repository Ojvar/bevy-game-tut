use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Person {
        name: "Parsa".to_string(),
        job: Some(Job::Doctor),
    });
    commands.spawn(Person {
        name: "Paria".to_string(),
        job: Some(Job::FireFighter),
    });
    commands.spawn(Person {
        name: "Parnia".to_string(),
        job: None,
    });
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, print_names);
    }
}

fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        if let Some(job) = &person.job {
            println!("{} has job {}", person.name, job.to_string());
        } else {
            println!("{} is ready for hire", person.name);
        }
    }
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}
impl From<&str> for Job {
    fn from(value: &str) -> Self {
        match value {
            "Doctor" => Job::Doctor,
            "FireFighter" => Job::FireFighter,
            "Lawyer" => Job::Lawyer,
            _ => panic!("Invalid string for Job enum"),
        }
    }
}
impl ToString for Job {
    fn to_string(&self) -> String {
        match self {
            Job::Doctor => "Doctor".to_string(),
            Job::FireFighter => "FireFighter".to_string(),
            Job::Lawyer => "Lawyer".to_string(),
        }
    }
}

#[derive(Component)]
pub struct Person {
    pub job: Option<Job>,
    pub name: String,
}
