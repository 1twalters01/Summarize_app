project('Summarize', 'c')

gtkdep = dependency('gtk4')
luadep = dependency('lua5.4')
dependencies = [gtkdep, luadep]

executable(
  '../target/summarize',
  'src/main.c',
  dependencies : [gtkdep, luadep]
)
