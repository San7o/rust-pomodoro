use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Padding},
};

use chrono::Local;
use crate::resources::State;
use crate::app::App;

const ASCII_ART: &str = r"
  _____           _           _____                          _                 
 |  __ \         | |         |  __ \                        | |                
 | |__) |   _ ___| |_ _   _  | |__) |__  _ __ ___   ___   __| | ___  _ __ ___  
 |  _  / | | / __| __| | | | |  ___/ _ \| '_ ` _ \ / _ \ / _` |/ _ \| '__/ _ \ 
 | | \ \ |_| \__ \ |_| |_| | | |  | (_) | | | | | | (_) | (_| | (_) | | | (_) |
 |_|  \_\__,_|___/\__|\__, | |_|   \___/|_| |_| |_|\___/ \__,_|\___/|_|  \___/ 
                       __/ |                                                   
                      |___/                                                    
";

const ASCII_ART2: &str = "
   .|||||||||.
  |||||||||||||
 /. `|||||||||||
o__,_||||||||||'
";

pub fn render(app: &mut App, f: &mut Frame) {
    


    // Check if session is terminated
    if app.state != State::Finished {
        if let Some(stop_time) = app.should_stop_at {
            if Local::now().time() > stop_time {
                app.state = State::Finished;
            }
        }
    }
   
    let frame_size = f.size(); // Rect
   
    let end_session_text = match app.state {
        State::Running => {
            format!("Session ending at {}",
                    // Get only hours and minutes
                    &app.should_stop_at.unwrap().to_string()[..5])
        },
        State::Stopped => {
            String::new()
        },
        State::Finished => {
            String::from("Tiime Finished! Go take some rest now")
        }
    };

    // TODO Add pomodoro ascii art
    f.render_widget(
    Paragraph::new(format!(
      "
{}
Current Time: {}
Current state: {:?} 
{} Minutes Session \n 
{}
      ",
      ASCII_ART2,
      &Local::now().time().to_string()[..5],
      app.state,
      app.session_lenght.as_secs_f32() / 60.0,
      end_session_text
    ))
   .block(
      Block::default()
        .title("Rusty Pomodoro")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        /*
        .padding(Padding::new(
            0, // left
            0, // right
            frame_size.height / 3, // top
            0, // bottom
        ))
        */
        ,
    )
    .style(Style::default().fg(Color::Blue))
    .alignment(Alignment::Center),
    f.size(),
  )
}
