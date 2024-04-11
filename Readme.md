# Summarise
## Lanugages
* Server:
    * Rust - Actix
* Website:
    * HTML
    * CSS - SCSS
    * JS - SolidJS, JSDoc
* Linux:
    * C - GTK
    * Lua
* Android:
    * Kotlin - Android SDK
* iOS:
    * Swift - XCode
* Windows:
    * C# - Windows Forms Apps

## Requirements
* Book files
    * Accept various rich text formats
        * html, md, xml, docx, epub
        * should be able to transform between them
        * pdf with OCR reader
    * File/folder upload
    * File/folder download

* Databases
    * Media
        * ID
        * Title
        * Subtitle
        * Images
        * Authors
        * Contributors
        * Edition
        * Series & number
        * Formats
            * Publisher
            * Publication date
            * Page count/audio length
            * Languages
            * ISBN
            * Weight
        * References (Studies, books, links, etc.)
        * Book description
        * Categories & Genres
        * Age group
        * Links (amazon, other websites), Ratings, Price
        * Chapters
            * Titles
            * Time to read
        * Summaries - Filterable
    * Authors
        * ID
        * First Name
        * Last Name
        * Author bio
        * Social Media
        * User|None
        * Average rating
        * Media - Filterable
        * get full name()
        * get first name()
        * get last name()
        * is User()
        * get User()
        * get Average rating()
    * Users
        * Username
        * First Name
        * Last Name
        * Email
        * Password - use proper security steps
        * Last Login
        * Datetime Joined
        * Groups
        * User Permissions
        * Is Staff
        * Is Active
        * Is Superuser
        * Is Authenticated
        * Is Anonymous
        * get username()
        * get full name()
        * get first name()
        * get last name()
        * set password()
        * check password()
        * get user permissions()
        * get groups()
        * has permission()
        * has permissions()

        * Favourite Authors
        * Liked Summaries
        * Bookmarked Summaries
        * Summaries
            * Finished
            * In Progress
            * Want to summarise
    * Summaries
        * Title
        * Media
        * User
        * Time to read - break down into chapters
        * Summary length to book length ratio
        * Likes
        * Bookmarks
        * Comments (optional)

* Community
    * Points for how much engagement your summaries get
    * Follow authors and summary writers
    * Leaderboard
    * Trending
        * Summaries
        * Summary Author
        * Books
        * Book Author

* Subscription
    * Not logged in
        * Part of Summary Reads Hidden
        * Cannot bookmark, like, follow, etc.
    * Free
        * Limited summary reads/writes
        * Very Limited written summary length
        * Forced Public written summaries
    * Paid
        * Unlimited summary reads/writes
        * Semi Limited written summary length
        * Private or selected viewers on summaries

* Features
    * ics calendar to remember to write summary
    * Pomodoro timer when writing
