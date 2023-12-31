use crate::resources::State;
use chrono::{ Local, NaiveTime };
use core::time::Duration;

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

}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        App {
            should_quit: false,
            state: State::default(),
            should_stop_at: None,
            // 30 minutes
            session_lenght: Duration::from_secs(60 * 30),
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
    }

    pub fn stop_timer(&mut self) {
        self.state = State::Stopped;
        self.should_stop_at = None;
    }
}
