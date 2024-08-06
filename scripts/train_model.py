import tensorflow as tf
import yaml
import whisper
import os
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
    # Load configuration
    with open('./config/config.yaml', 'r') as file:
        config = yaml.safe_load(file)

    # Load model architecture
    with open('./models/model_architecture.json', 'r') as file:
        model_json = file.read()

    model = tf.keras.models.model_from_json(model_json)

    # Set mixed precision policy
    tf.keras.mixed_precision.set_global_policy('mixed_float16')

    # Compile model with mixed precision
    optimizer = tf.keras.optimizers.Adam(learning_rate=config['optimizer']['learning_rate'])
    model.compile(optimizer=optimizer, loss='sparse_categorical_crossentropy', metrics=['accuracy'])

    # Load data
    train_data = tf.keras.preprocessing.image_dataset_from_directory(
        './datasets/train'
    )
    val_data = tf.keras.preprocessing.image_dataset_from_directory(
        './datasets/validation'
    )

    # Train model with mixed precision
    model.fit(train_data, validation_data=val_data, epochs=config['training']['epochs'])

    # Save model
    model.save('./models/best_model')

    # Whisper integration for speech recognition
    whisper_model = whisper.load_model("base")

    # Process all audio files in the audio directory
    audio_path = './datasets/audio'
    for audio_file in os.listdir(audio_path):
        if audio_file.endswith('.wav'):
            audio_file_path = os.path.join(audio_path, audio_file)
            result = whisper_model.transcribe(audio_file_path)
            logger.info(f"Transcription for {audio_file}: {result['text']}")

except Exception as e:
    logger.error(f"An error occurred: {e}", exc_info=True)
