use rand::Rng;
use crate::core::error::SeokjinError;

pub struct HyperParameters {
    pub learning_rate: f64,
    pub batch_size: usize,
    pub num_layers: usize,
    pub dropout_rate: f64,
}

pub fn random_search(num_trials: usize) -> Result<Vec<HyperParameters>, SeokjinError> {
    let mut rng = rand::thread_rng();
    let hyperparameters: Vec<HyperParameters> = (0..num_trials)
        .map(|_| HyperParameters {
            learning_rate: rng.gen_range(0.0001..0.1),
            batch_size: 2_usize.pow(rng.gen_range(4..8)), // 16, 32, 64, 128, 256
            num_layers: rng.gen_range(1..10),
            dropout_rate: rng.gen_range(0.0..0.5),
        })
        .collect();

    if hyperparameters.is_empty() {
        Err(SeokjinError::InvalidInput("No hyperparameters generated".to_string()))
    } else {
        Ok(hyperparameters)
    }
}

pub fn grid_search() -> Vec<HyperParameters> {
    let learning_rates = vec![0.001, 0.01, 0.1];
    let batch_sizes = vec![32, 64, 128];
    let num_layers_options = vec![2, 3, 4];
    let dropout_rates = vec![0.1, 0.3, 0.5];

    learning_rates.iter().flat_map(|&lr| {
        batch_sizes.iter().flat_map(move |&bs| {
            num_layers_options.iter().flat_map(move |&nl| {
                dropout_rates.iter().map(move |&dr| {
                    HyperParameters {
                        learning_rate: lr,
                        batch_size: bs,
                        num_layers: nl,
                        dropout_rate: dr,
                    }
                })
            })
        })
    }).collect()
}
