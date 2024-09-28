use seokjin_ai::{SeokjinAI, AIConfig, InteractionModule, LearningModule, PlatformModule};
use std::fs;
use std::path::Path;

#[cfg(test)]
mod integration_tests {
    use super::*;

    fn create_test_ai() -> SeokjinAI {
        let config = AIConfig {
            model_name: "TestModel".to_string(),
            model_version: "1.0".to_string(),
            wake_word: "Hey Seokjin".to_string(),
        };
        SeokjinAI::new(config)
    }

    #[test]
    fn test_full_interaction_flow() {
        let mut ai = create_test_ai();

        // Test initial interaction
        let result = ai.process_input("Hey Seokjin, what's the weather like?");
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("weather"));

        // Test learning from interaction
        ai.learn_from_interaction("weather", &response);

        // Test improved response after learning
        let result = ai.process_input("Hey Seokjin, tell me about the weather again");
        assert!(result.is_ok());
        let new_response = result.unwrap();
        assert!(new_response.contains("weather"));
        assert_ne!(response, new_response);  // Response should be different after learning

        // Test file operations
        let test_file = "test_context.txt";
        fs::write(test_file, "This is a test context").unwrap();
        let result = ai.load_context_from_file(test_file);
        assert!(result.is_ok());

        // Clean up
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_web_learning_and_response() {
        let mut ai = create_test_ai();

        // Test learning from web
        let result = ai.learn_from_web("https://example.com");
        assert!(result.is_ok());

        // Test response after learning from web
        let result = ai.process_input("Hey Seokjin, what did you learn from example.com?");
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("example") || response.contains("website"));
    }

    #[test]
    fn test_error_handling() {
        let mut ai = create_test_ai();

        // Test invalid input
        let result = ai.process_input("Invalid input without wake word");
        assert!(result.is_err());

        // Test non-existent file
        let result = ai.load_context_from_file("non_existent_file.txt");
        assert!(result.is_err());

        // Test invalid URL
        let result = ai.learn_from_web("https://invalid-url-that-does-not-exist.com");
        assert!(result.is_err());
    }
}
