# frozen_string_literal: true

require "libfmod"

system = FMOD::Studio::System.create
system.init(64, 0, 0)

bank = system.load_bank_file("spec/media/Master.bank", 0)
