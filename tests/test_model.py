# Example test file: tests/test_model.py
import pytest
from tensorflow.keras.models import model_from_json

def test_model_loading():
    with open('./models/model_architecture.json', 'r') as file:
        model_json = file.read()
    model = model_from_json(model_json)
    assert model is not None
