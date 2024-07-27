pub struct History {
    current: String,
    backward_list: Vec<String>,
    forward_list: Vec<String>,
}

impl History {
    pub fn new(current: String) -> Self {
        todo!()
    }
    pub fn current(&self) -> &String {
        todo!()
    }
    pub fn backward(&mut self) -> Result<(), ()> {
        todo!()
    }
    pub fn forward(&mut self) -> Result<(), ()> {
        todo!()
    }
    pub fn can_backward(&self) -> bool {
        todo!()
    }
    pub fn can_forward(&self) -> bool {
        todo!()
    }
    pub fn goto(&mut self, new_url: String) {
        todo!()
    }
}
