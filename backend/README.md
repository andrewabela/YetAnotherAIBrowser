# YAAB Image Generation Service Backend

This is a simple python script that creates a FastAPI server that generates images using `prompthero/openjourney-v4` a Stable Diffusion model.

## Running the Server
1. It is recommended using a dedicated Python virtual environment.
```bash
python -m venv .venv
source .venv/bin/activate  # On Windows use `.venv\Scripts\activate`
```
2. Install the required dependencies.
```bash
pip install -r requirements.txt
```
3. Run the FastAPI server.
```bash
python image-gen.py
```
4. After the model is downloaded the server will be available at `http://localhost:2345`.