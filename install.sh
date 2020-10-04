#!/bin/bash

printf '\nThis will install and configure Simple DirectMedia Layer (SDL 2.0) on your machine: https://www.libsdl.org/index.php.\n'
printf '\nIf you wish to continue, carry on...\n'

printf "\nWhat OS do you use?\n"
select lm in "linux" "macOS"; do
    case $lm in
        linux )
            printf "\nWhat distro are you using?\n"
            select dstro in "ubuntu/debian" "fedora/centOS" "arch_linux" "other"; do
                case $dstro in
                    ubuntu/debian )
                        printf "\nInstalling libsdl2-dev via apt...\n"
                        sudo apt install libsdl2-dev
                    exit;;
                    fedora/centOS )
                        printf "\nInstalling SDL2-devel via yum...\n"
                        sudo yum install SDL2-devel
                    exit;;
                    arch_linux )
                        printf "\nInstalling sdl2 via pacman...\n"
                        sudo pacman -S sdl2
                    exit;;
                    other )
                        printf "\nPlease check the official installation page for SDL 2.0: https://www.libsdl.org/download-2.0.php\n"
                    exit;;
                esac
            done;
        exit;;
        macOS )
            printf "\nInstalling sdl2 via Homebrew...\n"
            brew install sdl2
            
            printf "\nWhat shell do you use?\n"
            select bzo in "bash" "zsh" "other"; do
                case $bzo in
                    bash )
                        echo 'export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"' >> ~/.bash_profile;
                        printf "\nAdded 'export LIBRARY_PATH=\"$LIBRARY_PATH:/usr/local/lib\"' to your ~/.bash_profile!\n"
                    exit;;
                    zsh )
                        echo 'export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"' >> ~/.zshrc;
                        printf "\nAdded 'export LIBRARY_PATH=\"$LIBRARY_PATH:/usr/local/lib\"' to your ~/.zshrc!\n"
                    exit;;
                    other )
                        printf "\nPlease add 'export LIBRARY_PATH=\"$LIBRARY_PATH:/usr/local/lib\"' to your shell profile!\n";
                    exit;;
                esac
            done; exit;
        exit;;
    esac
done
