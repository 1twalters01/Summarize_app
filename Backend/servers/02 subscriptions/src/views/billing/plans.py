from src.datatypes.price import Price

async def get_plans_view():
    prices: Price = discount_service.get_base_prices()
    return prices