# Desktop Subsystem for Android (DSA)
# Compiles an app the user opens with the speed filter, as the agent asks while the guest is
# idle (vendor.dsa.app.compile, dsa-app-memory.rc): "<number> <package>". The answer goes to
# vendor.dsa.app.compile.done as "<number> <result>":
#   compiled   the package is compiled with speed now
#   current    it already was (speed or everything), so nothing ran
#   failed     the package manager refused or the compile failed
#   missing    no such package is installed
#   invalid    the request names no package
#
# The package manager's own compile (ART Service, the reason "cmdline") is what runs, without
# a forced recompile: it compiles only when speed is better than what the package has, which
# after an update is the install's speed-profile again. ART makes no app image for a filter
# that is not profile guided, so the code is shared from the .oat file and nothing of it is
# copied into each process. ART's background job leaves it alone, since its speed-profile is
# not better than speed.

tag=DsaAppCompile

# The compiler filter and the compile reason of the base APK's code for the primary ABI, as
# "speed-profile install-dm", from ART's dump of the package (as gapps-compile.sh reads it).
# Empty when the package is not installed.
status() {
    cmd package art dump "$1" 2>/dev/null |
        sed -n '/path: /,/\[primary-abi\]/{
/\[primary-abi\]/{
s/.*\[status=\([^]]*\)\] \[reason=\([^]]*\)\].*/\1 \2/p
q
}
}'
}

compile() {
    package=$1
    case "$package" in
        "" | *[!A-Za-z0-9._]*) echo invalid; return ;;
    esac
    current=$(status "$package")
    case "$current" in
        "") echo missing; return ;;
        speed\ * | everything\ *) echo current; return ;;
    esac
    log -t "$tag" "compiling $package with speed (it was $current)"
    if cmd package compile -m speed -p PRIORITY_BACKGROUND "$package" > /dev/null 2>&1; then
        log -t "$tag" "compiled $package: $(status "$package")"
        echo compiled
    else
        log -p w -t "$tag" "could not compile $package with speed; it keeps $current"
        echo failed
    fi
}

while :; do
    asked=$(getprop vendor.dsa.app.compile)
    number=${asked%% *}
    package=${asked#* }
    [ "$package" = "$asked" ] && package=
    result=$(compile "$package")
    setprop vendor.dsa.app.compile.done "$number $result"
    [ "$(getprop vendor.dsa.app.compile)" = "$asked" ] && break
done
