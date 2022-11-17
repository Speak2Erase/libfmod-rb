#! /usr/bin/ruby
# frozen_string_literal: true

require "libfmod"

puts FMOD::EventThread

System = FMOD::Studio::System.create
System.init(64, 0, 0)

puts System.get_user_data.to_s
System.set_user_data([
                       "test",
                       :this_is_fine,
                       {
                         "L" => -> {}
                       }
                     ])

GC.start
puts System.get_user_data.to_s

puts FMOD::Studio.parse_id("{00000000-0000-0000-0000-000000000000}")

Master = System.load_bank_file("media/Master.bank", 0)
Strings = System.load_bank_file("media/Master.strings.bank", 0)
Vehicles = System.load_bank_memory(File.read("media/Vehicles.bank").bytes, FMOD::Enum::LoadMemoryMode::Memory, 0)

puts Master.get_path
puts Strings.get_path

Strings.get_string_count.times do |i|
  _guid, _string = Strings.get_string_info(i)
end

Master.get_event_list.each do |e|
  rand(1..5).to_i.times do |_i|
    e.create_instance
  end
end

System.get_parameter_description_list.each do |p|
  puts p
end

System.set_callback(proc { |_system, _type, _data, userdata|
  puts userdata.to_s

  0
}, 0xFFFFFFFF)

loop do
  System.update

  sleep(1.0 / 60.0)
end
