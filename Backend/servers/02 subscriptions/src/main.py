from fastapi import FastAPI
from routes import router

app = FastAPI()


app.include_router(router)

def main():
    if __name__ == "src.main":
        import uvicorn
        uvicorn.run(app, host="127.0.0.1", port=8002)
