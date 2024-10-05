Accounts microservice created purely in Rust.

# Why Rust
* A fast, smooth experience here is critical for first impressions
* Will have a lot of throughput so low memory usage is desired
* Has solid libraries and great web development frameworks
* Is a very reusable piece of software so the time investment is worth it
* No proprietary apis so there are no worries about rust support

# Overview
* Solutions like clerk.js will take all your money
* Rust was chosen as it is a fast, efficient, lightweight language with solid libraries and great web devevelopment frameworks.
* Actix_web is the framework of choice due to its strong async capabilities and speed.
* Protobufs are used rather than json due to lower data size requirements to try and minimize costs.
* Creates JWT access tokens and opaque refresh tokens

# Details
* Has standard email functionality: login, registration, password reset, 2FA
* Has settings to update email, password, 2FA, username, name, language, theme
* Has settings to delete account
* Will have captcha for email login
* Will have Oauth2 for Google and Apple
* Will consolidate Google/Apple Oauth2 and respective emails
