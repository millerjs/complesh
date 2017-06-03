_complesh() {
    path=$(mktemp)
    complesh -o "${path}" -i "${COMP_WORDS[COMP_CWORD]}"
    COMPREPLY[0]=$(cat "${path}")
    rm "${path}"
}

_complesh_choices() {
    path=$(mktemp)
    complesh -o "${path}" -i "${COMP_WORDS[COMP_CWORD]}" -c "$@"
    COMPREPLY[0]=$(cat "${path}")
    rm "${path}"
}

if [ -z "$COMPLESH_COMMANDS" ]; then
    COMPLESH_COMMANDS="ls cd cat wc touch cp mv rm open"
fi

for command in $COMPLESH_COMMANDS
do
    complete -F _complesh -o nospace $command
done
