#!/system/bin/sh
echo "Hello World!"
echo "仓库地址: https://github.com/reigadegr/scx_controller"

[ ! -d /storage/emulated/0/Android/scx_controller ] && mkdir -p /storage/emulated/0/Android/scx_controller

if [ ! -f /storage/emulated/0/Android/scx_controller/app_config.toml ]; then
    cp $MODPATH/app_config.toml /storage/emulated/0/Android/scx_controller/app_config.toml
fi

time=$(date "+%Y-%m-%d_%H:%M:%S")
cp -af /storage/emulated/0/Android/scx_controller/app_config.toml /storage/emulated/0/Android/scx_controller/"$time"backup.toml
cp -f $MODPATH/app_config.toml /storage/emulated/0/Android/scx_controller/app_config.toml
