Microservice written purely in elixir for enabling multi-user collaboration on reading and writting.

# Why Elixir
* 

# Overview
* 

# Details
* Switch collaboration mode on and off for read and write
* Add users that are allowed to read/write on a per summary or account basis
* Allow hiding summaries from users you allow on an account basis
  * Optionally disable this feature for educators in organisations
* Web sockets allow users to collaborate in real time
* Writer
  * Lock each paragraph folder as it is being updated
* Reader
  * Set colour of highlights and notes for different participants

# Running
* Run `mix setup` to install and setup dependencies
* Start Phoenix endpoint with `mix phx.server` or inside IEx with `iex -S mix phx.server`
