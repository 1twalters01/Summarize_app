defmodule RustClient do
  @moduledoc "Client to interact with Rust service"

  use Tesla

  plug Tesla.Middleware.BaseUrl, ""
  plug Tesla.Middleware.JSON

end
