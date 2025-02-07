from pathlib import Path

files:dict[str, list[Path]] = dict()

imgdir = Path("./examples/images")

def to_gh_image(pth:Path):
    url = "https://github.com/bodenlosus/palettify-rust/blob/master/"
    url += str(pth).lstrip("./")
    return f"<img src=\"{url}\">"
    

for file in imgdir.glob("*"):
    if not file.is_dir():
        continue
    
    name = file.name
    
    files[name] = []
    
    for subfile in file.glob("*"):
        files[name].append(subfile)

rows:list[list[Path]]= []

for fs in files.values(): 
    for i in range(len(fs)):
        if i >= len(rows):
            rows.append([])
        rows[i].append(to_gh_image(fs[i]))

delim = "|"
base_md = ""

headstr = "|" + delim.join(files.keys()) + "|"
linestr = "|" + delim.join(["---"] * len(files.keys())) + "|"
bodystr = ""

for row in rows:
    bodystr += "|" + delim.join(row) + "|\n"

res = "\n".join((headstr, linestr, bodystr))

output = Path("ex.md")
output.touch(exist_ok=True)

output.write_text(res)
