wsl --shutdown
wsl.exe --list --verbose
diskpart
select vdisk file="C:\Users\home\AppData\Local\Docker\wsl\disk\docker_data.vhdx"
compact vdisk
