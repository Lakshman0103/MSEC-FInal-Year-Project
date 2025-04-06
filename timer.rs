use std::time::{Duration, Instant};

// Timer struct for measuring performance
pub struct Timer {
    start: Instant,
    checkpoints: Vec<(String, Duration)>,
}

impl Timer {
    // Create a new timer and start it
    pub fn new() -> Self {
        Timer {
            start: Instant::now(),
            checkpoints: Vec::new(),
        }
    }
    
    // Add a checkpoint with a label
    pub fn checkpoint(&mut self, label: &str) {
        let elapsed = self.start.elapsed();
        self.checkpoints.push((label.to_string(), elapsed));
    }
    
    // Get the elapsed time since the start
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
    
    // Print all checkpoints
    pub fn print_checkpoints(&self) {
        println!("\nPerformance Checkpoints:");
        println!("{:-<40}", "");
        
        for (i, (label, duration)) in self.checkpoints.iter().enumerate() {
            if i == 0 {
                println!("{}: {:.2?}", label, duration);
            } else {
                let previous_duration = self.checkpoints[i-1].1;
                let diff = duration.saturating_sub(previous_duration);
                println!("{}: {:.2?} (+{:.2?})", label, duration, diff);
            }
        }
        
        if !self.checkpoints.is_empty() {
            let last_checkpoint = self.checkpoints.last().unwrap().1;
            let diff = self.elapsed().saturating_sub(last_checkpoint);
            println!("Final: {:.2?} (+{:.2?})", self.elapsed(), diff);
        } else {
            println!("Total Time: {:.2?}", self.elapsed());
        }
        
        println!("{:-<40}", "");
    }
}