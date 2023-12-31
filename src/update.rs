use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use chrono::Local;

use crate::app::App;
use crate::resources::State;

pub fn update(app: &mut App, key_event: KeyEvent) {
      
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Char('s') => app.start_timer(),
        KeyCode::Char('p') => app.stop_timer(),
        KeyCode::Char(' ') => {
            match &app.state {
                State::Running => {
                    app.stop_timer();
                },
                State::Stopped | State::Finished => {
                    app.start_timer();
                }
            }
        },
        _ => {}
    };
}
