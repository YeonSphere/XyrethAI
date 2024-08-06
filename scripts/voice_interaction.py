import whisper
import os
import yaml
import logging
import logging.config
from dotenv import load_dotenv

# Load environment variables
load_dotenv()

# Load logging configuration
with open('./config/logging.yaml', 'r') as file:
    logging_config = yaml.safe_load(file)
logging.config.dictConfig(logging_config)
logger = logging.getLogger('my_app')

try:
    # Check required environment variables
    if not os.getenv('REQUIRED_ENV_VAR'):
        logger.error("Required environment variable 'REQUIRED_ENV_VAR' is not set.")
        raise EnvironmentError("Required environment variable 'REQUIRED_ENV_VAR' is not set.")

    # Load configuration
    try:
        with open('./config/config.yaml', 'r') as file:
            config = yaml.safe_load(file)
    except FileNotFoundError:
        logger.error("Configuration file not found.")
        raise

    # Whisper integration for speech recognition
    try:
        whisper_model = whisper.load_model("base")
    except Exception as e:
        logger.error(f"Failed to load Whisper model: {e}", exc_info=True)
        raise

    def collect_user_feedback(audio_file, transcription):
        user_feedback = input(f"Is the transcription correct for {audio_file}? (yes/no): ")
        if user_feedback.lower() == 'no':
            correct_text = input("Please provide the correct transcription: ")
            logger.info(f"Incorrect transcription: {transcription}, Correct transcription: {correct_text}")
            update_model_with_feedback([(audio_file, correct_text)])

    # Process all audio files in the audio directory
    audio_path = config['data']['audio_path']
    for audio_file in os.listdir(audio_path):
        if audio_file.endswith('.wav'):
            audio_file_path = os.path.join(audio_path, audio_file)
            result = whisper_model.transcribe(audio_file_path)
            logger.info(f"Transcription for {audio_file}: {result['text']}")
            collect_user_feedback(audio_file, result['text'])

except Exception as e:
    logger.error(f"An error occurred: {e}", exc_info=True)
