# ASIO on Windows

[ASIO](https://en.wikipedia.org/wiki/Audio_Stream_Input/Output) is an audio driver protocol by Steinberg. While it is available on multiple operating systems, it is most commonly used on Windows to work around limitations of WASAPI including access to large numbers of channels and lower-latency audio processing.

The [CPAL](https://crates.io/crates/cpal) crate provides an API that abstracts over multiple audio backends including ASIO. It allows for using the ASIO SDK as the audio host on Windows instead of WASAPI.

## Locating the ASIO SDK

The location of ASIO SDK is exposed to CPAL by setting the `CPAL_ASIO_DIR` environment variable.

The build script will try to find the ASIO SDK by following these steps in order:  

1. Check if `CPAL_ASIO_DIR` is set and if so use the path to point to the SDK.
2. Check if the ASIO SDK is already installed in the temporary directory, if so use that and set the path of `CPAL_ASIO_DIR` to the output of `std::env::temp_dir().join("asio_sdk")`.
3. If the ASIO SDK is not already installed, download it from <https://www.steinberg.net/asiosdk> and install it in the temporary directory. The path of `CPAL_ASIO_DIR` will be set to the output of `std::env::temp_dir().join("asio_sdk")`.

In an ideal situation you don't need to worry about this step.

## Preparing the build environment

1. `bindgen`, the library used to generate bindings to the C++ SDK, requires
   clang. **Download and install LLVM** from
   [here](http://releases.llvm.org/download.html) under the "Pre-Built Binaries"
   section. The version as of writing this is 17.0.1.
2. Add the LLVM `bin` directory to a `LIBCLANG_PATH` environment variable. If
   you installed LLVM to the default directory, this should work in powershell:

   ```powershell
   $env:LIBCLANG_PATH="C:\Program Files\LLVM\bin"
   ```

3. If you don't have any ASIO devices or drivers available, you can [**download
   and install ASIO4ALL**](http://www.asio4all.org/).
4. The build script assumes that Microsoft Visual Studio is installed. The script will try to find `vcvarsall.bat`
   and execute it with the right machine architecture regardless of the Microsoft Visual Studio version.
   If there are any errors encountered in this process which is unlikely,
   you may find the `vcvarsall.bat` manually and execute it with your machine architecture as an argument.
   The script will detect this and skip the step.

   A manually executed command example for 64 bit machines:

   ```powershell
   "C:\Program Files (x86)\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat" amd64
   ```

5. Select the ASIO host at the start of your program with the following code:

   ```rust,noplayground
   let host;
   #[cfg(target_os = "windows")]
   {
       host = cpal::host_from_id(cpal::HostId::Asio).expect("failed to initialise ASIO host");
   }
   ```

   If you run into compilations errors produced by `asio-sys` or `bindgen`, make
   sure that `CPAL_ASIO_DIR` is set correctly and try `cargo clean`.
6. Make sure to enable the `asio` feature when building CPAL:

   ```powershell
   cargo build --features "asio"
   ```

   or if you are using CPAL as a dependency in a downstream project, enable the
   feature like this:

   ```toml
   cpal = { version = "*", features = ["asio"] }
   ```
