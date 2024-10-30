from fastapi import FastAPI
from .ping.urls import router as ping_router
# from .settings.urls import router as settings_router
# from .moderation.urls import router as moderation_router
import uvicorn

app = FastAPI()
app.include_router(ping_router)
# app.include_router(settings_router)
# app.include_router(moderation_router)

def application():
    if __name__ == "src.main":
        print("ping: http://127.0.0.1:8014/ping/any_auth")
        uvicorn.run(app, host="127.0.0.1", port=8014)
