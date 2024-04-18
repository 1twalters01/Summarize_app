project('Summarize', 'c', version : '0.0.1')

# dependencies
deps = [
  dependency('gtk4'),
  dependency('lua5.4'),
  dependency('sqlite3')
]

# source file variables
project_sources = []
project_header_files = []

# include directories for h files and lua files
inc = [
  include_directories('include'),
  include_directories('lua'),
  include_directories('css'),
]

# subdirs of the project
subdir('src')
subdir('include')
subdir('lua')
subdir('css')
subdir('data')

executable('summarize',
  project_sources,
  include_directories : inc,
  dependencies : deps
)

