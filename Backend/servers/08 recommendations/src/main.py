from fastapi import FastAPI
from .ping.urls import router as ping_router
import uvicorn

app = FastAPI()
app.include_router(ping_router)

def application():
    if __name__ == "src.main":
        print("ping: http://127.0.0.1:8008/ping/any_auth")
        uvicorn.run(app, host="127.0.0.1", port=8008)
