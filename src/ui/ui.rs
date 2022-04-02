use std::error::Error;

#[derive(Clone, Default)]
pub struct Ui {}

impl Ui {
    pub fn new() -> Self {
        Ui {
            ..Default::default()
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
