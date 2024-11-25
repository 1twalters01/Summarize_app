defmodule PhoenixApp.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      # Starts a worker by calling: PhoenixApp.Worker.start_link(arg)
      # {PhoenixApp.Worker, arg}
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: PhoenixApp.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
