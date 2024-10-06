import uvicorn
from fastapi import FastAPI
from .ping.urls import router as ping_router
# from .settings.urls import router as settings_router
from .authors.urls import router as authors_router
from .publishers.urls import router as publishers_router
from .genres.urls import router as genres_router
from .books.urls import router as books_router

app = FastAPI()
app.include_router(ping_router)
# app.include_router(settings_router)
app.include_router(authors_router)
app.include_router(publishers_router)
app.include_router(genres_router)
app.include_router(books_router)

def application():
    if __name__ == "src.main":
        print("ping: http://127.0.0.1:8003/ping/any_auth")
        uvicorn.run(app, host="127.0.0.1", port=8003)
