#! /usr/bin/ruby

require "libfmod"

System = FMOD::Studio::System::create
System.init(64, 0, 0)

Master = System.load_bank_file("spec/media/Master.bank", 0)
Strings = System.load_bank_file("spec/media/Master.strings.bank", 0)
puts Master.get_path
puts Strings.get_path

Strings.get_string_count.times do |i|
    guid, string = Strings.get_string_info(i)
end

Master.get_event_list.each do |e|
    puts e.get_path
end