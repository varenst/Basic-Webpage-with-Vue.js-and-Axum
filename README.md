# Description

This project classifies satelite images of earth into predefined categories (cloudy, desert, green_area, and water) using a pre-trained model.
The backend server, implemented in Rust, loads and processes the image using a Python model trained in PyTorch, frontend is done with Vue.

# Requirements
Python (version 3.8 or later)
Rust
Git (for cloning the repository)

# Installation
 - Step 1: Clone the Repository
Clone this repository to your local machine:

``` bash
git clone https://github.com/yourusername/yourproject.git
cd yourproject
```

- Step 2: Set Up Python Environment and Install Dependencies
This project uses Python for model training. 

use a virtual environment:

Create and activate the virtual environment:

``` bash
python -m venv mlenv
```
Activate it:
``` bash
On Windows: mlenv\Scripts\activate
On macOS/Linux: source mlenv/bin/activate
```

Install dependencies:

``` bash
pip install -r requirements.txt
```

- Step 3: Configure the Rust Backend
Ensure Rust is installed on your system, then navigate to the backend directory:

``` bash
cd backend
cargo build
```

Project Structure
Here is an overview of the main directories and files:

``` bash
project/
├── backend/ 
│   ├── src/               # Rust backend server code
├── frontend/              # Frontend with vue
├── ml/       
│   ├── model.pth          # Saved PyTorch model
│   ├── __init__.py        # Python code for model loading
│   └── requirements.txt   # Python dependencies
├── mlenv/                 # python environment you will create
└── README.md              # Project instructions
```

# Setting Up and Running the Project

1. Train the Model
If training is required, navigate to the ml/ directory and run:

``` bash
python -m ml.train_model
```

This will train the model on the data in ml/data and save it as ml/model.pth.

2. Run the Rust Backend Server
To start the Rust backend:

``` bash
cd backend
cargo run
```
The server should start on localhost:3000 and be ready to handle requests.

3. Run the frontend
To start the frontend:

``` bash
cd frontend
npm run ("build" or "dev")
```
 
5. Test the Model Prediction
To test the model prediction, use the test_model function in Python or send an image request to the Rust server.

with command line

``` bash
python -c "from ml import test_model; test_model('path/to/image.jpg')"
```
