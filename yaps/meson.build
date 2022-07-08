project('yaps', 'rust', 
          version: '0.1.0',
          meson_version: '>=0.50.0',
  default_options: [ 'warning_level=2',
                     'werror=false',
                   ],
)

cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock',
)

prefix = get_option('prefix')
bindir = join_paths(prefix, 'bin')

install_data('scripts/yaps.add.sh',
   install_dir: bindir
)

install_data('scripts/yaps.build.sh',
   install_dir: bindir
)

install_data('scripts/yaps.remove.sh',
   install_dir: bindir
)

install_data('scripts/yaps.trigger.sh',
   install_dir: bindir
)

subdir('src')

