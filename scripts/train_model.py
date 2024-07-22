import tensorflow as tf
from tensorflow.keras.models import model_from_json
import yaml

# Load configuration
with open('config/config.yaml', 'r') as file:
    config = yaml.safe_load(file)

# Load model architecture
with open('models/model_architecture.json', 'r') as file:
    model_json = file.read()

model = model_from_json(model_json)

# Compile model
optimizer = tf.keras.optimizers.Adam(learning_rate=config['optimizer']['learning_rate'])
model.compile(optimizer=optimizer, loss='sparse_categorical_crossentropy', metrics=['accuracy'])

# Load data
train_data = tf.keras.preprocessing.image_dataset_from_directory(config['data']['train_path'])
val_data = tf.keras.preprocessing.image_dataset_from_directory(config['data']['val_path'])

# Train model
model.fit(train_data, validation_data=val_data, epochs=config['training']['epochs'])

# Save model
model.save('models/best_model')
