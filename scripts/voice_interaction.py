import whisper
import os
import yaml
import logging
import logging.config

# Load logging configuration
with open('./config/logging.yaml', 'r') as file:
    logging_config = yaml.safe_load(file)
logging.config.dictConfig(logging_config)
logger = logging.getLogger('my_app')

try:
    # Load configuration
    with open('./config/config.yaml', 'r') as file:
        config = yaml.safe_load(file)

    # Whisper integration for speech recognition
    whisper_model = whisper.load_model("base")

    # Process all audio files in the audio directory
    audio_path = config['data']['audio_path']
    for audio_file in os.listdir(audio_path):
        if audio_file.endswith('.wav'):
            audio_file_path = os.path.join(audio_path, audio_file)
            result = whisper_model.transcribe(audio_file_path)
            logger.info(f"Transcription for {audio_file}: {result['text']}")

            # Collect user feedback (simplified example)
            user_feedback = input(f"Is the transcription correct for {audio_file}? (yes/no): ")
            if user_feedback.lower() == 'no':
                correct_text = input("Please provide the correct transcription: ")
                # Log the incorrect transcription and the correct one
                logger.info(f"Incorrect transcription: {result['text']}, Correct transcription: {correct_text}")

                # Update model with feedback (simplified example)
                update_model_with_feedback([(audio_file_path, correct_text)])

except Exception as e:
    logger.error(f"An error occurred: {e}", exc_info=True)
