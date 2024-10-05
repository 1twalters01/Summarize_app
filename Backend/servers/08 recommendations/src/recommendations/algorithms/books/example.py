import json

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
    # replace json.dumps for a rust thing for speed?
    return json.dumps(recommendations[:recommendation_number], indent=4)

if __name__ == "__main__":
    book_id = "dddd"
    genre_level = 3
    recommendation_number = 3
    book = next((book for book in books_db if book["id"] == book_id), None)
    print(book)
    recommendations = get_recommendations(book_id, genre_level, recommendation_number)
    print(recommendations)
    # print(json.dumps(recommendations, indent=4))
