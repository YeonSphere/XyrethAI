use crate::core::learning::LearningModule;
use crate::core::interaction::InteractionModule;
use crate::core::error::SeokjinError;
use crate::core::file_organizer::FileOrganizer;
use crate::core::platform_module::PlatformModule;
use crate::core::benchmarking::Benchmark;
use crate::core::data_preprocessing::preprocess_data;
use crate::core::hyperparameter_tuning::{HyperParameters, random_search};
use crate::core::character_control::CharacterControl;
use crate::core::ui_interactions::UIInteraction;
use crate::browser_integration::web_api::WebAPI;
use crate::os_integration::process_manager::ProcessManager;
use crate::voice_interaction::voice_module::VoiceModule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Deserialize, Serialize)]
pub struct AIConfig {
    pub model_name: String,
    pub model_version: String,
    pub wake_word: String,
}

pub struct SeokjinAI {
    config: AIConfig,
    learning_module: LearningModule,
    interaction_module: InteractionModule,
    file_index: HashMap<String, Vec<String>>,
    file_organizer: FileOrganizer,
    platform_module: PlatformModule,
    character_control: CharacterControl,
    ui_interaction: UIInteraction,
    web_api: WebAPI,
    process_manager: ProcessManager,
    voice_module: VoiceModule,
}

impl SeokjinAI {
    pub fn new(config: AIConfig) -> Self {
        SeokjinAI {
            config,
            learning_module: LearningModule::new(),
            interaction_module: InteractionModule::new(),
            file_index: HashMap::new(),
            file_organizer: FileOrganizer::new(),
            platform_module: PlatformModule::new(),
            character_control: CharacterControl::new(),
            ui_interaction: UIInteraction::new(),
            web_api: WebAPI::new(),
            process_manager: ProcessManager::new(),
            voice_module: VoiceModule::new().expect("Failed to initialize voice module"),
        }
    }

    pub fn process_input(&mut self, input: &str) -> Result<String, SeokjinError> {
        if !input.starts_with(&self.config.wake_word) {
            return Err(SeokjinError::InvalidInput("Wake word not detected".to_string()));
        }

        let response = self.interaction_module.generate_response(input)?;
        self.learning_module.learn_from_interaction(input, &response);
        Ok(response)
    }

    pub fn learn_from_web(&mut self, url: &str) -> Result<(), SeokjinError> {
        self.web_api.fetch_and_process(url).map_err(|e| SeokjinError::WebAPIError(e.to_string()))?;
        Ok(())
    }

    pub fn load_context_from_file(&mut self, file_path: &str) -> Result<(), SeokjinError> {
        let content = fs::read_to_string(file_path).map_err(|e| SeokjinError::IOError(e.to_string()))?;
        let extension = Path::new(file_path).extension().and_then(|s| s.to_str()).unwrap_or("");
        self.file_index.entry(extension.to_string()).or_default().push(content);
        Ok(())
    }

    pub fn run_cli(&mut self) {
        use std::io::{self, Write};
        println!("Seokjin AI CLI. Type 'exit' to quit.");
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "exit" {
                break;
            }
            match self.process_input(input) {
                Ok(response) => println!("{}", response),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}