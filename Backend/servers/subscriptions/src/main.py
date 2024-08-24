from fastapi import FastAPI
from .ping.urls import router as ping_router
from .subscriptions.urls import router as subscriptions_router
import uvicorn

app = FastAPI()
app.include_router(ping_router)
app.include_router(subscriptions_router)

@app.get("/")
def read_root():
    return {"Hello": "World"}

def application():
    if __name__ == "src.main":
        uvicorn.run(app, host="127.0.0.1", port=8001)
