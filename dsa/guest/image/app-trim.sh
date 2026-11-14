# Desktop Subsystem for Android (DSA)
# Trims and compacts an app no window of which the user can see, as the agent asks
# (vendor.dsa.app.trim, dsa-app-memory.rc): "<number> <HIDDEN|BACKGROUND> <compact> <process>".
# The activity manager sends the process the trim level (ComponentCallbacks2.onTrimMemory), on
# which ART and the UI toolkit let caches go; it sends a level only once, and never a lower one
# than the process last had, so asking again costs nothing. With compact 1 the activity manager
# then compacts the process fully, which pages its anonymous memory and its files out at once.
# The request's number goes to vendor.dsa.app.trim.done once it is carried out, whatever came of
# it, so the agent sends the next one.

tag=DsaAppTrim

trim() {
    set -- $1
    number=$1
    level=$2
    compact=$3
    process=$4
    case "$level" in
        HIDDEN | BACKGROUND) ;;
        *) log -p w -t "$tag" "ignored a request with the level '$level'"; return ;;
    esac
    case "$process" in
        "" | *[!A-Za-z0-9._:]*) log -p w -t "$tag" "ignored a request for the process '$process'"; return ;;
    esac
    if am send-trim-memory "$process" "$level" > /dev/null 2>&1; then
        log -t "$tag" "sent $process the memory trim $level"
    else
        log -p w -t "$tag" "could not send $process the memory trim $level (it may have ended)"
    fi
    if [ "$compact" = 1 ]; then
        if am compact full "$process" > /dev/null 2>&1; then
            log -t "$tag" "compacted $process"
        else
            log -p w -t "$tag" "could not compact $process"
        fi
    fi
}

while :; do
    asked=$(getprop vendor.dsa.app.trim)
    trim "$asked"
    setprop vendor.dsa.app.trim.done "${asked%% *}"
    [ "$(getprop vendor.dsa.app.trim)" = "$asked" ] && break
done
