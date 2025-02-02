from fastapi import FastAPI
import uvicorn
from src.routes.ping import router as ping_router
from src.routes.subscriptions import router as subscriptions_router
from src.routes.settings import router as settings_router

app = FastAPI()
app.include_router(ping_router)
app.include_router(subscriptions_router)
app.include_router(settings_router)


def main():
    if __name__ == "src.main":
        print("ping: http://127.0.0.1:8002/ping/any_auth")
        uvicorn.run(app, host="127.0.0.1", port=8002)
