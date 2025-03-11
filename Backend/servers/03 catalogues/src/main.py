import uvicorn
from fastapi import FastAPI
from src.routes.ping import router as ping_router
from src.routes.manual_input import router as manual_input_router
from src.routes.scraped_input import router as scraped_input_router

app = FastAPI()
app.include_router(ping_router)
app.include_router(manual_input_router)
app.include_router(scraped_input_router)


def application():
    if __name__ == "src.main":
        print("ping: http://127.0.0.1:8003/ping/any_auth")
        uvicorn.run(app, host="127.0.0.1", port=8003)
