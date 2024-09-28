use std::time::{Instant, Duration};
use std::collections::HashMap;

pub struct Benchmark {
    name: String,
    start_time: Instant,
    checkpoints: HashMap<String, Duration>,
}

impl Benchmark {
    pub fn new(name: &str) -> Self {
        Benchmark {
            name: name.to_string(),
            start_time: Instant::now(),
            checkpoints: HashMap::new(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn checkpoint(&mut self, checkpoint_name: &str) {
        let elapsed = self.elapsed();
        self.checkpoints.insert(checkpoint_name.to_string(), elapsed);
    }

    pub fn get_checkpoint(&self, checkpoint_name: &str) -> Option<&Duration> {
        self.checkpoints.get(checkpoint_name)
    }

    pub fn print_results(&self) {
        println!("Benchmark: {}", self.name);
        println!("Total time: {:?}", self.elapsed());
        for (name, duration) in &self.checkpoints {
            println!("  {}: {:?}", name, duration);
        }
    }
}
