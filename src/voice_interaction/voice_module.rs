use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};
use tts::Tts;

pub struct VoiceModule {
    host: cpal::Host,
    input_device: cpal::Device,
    output_device: cpal::Device,
    whisper_context: WhisperContext,
    tts: Tts,
}

impl VoiceModule {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let host = cpal::default_host();
        let input_device = host.default_input_device().ok_or("No input device available")?;
        let output_device = host.default_output_device().ok_or("No output device available")?;
        let whisper_context = WhisperContext::new("path/to/whisper/model.bin")?;
        let tts = Tts::default()?;

        Ok(VoiceModule {
            host,
            input_device,
            output_device,
            whisper_context,
            tts,
        })
    }

    pub fn speak(&self, text: &str) -> Result<(), Box<dyn Error>> {
        println!("Speaking: {}", text);
        self.tts.speak(text, false)?;
        Ok(())
    }

    pub fn listen(&self) -> Result<String, Box<dyn Error>> {
        let config = self.input_device.default_input_config()?;
        let sample_rate = config.sample_rate().0;
        let channels = config.channels();

        let recording = Arc::new(Mutex::new(Vec::new()));
        let recording_clone = Arc::clone(&recording);

        let stream = self.input_device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                recording_clone.lock().unwrap().extend_from_slice(data);
            },
            |err| eprintln!("Error in input stream: {}", err),
        )?;

        stream.play()?;
        std::thread::sleep(Duration::from_secs(5)); // Record for 5 seconds
        stream.pause()?;

        let recorded_data = recording.lock().unwrap().clone();

        // Convert recorded data to i16 format for Whisper
        let audio_data: Vec<i16> = recorded_data.iter().map(|&x| (x * 32768.0) as i16).collect();

        // Perform speech recognition using Whisper
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        let mut state = self.whisper_context.create_state()?;
        state.full(params, &audio_data)?;

        let num_segments = state.full_n_segments()?;
        let mut transcription = String::new();

        for i in 0..num_segments {
            transcription.push_str(&state.full_get_segment_text(i)?);
            transcription.push(' ');
        }

        Ok(transcription.trim().to_string())
    }
}
