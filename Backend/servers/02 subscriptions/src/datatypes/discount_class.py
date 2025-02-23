class DiscountClass():
    id: int
    code: str
    max_uses: int|None
    current_uses: int|None
    created_at: datetime
    expires_at: datetime|None

    def __init__(self, id: int, code: str, max_uses: int | None, current_uses: int | None, created_at: datetime, expires_at: datetime | None):
        self.id = id
        self.code = code
        self.max_uses = max_uses
        self.current_uses = current_uses
        self.created_at = created_at
        self.expires_at = expires_at