wsl --shutdown
wsl.exe --list --verbose
diskpart
select vdisk file="C:\Users\home\AppData\Local\Docker\wsl\disk\docker_data.vhdx"
compact vdisk



# Настраиваем часовой пояс автоматически
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Europe/Kiev
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone