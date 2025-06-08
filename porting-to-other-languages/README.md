How This Works
==============

My goal was to make this a very, very easy-to-get-started method for running whatever language you want as your client for Adventure Land.

My goals did *not* include "performant" or "reasonable" or "sane".  :)

So, on the Adventure Land side, the entire script is a loop around "fetch from this URL and eval whatever it gives you and return the results".

The other end of the fetch is a tiny python script that basically is just "turn the web-based conversations with the game client into lines of text with a known structure in json, and feed them to a given program as stdin/stdout, feeding the replies back to the game client".

It's all a bit weird because the command flow is "tell the game client want to do", but the data flow is "the game client asks you what to do", so the game client layer and the python layer both feel a bit backwards.

But at the bottom is a Rust program that just sends constructed (and the construction is extremely simple) javascript on one line and reads the replies on the next line.

The only complex thing is that the actual json that's passed around is `[NUM, STRING]`, where string is like `use_hp_or_mp()` or `false` or `{"x":-150,"y":400,"width":26,"height":34,...}`, and NUM is a continuously incrementing number.

The only reason that number exists is so that if a request is lost, the system will bail, so you can't end up with partially-applied scripts.

The number system is currently very stupid; in particular, if you restart either side, you must restart both sides, because if for example you are running this way and you Disengage and then Engage on the Adventure Land side, the Python layer will say "wait I'm seeing sequence number 1 instead of 87 or whatever" and bail.

Other Options
=============

The obvious way to do this properly would be WebSockets, but my primary goal here was absolute simplicity.  In particular, from the point of view of all 3 sides, everything here is 100% synchronous: commands are executed in order, and nothing happens until a command completes.  No concurrency of any kind.  That doesn't really work with WebSockets, is my understanding.  Also I've never used them, so it would have been more work for me.

An Example Flow
===============

Note that for porting this to another language you only care about what the Rust side does; the rest is just here for general interest.

You can see the whole flow by turning on the entirely manual debug flag at the top of of `server.py`.  The Adventure Land and Rust sides also have such flags but just turning on the Python one is sufficient.

One thing to note is that every communication happens in two parts:
- A GET from the Adventure Land side; this says "hey I don't have anything to say but I'm here and ready for a command"; the "data" back from the "server" is the text of the next command to run.  So for this part, the only data from the Adventure Land side is the sequence number, and even though the Adventure Land side initiated the communication, the flow is from the Rust side as it responds with the command.
- A POST from the Adventure Land side with the return value from the command.  The Python side responds to this with nothing but headers.  So the data flow is entirely towards the Rust side.

This allows us to pretend that "the Rust script sends a command and then the Adventure Land script responds", even though in fact the Adventure Land script has to do all the initiating of communications.

Here's an actual flow:

- Adventure Land To Python, via HTTP to your URL: `GET /1`
    - This gives the current sequence number and shows readiness
- Python To Rust, via stdout/stdin: `[1, "ready"]`
- Rust To Python, via stdout/stdin: `[2,"use_hp_or_mp();"]`
- Python returns that string to Adventure Land as the contents of the GET reply.
- Adventure Land To Python: `POST [3,{}]` (not actually how POST works but close enough for our purposes here)
- Python To Rust, via stdout/stdin: `[3,{}]`

The flow ends at that point, and the Rust program blocks waiting for the next line on stdin.

A more complex flow, as the Python server reports it without all the commentary:

```
python-to-subproc: [19, "ready"]
subproc-to-python: [20,"get_nearest_monster({\"max_att\":120,\"min_xp\":100});"]
python-to-aland: [20,"get_nearest_monster({\"max_att\":120,\"min_xp\":100});"]
aland-to-python: POST [21,{"x":-63.64731603373824,"y":716.4170137267882,"width":23,"height":22,...}]
```

So How Do I Do This Myself For Another Language?
================================================

There's two parts to this:

1. Making a copy of the standard game functions in your language.
2. Writing enough helper functions to make those work.

Making the functions is just a matter of changing [make_functions.py](./make_functions.py) to emit your language instead of Rust.  You can look at [aland_functions.rs.generated](../src/aland_functions.rs.generated) to see what the generated output looked like originally for Rust, and I recommend you keep a similar copy for the reasons explained at the top of that file.

You'll notice, though, that those functions are just "call another function with the given string", like what the `is_npc` function *actually does* is it calls `handle_flow` with a string like `is_npc([argument_as_json])`.

That `handle_flow` helper function is the thing that interacts with the two part flow system described above.  You will just need to port that function, and any functions it calls, to your language.  It is not complicated.

That should be enough to let you start writing code that uses those functions to drive your Adventure Land character.

About Entities
==============

The version you'll get if you just do the above will have functions that pass entire entities (like your own character object, for example) back to Adventure Land.  That's wasteful, and also isn't guaranteed to work because those objects, in Adventure Land itself, contain inheritance loops, so what's sent to us isn't actually complete.  This is done automatically; normal calls to these functions will jsonify whatever you pass them as arguments.

You can edit functions to instead refer to the objects via variables already present in the Adventure Land environment.  There's never any reason to pass the `character` object back, for example; you can just replace it with the string `character` in the commands you send back and it'll work that way.  Also, most nearby entities will be in `parent.entities`, so that can be used to.

`deref_entity` in [helpers.rs](../src/helpers.rs) does this exact thing, and a bunch of functions in [aland_functions.rs](../src/aland_functions.rs) use it, so as a thing to work one once you have the basics, consider doing that for your language too.
