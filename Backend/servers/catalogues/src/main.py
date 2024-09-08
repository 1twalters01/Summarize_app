import uvicorn
from fastapi import FastAPI
from .ping.urls import router as ping_router
# from .subscriptions.urls import router as subscriptions_router
from .settings.urls import router as settings_router

app = FastAPI()
app.include_router(ping_router)
# app.include_router(subscriptions_router)

def application():
    if __name__ == "src.main":
        print("ping: http://127.0.0.1:8002/ping/any_auth")
        uvicorn.run(app, host="127.0.0.1", port=8002)
