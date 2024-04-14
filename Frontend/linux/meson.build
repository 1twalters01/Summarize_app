project('Summarize', 'c', version : '0.0.1')

# dependencies
deps = [
  dependency('gtk4'),
  dependency('lua5.4')
]

# source file variables
project_sources = []
project_header_files = []

# include directories for h files
inc = include_directories('include')

# subdirs of the project
subdir('src')
subdir ('include')

executable('../target/summarize',
  project_sources,
  include_directories : inc,
  dependencies : deps
)

