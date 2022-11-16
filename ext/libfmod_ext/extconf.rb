# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("libfmod_ext/libfmod_ext") do |r|
  # r.force_install_rust_toolchain = "nightly"
end
