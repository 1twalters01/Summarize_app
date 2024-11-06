defmodule FlashcardsTest do
  use ExUnit.Case
  doctest Flashcards

  test "greets the world" do
    assert Flashcards.hello() == :world
  end
end
