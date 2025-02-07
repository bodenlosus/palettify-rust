for palette in ./examples/palettes/*
do
    ./target/release-lto/palettify -i ./examples/src-images/ -p $palette -o ./examples/images/$(basename $palette) -d -r 720p -e 14
done
