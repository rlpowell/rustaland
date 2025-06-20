Feel free to bug me on the Discord if you want any help; I'm rlpowell.

First you need a system that has both `python3` and `cargo` working.  I personally use a Dockerfile that starts with `FROM docker.io/library/rust:[latest]`, but that's just me.

It should then be sufficient to just run `python3 server.py`.  This will launch an extremely simple Python server that will in turn run `cargo run` to run the Rust code in `src/main.rs`.

If possible you should test connectivity; typically this means running `curl [URL]` from wherever you're running Adventure Land.  If you're running everything on a single machine, it's quite likely to just work, but if your Rust bits are on a different machine from the Adventure Land bits, well, you're basically hosting a web server on the internet at that point, and there's a lot of things that can make that not work.  Hit me up on the Adventure Land discord if you need help at this stage.

Once you're sure the server is running and reachable, you need to copy [aland_side.js](aland_side.js) into your Adventure Land code window, replacing whatever's there (maybe make a copy first).  Change `URL` to match however it's going to reach your server.

PERFORMANCE NOTE: aland_side.js deliberately has a quarter second delay each time it loops.  This is very good for early trying stuff out, but you'll likely want to change that once you trust your code.

NOTE: Unless you want to set up your server with an SSL certificate, you *must* play Adventure Land with SSL turned off (that is: http instead of https), or it simply won't work; browser security will refuse to make the connection.

SECURITY NOTE: Whatever's at the other end of `URL` in your version of the Adventure Land side script can cause your browser to do anything it wants.  Make sure it's something you're in full control of.  (Although, to be fair, it's not like this is a common attack vector; anyone who tries to do this would have to know you use this system and have it in for you personally.)

At that point, running the script in Adventure Land *should* cause a bunch of logs of what the script is doing to show up in both your server window and in Adventure Land itself.  If not, most likely it can't connect.  You'll also get `TypeError: Failed to fetch` if there's a communication problem like that.

If you want to see something simple and easy to follow, to understand how the Rust itself works here, look at [main.rs.default_code](src/main.rs.default_code), which is a Rust version of exactly the same code that your character normally starts with by default.  Except without the `attack_mode` variable, because why?

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
