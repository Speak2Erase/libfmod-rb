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
    rand(1..5).to_i.times do |i|
        e.create_instance
    end
end

$call_count = 0
$old_call_count = 0
$prc = proc { |a, b, c|
    $call_count += 1
    puts [a, b, c].inspect

    nil
}

GC.disable

System.set_callback($prc, 0xFFFFFFFF)

loop do
    System.update

    if $old_call_count != $call_count 
        puts "I've been called #{$call_count} time(s)"
        $old_call_count = $call_count
    end

    sleep(1.0 / 60.0)

    # sleep(1.0 / 60.0)
end