# model_evaluation.py
from sklearn.metrics import accuracy_score

def evaluate_model(model, X_test, y_test):
    """
    Evaluate the model.
    
    Args:
        model: Trained model.
        X_test (np.array): Test data.
        y_test (np.array): Test labels.
    
    Returns:
        float: Accuracy of the model.
    """
    y_pred = model.predict(X_test)
    y_pred = (y_pred > 0.5).astype(int)
    accuracy = accuracy_score(y_test, y_pred)
    return accuracy

