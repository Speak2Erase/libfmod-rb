# frozen_string_literal: true

require "bundler/gem_tasks"
require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rake/extensiontask"

task build: :compile

Rake::ExtensionTask.new("libfmod_ext") do |ext|
  ext.lib_dir = "lib/libfmod"
end

task default: %i[clobber compile rubocop]
