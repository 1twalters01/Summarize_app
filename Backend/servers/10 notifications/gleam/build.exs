# move.exs

output_directory = "../_build/dev/lib/notifications_gleam"
if File.exists?(output_directory) do
  File.rm_rf!(output_directory)
end

System.cmd("gleam", ["build"], into: IO.stream(:stdio, :line))
File.rename!("build", output_directory)
