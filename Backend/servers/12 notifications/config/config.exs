import Config

# Select Json library
config :phoenix, :json_library, Jason

# Select server port
config :notifications, Notificationss.Web.Endpoint,
  http: [port: 8012]

# Logging
config :logger, level: :info

