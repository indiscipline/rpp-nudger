@echo off
set i=%PATH%
set NAME=rpp-nudger
if not exist latest mkdir latest
echo --------------Compiling x32--------------
set PATH=%i%;c:\mingw-w64\i686-6.2.0-win32-dwarf-rt_v5-rev1\mingw32\bin\
rem set RUSTUP_TOOLCHAIN=nightly-i686-pc-windows-gnu
cargo build --target=i686-pc-windows-gnu --release
copy target\i686-pc-windows-gnu\release\%NAME%.exe latest\%NAME%-i686.exe

echo --------------Compiling x64--------------
set PATH=%i%;c:\mingw-w64\x86_64-6.2.0-win32-seh-rt_v5-rev1\mingw64\bin\
rem set RUSTUP_TOOLCHAIN=nightly-x86_64-pc-windows-gnu
cargo build --target=x86_64-pc-windows-gnu --release
copy target\x86_64-pc-windows-gnu\release\%NAME%.exe latest\%NAME%-x86_64.exe

set PATH=%i%