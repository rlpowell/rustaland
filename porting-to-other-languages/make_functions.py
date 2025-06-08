import re
import urllib.request
import sys

url = 'https://raw.githubusercontent.com/kaansoral/adventureland/main/js/runner_functions.js'

response = urllib.request.urlopen(url)
html_content = response.read().decode('utf-8')

print("""use crate::helpers::*;
use serde_json::Value;
""")

for line in html_content.splitlines():
    # Python can't return multiple matches of a repeated capture group, so we have to break it up in two passes
    match = re.search(r"^function +([^(]+)", line)
    if match:
        func_name = match.group(1)
        # Fix keyword issues
        func_name = re.sub(r"^move$", "almove", func_name)
        # print(func_name)
        args_inp = re.sub(r"^function +([^,(]+)\(([^)]*)\)(?: .*|$)", "\\2", line)

        # print(args_inp)
        args_f_text = ""
        args_let_text = ""
        args_send_text = ""
        if args_inp:
            args_match = re.findall(r"[^,)]+", args_inp)
            # Fix keyword issues
            args_match = list(map(lambda x: re.sub(r"^fn$", "func", x), args_match))
            # print(args_match)

            args_f_text = ", ".join(map(lambda x: f"{x}: Value", args_match))
            # print(args_f_text)
            args_let_text = "\n    ".join(map(lambda x: f"let {x}_string = {x}.to_string();", args_match)) + "\n\n    "
            # print(args_let_text)
            args_send_text = ", ".join(map(lambda x: f"{{{x}_string}}", args_match))
            # print(args_send_text)

        print(f"""
#[allow(dead_code)]
pub fn {func_name}({args_f_text}) -> Result<Value, Box<dyn std::error::Error>> {{
    #[allow(clippy::useless_format)]
    {args_let_text}do_the_thing(format!("{func_name}({args_send_text});"))
}}
""")
