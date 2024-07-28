import deepspeech
import wave
import mycroft

# Load DeepSpeech model
model_file_path = 'models/deepspeech-0.9.3-models.pbmm'
model = deepspeech.Model(model_file_path)

# Function to transcribe audio
def transcribe_audio(audio_file):
    with wave.open(audio_file, 'rb') as wf:
        audio = wf.readframes(wf.getnframes())
        text = model.stt(audio)
        return text

# Function to generate speech
def generate_speech(text):
    mycroft.speak(text)

# Example usage
audio_file = 'path/to/audio.wav'
text = transcribe_audio(audio_file)
generate_speech(f"You said: {text}")
