#compdef lfd
# Zsh completion script for lfd
# Installation:
#   Place this file in your $fpath as _lfd
#   Usually: ~/.zsh/completions/_lfd or /usr/share/zsh/site-functions/_lfd
#   Then run: compinit

_lfd() {
    local line state

    _arguments -C \
        '(-q --quiet)'{-q,--quiet}'[Quiet mode - only show command output]' \
        '(-h --help)'{-h,--help}'[Show help message]' \
        '(-v --version)'{-v,--version}'[Show version]' \
        '1:file:_files' \
        '2:variable name:()' \
        '3:command:_command_names' \
        '*::command arguments:_normal'
}

_lfd "$@"
