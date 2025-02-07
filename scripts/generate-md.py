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


headstr = delim + delim.join(files.keys()) + delim
linestr = delim + delim.join(["---"] * len(files.keys())) + delim
bodystr = ""

for row in rows:
    bodystr += delim + delim.join(row) + delim + "\n"

res = "\n".join((headstr, linestr, bodystr))

output = Path("README.md")
output.touch(exist_ok=True)



base_md = f"""
# Palettify - CLI-Tool for applying a color palette to an image.
The tool is written in Rust and works like image quantization, with the huge benefit of creating significantly smoother outputs.
It currently supports:
-  Images
-  Videos
-  Batch Processing
## Examples

{res}


##  Installation
### Manually
1. Clone the repo and cd into it:

   ```sh
   git clone https://github.com/bodenlosus/palettify-rust.git
   cd palettify-rust
   ```

3. Build via Cargo

   ```sh
   cargo build --profile "release"`
   ```

### Nix

```sh 
nix shell github:bodenlosus/palettify-rust
```

## Palettes
Palettes are saved in hex files using hexadecimal colors like so:

```
#1C1E26
#232530
#2E303E
#6F6F70
#9DA0A2
#CBCED0
#DCDFE4
#E3E6EE
#E93C58
#E58D7D
#EFB993
#EFAF8E
#24A8B4
#DF5273
#B072D1
#E4A382
```
There is no limit for the number of colors in a palette

## Usage
For **single images**

```sh
palettify -i input.png -o output.png -p palette.txt
```

For **Videos**:

``` sh
palettify -v -i input.mp4 -o output.mp4 -p palette.txt
```

For **Directories**:

```sh
palettify -d -i input/ -o output/ -p palette.txt
```

"""

output.write_text(base_md)