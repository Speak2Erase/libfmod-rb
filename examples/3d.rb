#! /usr/bin/ruby
# frozen_string_literal: true

require "libfmod"
require "io/console"

SCREEN_WIDTH = 50
SCREEN_HEIGHT = 16

$buffer = " " * (SCREEN_WIDTH + 1) * (SCREEN_HEIGHT + 1)
SCREEN_HEIGHT.times do |i|
  $buffer[(SCREEN_WIDTH + 1) * i] = "\n"
end
$current_screen_position = -1

def get_character_idx(position)
  row = (-position.z + (SCREEN_HEIGHT / 2))
  col = (position.x + (SCREEN_WIDTH / 2))

  if row.positive? && row < SCREEN_HEIGHT && col.positive? && col < SCREEN_WIDTH
    (row * (SCREEN_WIDTH + 1)) + col
  else
    -1
  end
end

def update_screen_position(position)
  if $current_screen_position.positive?
    $buffer[$current_screen_position] = " "
    $current_screen_position = -1
  end

  origin = FMOD::Struct::Vector.new(0.0, 0.0, 0.0)
  idx = get_character_idx(origin)
  $buffer[idx] = "^"

  idx = get_character_idx(position)
  return unless idx != -1

  $buffer[idx] = "o"
  $current_screen_position = idx
end

print "\e[?25h"

Thread.abort_on_exception = true

System = FMOD::Studio::System.create
System.init(64, 0, 0)

Master = System.load_bank_file("media/Master.bank", 0)
Strings = System.load_bank_file("media/Master.strings.bank", 0)
Vehicles = System.load_bank_file("media/Vehicles.bank", 0)

event_description = System.get_event("event:/Vehicles/Ride-on Mower")

instance = event_description.create_instance
rpm = 650
instance.set_parameter_by_name("RPM", rpm, false)
instance.start

attributes, _vector = System.get_listener_attributes(0)
attributes.forward.z = 1.0
attributes.up.y = 1.0

System.set_listener_attributes(0, attributes, nil)

attributes.position.z = 2.0
instance.set_3d_attributes(attributes)

while $char != "q"
  print "\e[H"
  print "\e[J"

  System.update

  case $char
  when "w"
    attributes.position.z += 1.0
    instance.set_3d_attributes(attributes)
  when "a"
    attributes.position.x -= 1.0
    instance.set_3d_attributes(attributes)
  when "s"
    attributes.position.z -= 1.0
    instance.set_3d_attributes(attributes)
  when "d"
    attributes.position.x += 1.0
    instance.set_3d_attributes(attributes)
  when "e"
    rpm -= 5
    instance.set_parameter_by_name("RPM", rpm, false)
  when "r"
    rpm += 5
    instance.set_parameter_by_name("RPM", rpm, false)
  end

  update_screen_position(attributes.position)
  print "==================================================\e[K\n"
  print "Event 3D Example.\e[K\n"
  print "Adapted from the official FMOD example\e[K\n"
  print "==================================================\e[K\n"
  print "#{$buffer}\e[K\n"
  print "Use the arrow keys (w, a, s, d) to control the event position\e[K\n"
  print "Use e and r to change RPM\e[K\n"
  print "RPM: #{rpm}\e[K\n"
  print "Press q to quit\e[K\n"

  sleep(1.0 / 30.0)

  $char = $stdin.getch
end

System.release
