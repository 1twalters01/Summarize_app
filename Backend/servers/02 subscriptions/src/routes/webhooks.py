from fastapi import APIRouter, Depends
from src.middleware.authentication import is_authenticated