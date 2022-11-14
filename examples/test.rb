#! /usr/bin/ruby

require "libfmod"

System = FMOD::Studio::System::create
System.init(64, 0, 0)

Master = System.load_bank_file("spec/media/Master.bank", 0)
Strings = System.load_bank_file("spec/media/Master.strings.bank", 0)

Strings.get_string_count.times do |i|
    guid, string = Strings.get_string_info(i)
end

Master.get_event_list.each do |e|
    rand(1..5).to_i.times do |i|
        e.create_instance
    end
end

loop do
    System.update
end