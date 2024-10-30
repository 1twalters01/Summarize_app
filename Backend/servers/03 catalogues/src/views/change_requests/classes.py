from pydantic import BaseModel

# Use rust for type safety for this bit?
class ChangeRequest(BaseModel):
    type: DatatypeEnum # Author, Book, Genre, Publisher
    fields: fieldsEnum # Based on DatatypeEnum


