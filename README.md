# PPM Writer

Writes image data from memory to disk in PPM 3 format

## Build from source

```sh
git clone git+https://github.com/Num0Programmer/ppm-writer.git
cd ppm-writer
cargo build --release
```

## Run the program

From within the repository
```sh
./target/release/ppmw
```

You can also opt to make an alias to the binary
```sh
alias ppmw="$PWD/target/release/ppmw"
```

Then, you can use the alias to run the program from anywhere in the system
```sh
ppmw
```

Alternatively, you can copy the binary to /usr/local/bin
```sh
cp target/release/ppmw
```

This also allows you to run the program from anywhere without the need to maintain an alias
