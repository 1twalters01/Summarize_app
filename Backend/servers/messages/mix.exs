defmodule Messages.MixProject do
  use Mix.Project

  def project do
    [
      app: :messages,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:phoenix, "~> 1.7.14"}, # phoenix framework
      {:telemetry_metrics, "~> 1.0"},  # Monitoring tools
      {:telemetry_poller, "~> 1.0"},
      {:jason, "~> 1.4.4"},   # JSON handling
      {:plug_cowboy, "~> 2.7.2"}, # web framework


      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
    ]
  end
end
