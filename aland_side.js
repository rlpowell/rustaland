LASTNUM=0
DEBUG=false
URL="http://localhost:50555/"

function debug(string) {
    if(DEBUG) {
        game_log(string);
        console.log(string);
    }
}

async function mystuff() {
    while (true) {
        debug("in while");

        LASTNUM = LASTNUM + 1

        try {
            response = await fetch(URL + LASTNUM);
            // Check if the request was successful (status code 200-299)
            if (!response.ok) {
                //game_log(response);
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            debug("response: ");
            debug(response);
            data = await response.json();

            debug("data: ");
            debug(data); // Print the JSON data to the console

            newnum = data[0];
            if( newnum != LASTNUM + 1 ) {
                throw new Error("Expected sequence number " + (LASTNUM + 1) + " but got " + newnum);
            } else {
                LASTNUM = newnum;
            }
            command = data[1];
            output = ""
            try {
                // Some things are easier if this is `await eval`
                // instead, because it waits for them to actually
                // complete, but it eans that we can't do *literally
                // anything* while running smart_move, which sucks.
                //
                // If wanting to wait for stuff becomes a problem
                // later, could try an if statement between await
                // and non-await based on the contents of the
                // command.
                output = eval(command);
            } catch (error) {
                output = "{}"
                game_log('command error:' + parent.game_stringify(error));
            }

            debug("output:");
            debug(parent.game_stringify(output));


            try {
                LASTNUM = LASTNUM + 1;

                output_str=""
                if(typeof(output) == 'string') {
                    output_str="[" + LASTNUM + ", " + output + "]"
                } else {
                    output_str=parent.game_stringify([LASTNUM, output])
                }

                await fetch(URL, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: output_str,
                })
            } catch (error) {
                game_log('reply error:' + parent.game_stringify(error));
            }
        } catch (error) {
            game_log('initial error:' + parent.game_stringify(error));
        }

        debug("end of while");

        // NOTE: You will almost certainly want to lower this 250
        // once your code is working reliably
        await new Promise(r => setTimeout(r, 250));
    }
}

if( window.location.href.includes("https") ) {
    game_log("You must switch the game to http.");
} else {
    mystuff();
}
