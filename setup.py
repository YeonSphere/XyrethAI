# setup.py
from setuptools import setup, find_packages

setup(
    name='Seokhwa',
    version='1.0',
    packages=find_packages(),
    install_requires=[
        'numpy',
        'pandas',
        'matplotlib',
        'tensorflow',
        'scikit-learn',
        'jupyter'
    ],
)

