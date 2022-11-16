# frozen_string_literal: true

require "bundler/gem_tasks"
require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rake/extensiontask"

task build: :compile

Rake::ExtensionTask.new("libfmod_ext") do |ext|
  ext.lib_dir = "lib/libfmod"
end

desc "Compile the extension with debug symbols"
task "compile:debug" do
  ENV["RB_SYS_CARGO_PROFILE"] = "dev"
  Rake::Task["compile"].invoke
end

task default: %i[clobber compile rubocop]
