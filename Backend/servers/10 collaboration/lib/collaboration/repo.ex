defmodule Collaboration.Repo do
  use Ecto.Repo,
    otp_app: :collaboration,
    adapter: Ecto.Adapters.Postgres
end
