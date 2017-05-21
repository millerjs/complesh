completion_function() {
    path=$(mktemp)
    CDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
    "${CDIR}/target/release/complesh" -o "${path}" -i "${COMP_WORDS[COMP_CWORD]}"
    COMPREPLY[0]=$(cat "${path}")
    rm "${path}"
}

test () {
    echo "test '$@'"
}

COMPLESH_COMMANDS="ls cd cat wc touch cp mv rm test"

for command in $COMPLESH_COMMANDS
do
    complete -F completion_function -o nospace $command
done
