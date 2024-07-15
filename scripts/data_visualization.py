# data_visualization.py
import matplotlib.pyplot as plt

def plot_data(data):
    """
    Plot the data.
    
    Args:
        data (pd.DataFrame): Data to plot.
    """
    plt.figure(figsize=(10, 6))
    plt.plot(data)
    plt.title('Data Visualization')
    plt.xlabel('X-axis')
    plt.ylabel('Y-axis')
    plt.show()

