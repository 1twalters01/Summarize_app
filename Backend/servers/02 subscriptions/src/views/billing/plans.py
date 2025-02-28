from src.datatypes.price import Price
from src.queries import price

async def get_plans_view():
    prices: list[Price] = price.get.all()
    return prices