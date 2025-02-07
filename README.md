
# Palettify - CLI-Tool for applying a color palette to an image.
The tool is written in Rust and works like image quantization, with the huge benefit of creating significantly smoother outputs.
It currently supports:
-  Images
-  Videos
-  Batch Processing
## Examples

|tokyo-night-dark|nord|gruvbox-dark-hard|catppuccin-mocha|horizon-dark|material|
|---|---|---|---|---|---|
|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/tokyo-night-dark/palettify-s9uee41d5ece1.png">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/nord/palettify-s9uee41d5ece1.png">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/gruvbox-dark-hard/palettify-s9uee41d5ece1.png">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/catppuccin-mocha/palettify-s9uee41d5ece1.png">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/horizon-dark/palettify-s9uee41d5ece1.png">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/material/palettify-s9uee41d5ece1.png">|
|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/tokyo-night-dark/palettify-moe-wanders-the-rox-to-dueling-peaks-web.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/nord/palettify-moe-wanders-the-rox-to-dueling-peaks-web.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/gruvbox-dark-hard/palettify-moe-wanders-the-rox-to-dueling-peaks-web.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/catppuccin-mocha/palettify-moe-wanders-the-rox-to-dueling-peaks-web.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/horizon-dark/palettify-moe-wanders-the-rox-to-dueling-peaks-web.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/material/palettify-moe-wanders-the-rox-to-dueling-peaks-web.jpg">|
|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/tokyo-night-dark/palettify-shahab-alizadeh-nebula.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/nord/palettify-shahab-alizadeh-nebula.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/gruvbox-dark-hard/palettify-shahab-alizadeh-nebula.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/catppuccin-mocha/palettify-shahab-alizadeh-nebula.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/horizon-dark/palettify-shahab-alizadeh-nebula.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/material/palettify-shahab-alizadeh-nebula.jpg">|
|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/tokyo-night-dark/palettify-10-12.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/nord/palettify-10-12.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/gruvbox-dark-hard/palettify-10-12.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/catppuccin-mocha/palettify-10-12.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/horizon-dark/palettify-10-12.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/material/palettify-10-12.jpg">|
|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/tokyo-night-dark/palettify-ashim-d-silva-WeYamle9fDM-unsplash.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/nord/palettify-ashim-d-silva-WeYamle9fDM-unsplash.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/gruvbox-dark-hard/palettify-ashim-d-silva-WeYamle9fDM-unsplash.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/catppuccin-mocha/palettify-ashim-d-silva-WeYamle9fDM-unsplash.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/horizon-dark/palettify-ashim-d-silva-WeYamle9fDM-unsplash.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/material/palettify-ashim-d-silva-WeYamle9fDM-unsplash.jpg">|
|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/tokyo-night-dark/palettify-s-b-vonlanthen-A8iLzX6OddM-unsplash.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/nord/palettify-s-b-vonlanthen-A8iLzX6OddM-unsplash.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/gruvbox-dark-hard/palettify-s-b-vonlanthen-A8iLzX6OddM-unsplash.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/catppuccin-mocha/palettify-s-b-vonlanthen-A8iLzX6OddM-unsplash.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/horizon-dark/palettify-s-b-vonlanthen-A8iLzX6OddM-unsplash.jpg">|<img src="https://github.com/bodenlosus/palettify-rust/blob/master/examples/images/material/palettify-s-b-vonlanthen-A8iLzX6OddM-unsplash.jpg">|



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

