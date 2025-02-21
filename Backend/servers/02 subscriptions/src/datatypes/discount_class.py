class DiscountClass():
    id: int
    code: str
    max_uses: int|None
    current_uses: int|None
    created_at: datetime
    expires_at: datetime|None