#! /usr/bin/ruby
# frozen_string_literal: true

require "libfmod"

puts FMOD::EventThread

System = FMOD::Studio::System.create
System.init(64, 0, 0)

Master = System.load_bank_file("media/Master.bank", 0)
Strings = System.load_bank_file("media/Master.strings.bank", 0)
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

loop do
  System.update

  sleep(1.0 / 60.0)
end
