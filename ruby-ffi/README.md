```
# setup
$ bundle
$ cargo build --release
## on mac os x
$ cp ./target/libmulti_bind_rb.dylib ./lib/libmulti_bind_rb.dylib
## on linux
$ cp ./target/libmulti_bind_rb.so ./lib/libmulti_bind_rb.so
$ sed -i "s/dylib/so/" ./lib/multi_bind.rb

# execute
$ bundle exec ruby -Ilib ./main.rb
>> add => 3
>> unwrap_future => 10
```
