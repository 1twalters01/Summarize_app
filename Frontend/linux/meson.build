project('Summarize', 'c', version : '0.0.1')

# dependencies
deps = [
  dependency('gtk4'),
  dependency('lua5.4')
]

# source file variables
project_sources = []
project_header_files = []
configure_file(input : 'src/widget_test_2/widget_2.lua', output : 'widget_2.lua', copy : true)

# include directories for h files
inc = include_directories('include')

# subdirs of the project
subdir('src')
subdir ('include')

executable('summarize',
  project_sources,
  include_directories : inc,
  dependencies : deps
)

