use super::*;
use tokio::runtime::Runtime;

mod ai_tests {
    use super::*;
    use crate::ai::{self, Model, Prediction};

    #[test]
    fn test_model_initialization() {
        let rt = Runtime::new().unwrap();
        let model = rt.block_on(async {
            Model::new().await
        });
        assert!(model.is_ok());
    }

    #[test]
    fn test_text_prediction() {
        let rt = Runtime::new().unwrap();
        let model = rt.block_on(async {
            Model::new().await.unwrap()
        });

        let prediction = rt.block_on(async {
            model.predict_text("Hello, how are you?").await
        });
        assert!(prediction.is_ok());
    }

    #[test]
    fn test_error_handling() {
        let rt = Runtime::new().unwrap();
        let model = rt.block_on(async {
            Model::new().await.unwrap()
        });

        let prediction = rt.block_on(async {
            model.predict_text("").await
        });
        assert!(prediction.is_err());
    }
}

mod audio_tests {
    use super::*;
    use crate::audio::{self, AudioProcessor};

    #[test]
    fn test_audio_processing() {
        let processor = AudioProcessor::new();
        let sample_data = vec![0.0f32; 1024];
        let result = processor.process(&sample_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_audio_device_enumeration() {
        let devices = AudioProcessor::list_devices();
        assert!(devices.is_ok());
    }
}

mod language_tests {
    use super::*;
    use crate::language::{self, LanguageDetector};

    #[test]
    fn test_language_detection() {
        let detector = LanguageDetector::new();
        let result = detector.detect("Hello, world!");
        assert_eq!(result.unwrap(), "eng");
    }

    #[test]
    fn test_multiple_languages() {
        let detector = LanguageDetector::new();
        let texts = vec![
            ("Hello, world!", "eng"),
            ("Bonjour le monde!", "fra"),
            ("안녕하세요!", "kor"),
        ];

        for (text, expected) in texts {
            let result = detector.detect(text);
            assert_eq!(result.unwrap(), expected);
        }
    }
}

mod ffi_tests {
    use super::*;
    use crate::ffi::{self, Bridge};

    #[test]
    fn test_ffi_bridge() {
        let bridge = Bridge::new();
        let result = bridge.call("test_function", &["arg1", "arg2"]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_propagation() {
        let bridge = Bridge::new();
        let result = bridge.call("nonexistent_function", &[]);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_text_prediction(b: &mut Bencher) {
        let rt = Runtime::new().unwrap();
        let model = rt.block_on(async {
            Model::new().await.unwrap()
        });

        b.iter(|| {
            rt.block_on(async {
                model.predict_text("This is a benchmark test").await.unwrap()
            })
        });
    }

    #[bench]
    fn bench_audio_processing(b: &mut Bencher) {
        let processor = AudioProcessor::new();
        let sample_data = vec![0.0f32; 1024];

        b.iter(|| {
            processor.process(&sample_data).unwrap()
        });
    }
}
