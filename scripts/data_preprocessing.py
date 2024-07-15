# data_preprocessing.py
import pandas as pd

def preprocess_data(file_path):
    """
    Preprocess the data from the given file path.
    
    Args:
        file_path (str): Path to the CSV file.
    
    Returns:
        pd.DataFrame: Preprocessed data.
    """
    data = pd.read_csv(file_path)
    # Add your preprocessing steps here
    return data

