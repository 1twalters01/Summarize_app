import smtplib, ssl
from email.mime.multipart import MIMEMultipart
from email.mime.text import MIMEText

class EmailService():
    smtp_username: str
    smtp_password: str
    smtp_server: str
    recipient: str
    subject: str|None
    body: str|None

    def __init__(self, recipient: str):
        DATABASE_URL = os.getenv("PG_URL")

        smtp_username = os.getenv("SMTP_USERNAME")
        smtp_password = os.getenv("SMTP_PASSWORD")
        smtp_server = os.getenv("SMTP_SERVER")

        if smtp_username == None:
            raise RuntimeError(f"Invalid smtp username")
        if smtp_password == None:
            raise RuntimeError(f"Invalid smtp password")
        if smtp_server == None:
            raise RuntimeError(f"Invalid smtp server")

        self.smtp_username = smtp_username
        self.smtp_password = smtp_password
        self.smtp_server = smtp_server
        self.recipient = recipient

    def compose_message(message_type_enum):
        pass

    def send_email(self):
        msg = MIMEMultipart()
        msg["From"] = self.smtp_username
        msg["To"] = self.recipient
        msg["Subject"] = self.subject
        msg.attach(MIMEtext(self.body, "html"))

        try:
            server = smtplib.SMTP(smtp_server, smtp_port)
            context = ssl.create_default_context()
            server.starttls(context=context)
            server.login(self.smtp_username, self.recipient)
            server.sendmail(self.smtp_username, self.recipient, msg.as_string())
            server.quit()
        except Exception as e:
            raise RuntimeError(f"Error sending email: {e}")
