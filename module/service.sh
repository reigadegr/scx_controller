#!/system/bin/sh
MODDIR=${0%/*}
LOG=$MODDIR/log.txt

wait_until_login() {
    # in case of /data encryption is disabled
    while [ "$(getprop sys.boot_completed)" != "1" ]; do sleep 1; done
    # we doesn't have the permission to rw "/sdcard" before the user unlocks the screen
    until [ -d /sdcard/Android ]; do sleep 1; done
}

lock_val() {
    find "$2" -type f | while read -r file; do
        file="$(realpath "$file")"
        umount "$file"
        chown root:root "$file"
        chmod 0644 "$file"
        echo "$1" >"$file"
        chmod 0444 "$file"
    done
}

if [ "$(getprop sys.boot_completed)" != "1" ]; then
    wait_until_login
    if [ ! -L $MODDIR/app_config.toml ]; then
        rm $MODDIR/app_config.toml
        ln -s /storage/emulated/0/Android/app_config.toml $MODDIR/app_config.toml
    fi
fi

killall -15 scx_controller; rm $LOG
chmod +x ${0%/*}/scx_controller
RUST_BACKTRACE=1 nohup $MODDIR/scx_controller >$LOG 2>&1 &
