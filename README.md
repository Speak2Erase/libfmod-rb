# libfmod
High level Ruby bindings to the FMOD and FMOD Studio libraries.

# ModShot FMOD readme

Functions will always return an array of values (usually two) in the order of `[result, values...]`.
This means you can do `result, value = FMOD.some_func()`, which is pretty neat, right? If a result is not `FMOD_OK` (0) there will be no return value. Keep that in mind!

The FMOD bindings won't hold your hands either- You will need to clean up after yourself.
Because of the way the bindings work as well calling the same function twice will **NOT** return the "same" object. Fundamentally, it is the same object, as the Rust side object is the same, but it is a brand new object as far as ruby is concerned.

Because of this behavior, you can quite easily cause a memory leak by repeatedly storing an object returned in an array somewhere constantly loaded such that ruby will not garbage collect it.
i.e
```rb
array = []
100000.times do
    array << bank.get_event_list # Bad will memory leak!
end
```
So, be mindful of what you write! Luckily instances of objects from these bindings are very small so it's not a big deal if your code isn't perfect, but **PLEASE** do be mindful of this!
There is an `==` operator provided that will check if an object is the same for you as well. You can usually assign a value `nil` to get it garbage collected.

The bindings should generally line up with what's documented in the latest FMOD docs- although things like `FMOD_Studio_System_Create` are also aliased under `FMOD::Studio::System.new` instead. The bindings are closest to the `C#` bindings for FMOD.

One other thing to note is that with structs you need to be mindful of method chaining.
You can chain methods on the struct (like `struct.position.y = 15`) but if chained on the return vaue from a method it won't work.
```rb
# will not work!
eventinstance.get_3d_attributes[1].up.x = 15
# will work!
struct = eventinstance.get_3d_attributes[1]
struct.up.x = 15
eventinstance.set_3d_attributes(struct)
```

Also of note:
Callbacks run in an event thread. See [this](https://www.burgestrand.se/articles/asynchronous-callbacks-in-ruby-c-extensions/) for more information.

Due to weird behavior and needing to satisfy teh ruby garbage collector, user data is unsupported for now. I will return to it later.

I hope that's enough info to get you started!