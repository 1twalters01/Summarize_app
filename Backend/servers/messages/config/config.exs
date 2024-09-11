import Config

# Select Json library
config :phoenix, :json_library, Jason

# Select server port
config :messages, Messages.Web.Endpoint,
  http: [port: 8010]

# Logging
config :logger, level: :info

