

completion_function() {
    path=$(mktemp)
    /Users/jmiller/jsm/complesh/target/release/complesh -o "${path}" -i "${COMP_WORDS[COMP_CWORD]}"
    COMPREPLY[0]=$(cat "${path}")
    rm "${path}"
}

complete -F completion_function -o nospace test

test () {
    echo "test '$@'"
}
