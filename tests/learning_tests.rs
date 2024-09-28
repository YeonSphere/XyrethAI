use seokjin_ai::{SeokjinAI, AIConfig, LearningModule};
use std::fs;

#[cfg(test)]
mod tests {
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
    fn test_learning_module() {
        let mut learning_module = LearningModule::new();
        learning_module.learn_from_interaction("Hello", "Hi there!");
        let response = learning_module.get_learned_response("Hello");
        assert_eq!(response, Some(&"Hi there!".to_string()));
    }

    #[test]
    fn test_continuous_learning() {
        let mut ai = create_test_ai();
        let input = "What's the capital of France?";
        let first_response = ai.process_input(input).unwrap();
        let second_response = ai.process_input(input).unwrap();
        assert_eq!(first_response, second_response);
    }

    #[test]
    fn test_learn_from_web() {
        let mut ai = create_test_ai();
        let result = ai.learn_from_web("https://example.com");
        assert!(result.is_ok());
        // You might want to add more specific assertions here, depending on what you expect
        // the AI to learn from the example.com website
    }

    #[test]
    fn test_load_context_from_file() {
        let mut ai = create_test_ai();
        let test_file = "test_context.txt";
        fs::write(test_file, "This is a test context").unwrap();
        let result = ai.load_context_from_file(test_file);
        assert!(result.is_ok());
        fs::remove_file(test_file).unwrap();
    }
}
