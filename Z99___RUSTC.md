rustc -C debuginfo=0 -C link-arg=/DEBUG:NONE <FILESRC> -o <EXEDST>

* Within same dir
rustc -C debuginfo=0 -C link-arg=/DEBUG:NONE --test <FILE> && ./<FILE>.exe