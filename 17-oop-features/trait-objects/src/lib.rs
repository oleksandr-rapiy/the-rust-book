pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // NOTE: dyn means that the type is a trait object
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
         // draw the button
    
    }
}
