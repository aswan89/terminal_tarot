# terminal_tarot
Toy Rust application for making tarot readings at the terminal

Run `terminal-tarot -h` to show flags and options. 

Application is built with several default source files that will be written to a default home directory if it does not exist 
or if the `-o` flag is supplied when run. 

Application will make readings using JSON formatted files located at `$HOME/.local/share/terminal_tarot` as source files. 
Custom files can be added if they match the fields defined in the default files.
