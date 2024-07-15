# model_training.py
from tensorflow.keras.models import Sequential
from tensorflow.keras.layers import Dense

def build_model(input_shape):
    """
    Build and compile the model.
    
    Args:
        input_shape (tuple): Shape of the input data.
    
    Returns:
        model: Compiled model.
    """
    model = Sequential([
        Dense(64, activation='relu', input_shape=input_shape),
        Dense(1, activation='sigmoid')
    ])
    model.compile(optimizer='adam', loss='binary_crossentropy', metrics=['accuracy'])
    return model

def train_model(model, X_train, y_train, epochs=10, batch_size=32):
    """
    Train the model.
    
    Args:
        model: Compiled model.
        X_train (np.array): Training data.
        y_train (np.array): Training labels.
        epochs (int): Number of epochs.
        batch_size (int): Batch size.
    
    Returns:
        history: Training history.
    """
    history = model.fit(X_train, y_train, epochs=epochs, batch_size=batch_size)
    return history

