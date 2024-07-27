pub struct History {
    current: String,
    backward_list: Vec<String>,
    forward_list: Vec<String>,
}

impl History {
    pub fn new(current: String) -> Self {
        Self {current, backward_list: vec![], forward_list: vec![]}
    }
    pub fn current(&self) -> &String {
        &self.current
    }
    pub fn backward(&mut self) -> Result<(), ()> {
        if self.backward_list.is_empty() {
            return Err(());
        }
        self.forward_list.push(self.current.clone());
        self.current=self.backward_list.pop().unwrap();
        Ok(())
    }
    pub fn forward(&mut self) -> Result<(), ()> {
        if self.forward_list.is_empty() {
            return Err(());
        }
        self.backward_list.push(self.current.clone());
        self.current=self.forward_list.pop().unwrap();
        Ok(())
    }
    pub fn can_backward(&self) -> bool {
        !self.backward_list.is_empty()
    }
    pub fn can_forward(&self) -> bool {
        !self.forward_list.is_empty()
    }
    pub fn goto(&mut self, new_url: String) {
        self.backward_list.push(self.current.clone());
        self.current=new_url;
        self.forward_list.clear();
    }
}
