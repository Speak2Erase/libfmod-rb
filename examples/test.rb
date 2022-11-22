#! /usr/bin/ruby
# frozen_string_literal: true

require "libfmod"

puts FMOD::EventThread

System = FMOD::Studio::System.create
System.init(64, 0, 0)

System.set_callback(proc { |_system, _type, _data, _userdata|
  0
}, 0xFFFFFFFF)

puts FMOD::Studio.parse_id("{00000000-0000-0000-0000-000000000000}")

Master = System.load_bank_file("media/Master.bank", 0)
Strings = System.load_bank_file("media/Master.strings.bank", 0)

puts Master.get_path
puts Strings.get_path

Strings.get_string_count.times do |i|
  _guid, _string = Strings.get_string_info(i)
end

Master.get_event_list.each_with_index do |e, n|
  if (n % 3).zero?
    e.set_callback(proc { |event, type, data|
      puts [event, type, data].to_s
      puts "Event called"

      0
    }, 0xFFFFFFFF)
  end

  i = e.create_instance
  if ((n + 1) % 3).zero?
    i.set_callback(proc { |event, type, data|
      puts [event, type, data].to_s
      puts "Custom event callback called"

      0
    }, 0xFFFFFFFF)
  end
  i.start
end

System.get_parameter_description_list.each do |p|
  puts p
end

loop do
  System.update

  sleep(1.0 / 60.0)
end
