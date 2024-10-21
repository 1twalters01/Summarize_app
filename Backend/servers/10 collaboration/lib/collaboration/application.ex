defmodule Collaboration.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      CollaborationWeb.Telemetry,
      Collaboration.Repo,
      {DNSCluster, query: Application.get_env(:collaboration, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Collaboration.PubSub},
      # Start the Finch HTTP client for sending emails
      {Finch, name: Collaboration.Finch},
      # Start a worker by calling: Collaboration.Worker.start_link(arg)
      # {Collaboration.Worker, arg},
      # Start to serve requests, typically the last entry
      CollaborationWeb.Endpoint
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Collaboration.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    CollaborationWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
