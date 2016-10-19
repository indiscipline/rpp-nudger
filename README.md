# reaper-item-nudger

[![License](https://img.shields.io/badge/license-GPLv2-blue.svg)](https://github.com/Indiscipline/rpp-nudger/blob/master/LICENSE)

Command line utility for editing Reaper DAW project files and nudging audio content in all items to a specific amount of samples. Program just prints a modified project so user should direct the output to a desired file. 
It's not a proper parser for a project, just a simple Regular Expression needlessly wrapped in an application instead of a script.

## Usage
rpp-nudger <INPUT> <samples> > output.rpp

Full help available on `--help` switch.

Latest Windows **binaries** are luckily provided in [Releases](https://github.com/indiscipline/rpp-nudger/releases) via Appveyor.

## How to build
Developed on Rust Nightly, but should build with stable or beta as well.

To build the code, run:

```
$ cargo build --release
```

and the executable will be in `target/release/rpp-nudger`.

## Contributing ##
This is an enthusiast project, so any help and/or critique will be much appreciated.

Feel free to file a bug report or feature request via [Issues](https://github.com/Indiscipline/rpp-nudger/issues).

## License ##
**rpp-nudger** licensed under GNU General Public License version 2 or later;

See `LICENSE` file for full details.