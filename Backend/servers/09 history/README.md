History microservice created purely in Go.

# Why Go
* Easy to use language with good speed
* Performance is not critical due to likely lower useage
    * Caching the more recent results will increase desired performance

# Overview
* Retrieve and store the user's history

# Details
* Keeps a detailed history of the users activity
* Also keep a simple history
* Various ways of sorting through the results
    * Cache common queries
* Split into sections - authors/artists, books, summaries, genres, etc.
    * Cache more recent results
        * Store bulk data in some sort of data lake
        * Store midterm data in a database
        * Store shortterm data in a cache
