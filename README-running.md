Feel free to bug me on the Discord if you want any help; I'm rlpowell.

First you need a system that has both `python3` and `cargo` working.  I personally use a Dockerfile that starts with `FROM docker.io/library/rust:[latest]`, but that's just me.

It should then be sufficient to just run `python3 server.py -- YourCharacterNameHere`.  This will launch an extremely simple Python server that will in turn run `cargo run` to run the Rust code in `src/main.rs`.

If possible you should test connectivity; typically this means running `curl [URL]` from wherever you're running Adventure Land.  If you're running everything on a single machine, it's quite likely to just work, but if your Rust bits are on a different machine from the Adventure Land bits, well, you're basically hosting a web server on the internet at that point, and there's a lot of things that can make that not work.  Hit me up on the Adventure Land discord if you need help at this stage.

Once you're sure the server is running and reachable, you need to copy [aland_side.js](aland_side.js) into your Adventure Land code window, replacing whatever's there (maybe make a copy first).  Change `URL` to match however it's going to reach your server.

PERFORMANCE NOTE: aland_side.js deliberately has a quarter second delay each time it loops.  This is very good for early trying stuff out, but you'll likely want to change that once you trust your code.

PERFORMANCE NOTE: Running this using caracAL (see note below about that) is, at least for me, *hugely* more performant than running my characters in a browser.  I have no idea why but I assume it's because the browser is having its fetch() calls throttled.  The difference is, like, probably at least a factor of 10x.

NOTE: Unless you want to set up your server with an SSL certificate, you *must* play Adventure Land with SSL turned off (that is: http instead of https), or it simply won't work; browser security will refuse to make the connection.

SECURITY NOTE: Whatever's at the other end of `URL` in your version of the Adventure Land side script can cause your browser to do anything it wants.  Make sure it's something you're in full control of.  (Although, to be fair, it's not like this is a common attack vector; anyone who tries to do this would have to know you use this system and have it in for you personally.)

At that point, running the script in Adventure Land *should* cause a bunch of logs of what the script is doing to show up in both your server window and in Adventure Land itself.  If not, most likely it can't connect.  You'll also get `TypeError: Failed to fetch` if there's a communication problem like that.

If you want to see something simple and easy to follow, to understand how the Rust itself works here, look at [main.rs.default_code](src/main.rs.default_code), which is a Rust version of exactly the same code that your character normally starts with by default.  Except without the `attack_mode` variable, because why?

More Than One Character
-----------------------

Once you're done testing with a single character, what you want to do is run `python3 server.py NNNN -- TankCharacter OtherCharacter ThirdCharacter`, where NNNN is a port number and the rest are character names in your party.  The tank character should always be listed first, the order of the rest doesn't matter.  You need to run it once for each character, on different ports.  The run multiple instances of AdventureLand pointing each instance to a different port.  It doesn't matter which character gets which port; the system will figure out which character it is, it just needs the names of all the characters to put the party together.

caracAL
-------

SPECIAL NOTE ABOUT caracAL: It *is* possible to make this work with caracAL but (again unless you set up an https endpoint for your server), you'll need to modify aland_side.js (once you copy it into caracAL's CODE/ directory), because the CSRF protection doesn't actually apply here, so change:

```
if( window.location.href.includes("https") ) {
    debug("You must switch the game to http.");
    mystuff();
} else {
    mystuff();
}
```

to just:

```
mystuff();
```
