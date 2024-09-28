use seokjin_ai::{SeokjinAI, AIConfig};
use seokjin_ai::voice_interaction::VoiceModule;
use std::env;

fn main() {
    // Initialize logging
    seokjin_ai::core::logger::init().expect("Failed to initialize logger");

    let config = AIConfig {
        model_name: "SeokjinAI".to_string(),
        model_version: "1.0".to_string(),
        wake_word: "Hey Seokjin".to_string(),
    };

    let mut ai = SeokjinAI::new(config);

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // Process command-line arguments
        match args[1].as_str() {
            "learn" if args.len() > 2 => {
                if let Err(e) = ai.learn_from_web(&args[2]) {
                    eprintln!("Error learning from web: {}", e);
                }
            },
            "cli" => ai.run_cli(),
            "voice" => {
                let voice_module = VoiceModule::new().expect("Failed to initialize voice module");
                loop {
                    println!("Listening...");
                    match voice_module.listen() {
                        Ok(input) => {
                            println!("You said: {}", input);
                            match ai.process_input(&input) {
                                Ok(response) => {
                                    println!("Seokjin: {}", response);
                                    voice_module.speak(&response).expect("Failed to speak");
                                },
                                Err(e) => eprintln!("Error processing input: {}", e),
                            }
                        },
                        Err(e) => eprintln!("Error listening: {}", e),
                    }
                }
            },
            _ => println!("Usage: seokjin_ai [learn <url> | cli | voice]"),
        }
    } else {
        // Default to CLI mode if no arguments are provided
        ai.run_cli();
    }
}
