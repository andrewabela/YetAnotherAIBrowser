import torch
from diffusers import StableDiffusionPipeline, DDIMScheduler
from typing import Union
import uvicorn
from fastapi import FastAPI
from starlette.responses import StreamingResponse
from io import BytesIO

pipe = StableDiffusionPipeline.from_pretrained("prompthero/openjourney-v4")
pipe.scheduler = DDIMScheduler.from_config(pipe.scheduler.config)
pipe = pipe.to("cuda")

app = FastAPI()

@app.get("/")
def read_root():
    return "use /generate_image?description=<description> to generate an image"

@app.get("/generate_image")
def generate_image(description: str) -> Union[str, bytes]:
    image = pipe(description).images[0]
    image_stream = BytesIO()
    image.save(image_stream, format="PNG")
    image_stream.seek(0)
    return StreamingResponse(image_stream, media_type="image/png")

if __name__ == "__main__":
    # Only serve localhost
    uvicorn.run(app, host="127.0.0.1", port=2345)
