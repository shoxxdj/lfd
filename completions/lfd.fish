# Fish completion script for lfd
# Installation:
#   cp lfd.fish ~/.config/fish/completions/

# Options
complete -c lfd -s q -l quiet -d "Quiet mode - only show command output"
complete -c lfd -s h -l help -d "Show help message"
complete -c lfd -s v -l version -d "Show version"

# Premier argument: fichier
complete -c lfd -n "__fish_is_first_arg" -F -d "Input file"

# Deuxième argument: variable name (pas de complétion)
# Troisième argument et suivants: commandes
complete -c lfd -n "test (count (commandline -opc)) -ge 3" -a "(__fish_complete_command)"
