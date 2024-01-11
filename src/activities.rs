
#[derive(Debug)]
pub struct ActivityList {
    activities: Vec<String>,
    index: usize,
}

impl ActivityList {
    pub fn new() -> Self {
        Self {
            activities: Vec::new(),
            index: 0
        }
    }

    pub fn from(vec: Vec<String>) -> Self {
        Self {
            activities: vec,
            index: 0,
        }
    }

    pub fn get(&self) -> String {
        self.activities[self.index].clone()
    }

    pub fn next(&mut self) -> String {
        if self.index + 1 >= self.activities.len() {
            self.index = 0;
        }
        else {
            self.index += 1;
        }

        self.activities[self.index].clone()
    } 

    pub fn prev(&mut self) -> String {
        if self.index == 0 {
            self.index = self.activities.len() - 1;
        }
        else {
            self.index -= 1;
        }

        self.activities[self.index].clone()
    }
}
