import json, uuid
from dataclasses import dataclass

@dataclass
class Book:
    id: str
    title: str
    authors: list[str]
    genres: list[str]

    @staticmethod
    def from_dict(obj_arr):
        self = [
            Book(
                obj.get("id"),
                obj.get("title"),
                obj.get("authors"),
                obj.get("genres")
            ) for obj in obj_arr
        ]

        return self

books_db = [
    {
        "id": "e09b06aa-0dfa-4d21-ae79-edc0cd10a6e6",
        "title": "The Art of Electronics",
        "authors": ["Paul Horowitz", "Winfield Hill"],
        "genres": ["Electrical Engineering", "Engineering", "Non-fiction"],
    },
    {
        "id": "857db374-123c-435b-9901-389ac9eaf0b7",
        "title": "The Laws of Human Nature",
        "authors": ["Robbert Greene"],
        "genres": ["Self help", "Psychology", "Non-fiction"],
    },
    {
        "id": "2a9089f2-01c9-4f97-8f3e-69e3a5fcd04d",
        "title": "TCP/IP Guide",
        "authors": ["Charles M. Kozierok"],
        "genres": ["Software Engineering", "Engineering", "Non-fiction"],
    },
    {
        "id": "dc924fb5-38c2-43ae-a7ec-540dc51c14e9",
        "title": "Design Patterns Elements of Reusable Object-Oriented Software",
        "authors": ["Erich Gamma", "John Vlissides", "Richard Helm", "Ralph Johnson"],
        "genres": ["Software Architecture", "Software Engineering", "Engineering", "Non-fiction"],
    },
    {
        "id": "737b74b4-858f-4ac5-a615-46158fa46495",
        "title": "The Algorithm Design Manual",
        "authors": ["Steven S. Skiena"],
        "genres": ["Software Architecture", "Algorithms", "Software Engineering", "Engineering", "Non-fiction"],
    },
]

def get_recommendations(book_id, genre_level=10, recommendation_number=15):
    book = next((book for book in books_db if book["id"] == book_id), None)
    if book == None:
        return []

    genres = book["genres"]

    recommendations = []
    for genre in genres[:genre_level]:
        recommendations += [
            book for book in books_db if genre in book["genres"] and book["id"] != book_id and book not in recommendations
        ]

    books = recommendations[:recommendation_number]
    return Book.from_dict(books)
    # return json.dumps(books, indent=4)

if __name__ == "__main__":
    book_id = "2a9089f2-01c9-4f97-8f3e-69e3a5fcd04d"
    genre_level = 3
    recommendation_number = 3
    book = next((book for book in books_db if book["id"] == book_id), None)
    recommendations = get_recommendations(book_id, genre_level, recommendation_number)
    print(recommendations)
