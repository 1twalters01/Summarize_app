defmodule MessagesTest do
  use ExUnit.Case
  doctest Messages

  test "greets the world" do
    assert Messages.hello() == :world
  end
end
