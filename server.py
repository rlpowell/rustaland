import http.server
import json
import socketserver
import socket
import sys
import subprocess

debug=False

PORT = 50555

LASTNUM = 0

def send_to_subproc(string):
    if debug:
        print("python-to-subproc: " + string)
    my_subprocess.stdin.write(string + "\n")
    my_subprocess.stdin.flush()
    if debug:
        print("python-to-subproc done")

def get_from_subproc():
    if debug:
        print("subproc-to-python start")
    string = my_subprocess.stdout.readline().rstrip()
    if debug:
        print("subproc-to-python: " + string)
    return string

def send_to_aland(handler, string):
    if debug:
        print("python-to-aland: " + string)
    handler.wfile.write(string.encode())

class MyTCPServer(socketserver.TCPServer):
    def server_bind(self):
        self.socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        self.socket.bind(self.server_address)

class MyHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, format, *args):
        pass  # Suppress all logging

    def do_OPTIONS(self):
            self.send_response(200)
            self.send_header("Content-type", "text/html")
            self.send_header("Access-Control-Allow-Origin", "*")
            self.send_header("Access-Control-Allow-Headers", "*")
            self.send_header("Allow", "POST,OPTIONS,HEAD,GET,TRACE")
            self.end_headers()

    def do_GET(self):
        try:
            global LASTNUM

            global my_subprocess

            self.send_response(200)
            self.send_header("Content-type", "text/html")
            self.send_header("Access-Control-Allow-Origin", "*")
            self.send_header("Access-Control-Allow-Headers", "*")
            self.send_header("Allow", "POST,OPTIONS,HEAD,GET,TRACE")
            self.end_headers()

            if debug:
                print(f"aland-to-python: GET {self.path}")

            newnum = int(self.path[1:])
            # Here only, a sequence reset is allowed
            if newnum != LASTNUM + 1:
                if newnum == 1:
                    print("Sequence number reset");
                else:
                    raise ValueError(f"FATAL: Call number not in sequence, expected {LASTNUM + 1} got {newnum}.")

            LASTNUM = newnum

            send_to_subproc(f"[{LASTNUM}, \"ready\"]")

            try:
                raw_inp = get_from_subproc()
                inp = json.loads(raw_inp)
            except Exception as e:
                raise ValueError(f"Failed to parse json {raw_inp}, error {e}")

            newnum = int(inp[0])
            if newnum != LASTNUM + 1:
                raise ValueError(f"FATAL: Call number not in sequence, expected {LASTNUM + 1} got {newnum}.")

            LASTNUM = newnum

            send_to_aland(self, raw_inp)
        except Exception as e:
            sys.exit(e)

    def do_POST(self):
        global LASTNUM

        # retrieve data from input stream
        length = int(self.headers['Content-Length'])
        raw_body = self.rfile.read(length).decode('utf-8')

        if debug:
            print(f"aland-to-python: POST {raw_body}")

        body = json.loads(raw_body);

        newnum = int(body[0])
        if newnum != LASTNUM + 1:
            sys.exit(f"FATAL: Call number not in sequence, expected {LASTNUM + 1} got {newnum}.")

        LASTNUM = newnum

        send_to_subproc(raw_body)

        self.send_response(200)
        self.send_header("Content-type", "text/html")
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Headers", "*")
        self.send_header("Allow", "POST,OPTIONS,HEAD,GET,TRACE")
        self.end_headers()

if len(sys.argv) > 1:
    PORT = int(sys.argv[1])

my_subprocess = subprocess.Popen(['cargo', 'run', *sys.argv[2:]], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)

with MyTCPServer(("", PORT), MyHandler) as httpd:
    print(f"Serving at port {PORT}")
    try:
        httpd.serve_forever()
    except Exception as e:
        print("exiting {e}")
    finally:
        httpd.server_close()
        httpd.socket.close()
