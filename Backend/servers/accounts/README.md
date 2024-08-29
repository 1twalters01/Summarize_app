Accounts microservice created in Rust.

# Overview
* A solution like clerk.js was not used due to costs.
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
