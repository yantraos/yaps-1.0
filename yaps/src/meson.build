yaps_sources = [
  cargo_sources,
  'main.rs',
]

yaps_deps = [
]

cargo_options = [ '--manifest-path', meson.source_root() / 'Cargo.toml' ]
cargo_options += [ '--target-dir', meson.build_root() / 'src' ]


cargo_env = [ 'CARGO_HOME=' + meson.build_root() / 'cargo-home' ]
cargo = find_program('cargo', required: true)

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  input: yaps_sources,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    'env',
    cargo_env,
    cargo, 'build',
    cargo_options,
    '&&',
    'cp', 'src' / get_option('buildtype') / meson.project_name(), '@OUTPUT@',
  ]
)

