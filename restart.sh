rm server.log
kill `ps -e | grep pastebin-server | awk '{print $1}'`
source start.sh