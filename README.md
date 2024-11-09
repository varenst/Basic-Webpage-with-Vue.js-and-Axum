# Description

This project classifies satelite images of earth into predefined categories (cloudy, desert, green_area, and water) using a pre-trained model.
The backend server, implemented in Rust, loads and processes the image using a Python model trained in PyTorch, frontend is dow with Vue.

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

Step 2: Set Up Python Environment and Install Dependencies
This project uses Python for model training and inference. To ensure a clean environment, use a virtual environment:

Create and activate the virtual environment:

bash
Copy code
python -m venv mlenv
Activate it:
On Windows: mlenv\Scripts\activate
On macOS/Linux: source mlenv/bin/activate
Install dependencies:

bash
Copy code
pip install -r requirements.txt
Step 3: Configure the Rust Backend
Ensure Rust is installed on your system, then navigate to the backend directory:

bash
Copy code
cd backend
cargo build
Project Structure
Here is an overview of the main directories and files:

bash
Copy code
project/
├── backend/               # Rust backend server code
│   ├── src/               # Rust source files
│   └── target/            # Rust build output (ignored by .gitignore)
├── frontend/              # Frontend code, if applicable
├── ml/                    # Machine learning model and training code
│   ├── data/              # Training data (ignored by .gitignore)
│   ├── model.pth          # Saved PyTorch model
│   ├── __init__.py        # Python code for model loading and inference
│   └── requirements.txt   # Python dependencies
├── mlenv/                 # Virtual environment directory (ignored by .gitignore)
└── README.md              # Project instructions
Setting Up and Running the Project
1. Train the Model
If training is required, navigate to the ml/ directory and run:

bash
Copy code
python -m ml.train_model
This will train the model on the data in ml/data and save it as ml/model.pth.

2. Run the Rust Backend Server
To start the Rust backend:

Ensure your virtual environment is activated.

Navigate to the backend directory and run:

bash
Copy code
cargo run
The server should start on localhost:3000 and be ready to handle requests.

3. Test the Model Prediction
To test the model prediction, use the test_model function in Python or send an image request to the Rust server. Ensure the model file model.pth is present in the ml directory.

For example, in Python:

bash
Copy code
python -c "from ml import test_model; test_model('path/to/image.jpg')"
