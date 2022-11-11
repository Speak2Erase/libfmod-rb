# frozen_string_literal: true

require_relative "lib/libfmod/version"

Gem::Specification.new do |spec|
  spec.name = "libfmod"
  spec.version = FMOD::VERSION
  spec.authors = ["Speak2Erase"]
  spec.email = ["lily@nowaffles.com"]

  spec.summary = "Ruby bindings to FMOD"
  spec.homepage = "https://github.com/Astrabit-ST/fmod-rs"
  spec.required_ruby_version = ">= 2.6.0"

  spec.metadata["allowed_push_host"] = "TODO: Set to your gem server 'https://example.com'"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/Astrabit-ST/fmod-rs"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (f == __FILE__) || f.match(%r{\A(?:(?:bin|test|spec|features)/|\.(?:git|travis|circleci)|appveyor)})
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/libfmod_ext/extconf.rb"]

  # actually a build time dependency, but that's not an option.
  spec.add_runtime_dependency "rake", "> 1"

  # needed until rubygems supports Rust support is out of beta
  spec.add_dependency "rb_sys", "~> 0.9.37"

  # only needed when developing or packaging your gem
  spec.add_development_dependency "rake-compiler", "~> 1.2.0"

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
