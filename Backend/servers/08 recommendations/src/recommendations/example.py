books_db = [
    {
        "id": "aaaa",
        "title": "The Art of Electronics",
        "authors": ["Paul Horowitz", "Winfield Hill"],
        "genres": ["Electrical Engineering", "Engineering", "Non-fiction"],
    },
    {
        "id": "bbbb",
        "title": "The Laws of Human Nature",
        "authors": ["Robbert Greene"],
        "genres": ["Self help", "Psychology", "Non-fiction"],
    },
    {
        "id": "cccc",
        "title": "TCP/IP Guide",
        "authors": ["Charles M. Kozierok"],
        "genres": ["Software Engineering", "Engineering", "Non-fiction"],
    },
    {
        "id": "dddd",
        "title": "Design Patterns Elements of Reusable Object-Oriented Software",
        "authors": ["Erich Gamma", "John Vlissides", "Richard Helm", "Ralph Johnson"],
        "genres": ["Software Architecture", "Software Engineering", "Engineering", "Non-fiction"],
    },
    {
        "id": "eeee",
        "title": "The Algorithm Design Manual",
        "authors": ["Steven S. Skiena"],
        "genres": ["Software Architecture", "Algorithms", "Software Engineering", "Engineering", "Non-fiction"],
    },
]

def get_recommendations(book_id, number_of_books=15, genre_level=10):
    book = next((book for book in books_db if book["id"] == book_id), None)
    if book == None:
        return []

    genres = book["genres"]

    recommendations = []
    for genre in genres[:genre_level]:
        recommendations += [
            book for book in books_db if genre in book["genres"] and book["id"] != book_id and book not in recommendations
        ]
    return recommendations[:number_of_books]

book_id = "dddd"
number_of_books = 3
genre_level = 3
book = next((book for book in books_db if book["id"] == book_id), None)
print(book)
recommendations = get_recommendations(book_id, number_of_books, genre_level)
import json
print(json.dumps(recommendations, indent=4))
