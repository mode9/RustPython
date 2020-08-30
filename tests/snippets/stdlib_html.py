import html

assert html.escape("&") == "&amp;"
assert html.escape("<") == "&lt;"
assert html.escape(">") == "&gt;"
assert html.escape('"') == "&quot;"
assert html.escape("\'") == "&#x27;"

assert html.escape('"', False) == '"'
assert html.escape("\'", False) == "\'"
