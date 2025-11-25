# Bash completion script for lfd
# Installation: 
#   sudo cp lfd-completion.bash /etc/bash_completion.d/lfd
#   or: source lfd-completion.bash in your ~/.bashrc

_lfd_completion() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    
    # Options disponibles
    opts="-q --quiet -h --help -v --version"
    
    # Position dans la commande
    local pos=1
    local quiet_mode=0
    
    # Détecter si -q est présent
    for word in "${COMP_WORDS[@]:1:$COMP_CWORD}"; do
        if [[ "$word" == "-q" || "$word" == "--quiet" ]]; then
            quiet_mode=1
            break
        fi
    done
    
    # Ajuster la position si mode quiet
    if [[ $quiet_mode -eq 1 ]]; then
        pos=$((COMP_CWORD - 1))
    else
        pos=$COMP_CWORD
    fi
    
    case $pos in
        1)
            # Premier argument: options ou fichier
            if [[ ${cur} == -* ]]; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            else
                # Complétion de fichiers
                COMPREPLY=( $(compgen -f -- ${cur}) )
            fi
            ;;
        2)
            # Deuxième argument: nom de variable (pas d'autocomplétion)
            COMPREPLY=()
            ;;
        3)
            # Troisième argument et suivants: complétion de commandes
            # Compléter avec les commandes disponibles
            COMPREPLY=( $(compgen -c -- ${cur}) )
            ;;
        *)
            # Arguments suivants: complétion de fichiers et commandes
            if [[ ${cur} == -* ]]; then
                # Si ça commence par -, proposer des options communes
                local common_opts="-h --help -v --version -f --file -o --output -i --input"
                COMPREPLY=( $(compgen -W "${common_opts}" -- ${cur}) )
            else
                # Sinon, complétion de fichiers et commandes
                COMPREPLY=( $(compgen -f -c -- ${cur}) )
            fi
            ;;
    esac
    
    return 0
}

# Activer la complétion pour lfd
complete -F _lfd_completion lfd
