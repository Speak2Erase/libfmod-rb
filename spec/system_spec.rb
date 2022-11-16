# frozen_string_literal: true

require "libfmod"

describe FMOD::Studio::System do
  describe "#system" do
    before do
      @system = FMOD::Studio::System.create
      @system.init(64, 0, 0)
    end

    describe "update" do
      it "updates the system" do
        @system.update
      end
    end

    describe "load_bank_file" do
      it "loads banks from files" do
        expect(@system.load_bank_file("media/Master.bank", 0))
      end
    end

    describe "get_core_system" do
      it "can get the core system" do
        expect(@system.get_core_system)
      end
    end

    describe "load_bank_memory" do
      it "loads banks from memory" do
        bank_data = File.read("media/Master.bank").bytes
        expect(@system.load_bank_memory(bank_data, FMOD::Enum::LoadMemoryMode::Memory, 0))
      end
    end
  end
end
