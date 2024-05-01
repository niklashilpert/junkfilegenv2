#!/bin/bash
if [ `id -u` -ne 0 ]; then
    echo Root access is required to add the file to /bin.
    exit
fi

name="gen-junkfile"

while [[ $# -gt 0 ]]
do
    case $1 in 
        -n|--name)
            name="$2"
            shift
            shift
            ;;
        -s|--source)
            src_path="$2"
            shift
            shift
            ;;
        *)
            echo Unrecognized commandline argument $1. 
            echo Exiting...
            exit
            ;;
    esac
done

target_path="/bin/$name"

if [ -f "$target_path" ]; then
    echo A command with the name \'$name\' already exists.
    echo Exiting... 
    exit
fi

if [ -v src_path ]; then
    chmod +x "$src_path"
    cp "$src_path" "$target_path" 
    echo The executable was added to the \'/bin\' folder and can now be executed with the \'$name\' command.
else
    echo No source path was supplied. Rerun the script with -s \<path to executable\>.
fi

