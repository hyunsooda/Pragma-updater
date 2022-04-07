How to build
```
cargo build
```

How to run
```
./target/debug/pragma-updater --dir <DIR_PATH> --version="^0.8.0"
```

Configurable option
```
./target/debug/pragma-updater --help
USAGE:
    pragma-updater [OPTIONS] --dir <DIR> --version <VERSION>

OPTIONS:
    -d, --dir <DIR>            directory path
    -h, --help                 Print help information
    -l, --license              Write this option if you want to add SPDX license at the top of the
                               source file
        --overwrite            Write this option if you want to overwrite it
        --revert               Write this option if you want to revert the editted files
        --version <VERSION>    Write a version to be editted
```
