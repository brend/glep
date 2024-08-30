pub trait OutputTarget {
    fn write(&mut self, message: &str);
}

pub struct StdoutTarget;

impl OutputTarget for StdoutTarget {
    fn write(&mut self, message: &str) {
        println!("{}", message);
    }
}

#[allow(dead_code)]
pub struct TestOutputTarget {
    pub messages: Vec<String>,
}

impl OutputTarget for TestOutputTarget {
    fn write(&mut self, message: &str) {
        self.messages.push(message.to_string());
    }
}
