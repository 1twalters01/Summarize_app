from fastapi import APIRouter, Depends
from src.middleware.authentication import is_admin, is_authenticated

from src.views.manual_input.forced import (
    insert_manual_forced_formats,
    insert_manual_forced_genres,
    insert_manual_forced_publishers,
    insert_manual_forced_author,
    insert_manual_forced_books,
)
from src.views.manual_input.regular import (
    insert_manual_regular_formats,
    insert_manual_regular_genres,
    insert_manual_regular_publishers,
    insert_manual_regular_author,
    insert_manual_regular_books,
)
from src.views.scraped_input_input.post import (
    post_scraped_formats,
    post_scraped_genres,
    post_scraped_publishers,
    post_scraped_authors,
    post_scraped_books,
)
from src.views.scraped_input_input.confirm import (
    confirm_scraped_formats,
    confirm_scraped_genres,
    confirm_scraped_publishers,
    confirm_scraped_authors,
    confirm_scraped_books,
)

router = APIRouter()

router.add_api_route("/catalogue/manual-input/formats/admin", insert_manual_forced_formats, methods=["POST"])
router.add_api_route("/catalogue/manual-input/genres/admin", insert_manual_forced_genres, methods=["POST"])
router.add_api_route("/catalogue/manual-input/publishers/admin", insert_manual_forced_publishers, methods=["POST"])
router.add_api_route("/catalogue/manual-input/authors/admin", insert_manual_forced_author, methods=["POST"])
router.add_api_route("/catalogue/manual-input/books/admin", insert_manual_forced_books, methods=["POST"])

router.add_api_route("/catalogue/manual-input/formats/admin", insert_manual_forced_formats, methods=["POST"])
router.add_api_route("/catalogue/manual-input/genres/admin", insert_manual_forced_genres, methods=["POST"])
router.add_api_route("/catalogue/manual-input/publishers/admin", insert_manual_forced_publishers, methods=["POST"])
router.add_api_route("/catalogue/manual-input/authors/admin", insert_manual_forced_author, methods=["POST"])
router.add_api_route("/catalogue/manual-input/books/admin", insert_manual_forced_books, methods=["POST"])

router.add_api_route("/catalogue/scraped-input/formats", post_scraped_formats, methods=["POST"])
router.add_api_route("/catalogue/scraped-input/genres", insert_manual_forced_genres, methods=["POST"])
router.add_api_route("/catalogue/scraped-input/publishers", insert_manual_forced_publishers, methods=["POST"])
router.add_api_route("/catalogue/scraped-input/authors", insert_manual_forced_author, methods=["POST"])
router.add_api_route("/catalogue/scraped-input/books", insert_manual_forced_books, methods=["POST"])

router.add_api_route("/catalogue/scraped-input/formats/confirmation", confirm_scraped_formats, methods=["POST"])
router.add_api_route("/catalogue/scraped-input/genres/confirmation", insert_manual_forced_genres, methods=["POST"])
router.add_api_route("/catalogue/scraped-input/publishers/confirmation", insert_manual_forced_publishers, methods=["POST"])
router.add_api_route("/catalogue/scraped-input/authors/confirmation", insert_manual_forced_author, methods=["POST"])
router.add_api_route("/catalogue/scraped-input/books/confirmation", insert_manual_forced_books, methods=["POST"])