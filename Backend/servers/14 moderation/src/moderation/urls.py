from fastapi import APIRouter
from .views.report.summary import report_summary
from .views.report.message import report_message
from .views.report.comment import report_comment
from .views.report.user import report_user
from .views.block.user import block_user, unblock_user
from .views.block.author import block_author, unblock_author
from .views.block.summarizer import block_summarizer, unblock_summarizer
from .views.moderate.user import ban_user

router = APIRouter()

router.add_api_route("/report/summary", report_summary, methods=["POST"])
router.add_api_route("/report/message", report_message, methods=["POST"])
router.add_api_route("/report/comment", report_comment, methods=["POST"])
router.add_api_route("/report/user", report_user, methods=["POST"])

router.add_api_route("/block/user", block_user, methods=["POST"])
router.add_api_route("/block/author", block_author, methods=["POST"])
router.add_api_route("/block/summarizer", block_summarizer, methods=["POST"])
router.add_api_route("/unblock/user", unblock_user, methods=["POST"])
router.add_api_route("/unblock/author", unblock_author, methods=["POST"])
router.add_api_route("/unblock/summarizer", unblock_summarizer, methods=["POST"])

router.add_api_route("/moderate/user", ban_user, methods=["POST"])

