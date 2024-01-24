use crate::resources::State;
use chrono::{ Local, NaiveTime };
use core::time::Duration;
use crate::activities::ActivityList;
use crate::ledger::ledger_log;

/// Application.
#[derive(Debug)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// current app state 
    pub state: State,
    /// the time when the times should stop
    pub should_stop_at: Option<NaiveTime>,
    /// how long should the timer last
    pub session_lenght: Duration,
    /// Possible activities 
    pub activities: ActivityList,

}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        
        let activities = vec![
            "ProgrammingProjects".to_owned(),
                "ProgrammingProjects:Rust".to_owned(),
                "ProgrammingProjects:Web".to_owned(),
            "Studying".to_owned(),
                "Studying:Reti".to_owned(),
                "Studying:Algorithms".to_owned(),
                "Studying:IS".to_owned(),
            "Entertainment".to_owned(),
                "Entertainment:Youtube".to_owned(),
                "Entertainment:Film".to_owned(),
            ];

        App {
            should_quit: false,
            state: State::default(),
            should_stop_at: None,
            // 30 minutes
            session_lenght: Duration::from_secs(60 * 30),
            activities: ActivityList::from(activities),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn start_timer(&mut self) {
        self.state = State::Running; 
        self.should_stop_at = Some(Local::now().time() + self.session_lenght);

        log::info!("{} session started with mins {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                    self.session_lenght.as_secs_f32() / 60.0
                );

        match ledger_log(self.activities.get(), self.session_lenght.as_secs_f32() / 60.0) {
            Ok(()) => {},
            Err(why) => println!("Error opening the ledger file: {}", why),
        };

    }

    pub fn stop_timer(&mut self) {
        self.state = State::Stopped;
        self.should_stop_at = None;

        log::info!("{} stopped session",
                    Local::now().format("%Y-%m-%dT%H:%M:%S"),
                    );

    }
}
