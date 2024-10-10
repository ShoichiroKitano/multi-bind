require 'ffi'

module MultiBind
  extend FFI::Library
  ffi_lib './target/release/deps/libmulti_bind_rb.dylib'

  attach_function :add, [:uint64, :uint64], :uint64
  attach_function :unwrap_future, [], :uint64
end
