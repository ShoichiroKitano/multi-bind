```
# setup
$ sed -I "s/XXX/your_ruby-dev_path/" ./build.rs
$ cargo build --release
$ cp ./target/release/libmulti_bind_rb.dylib ./lib/multi_bind_rb.bundle

# execute
$ ruby -Ilib ./lib/main.rb
>> add => 3
>> unwrap_future => 10
```
