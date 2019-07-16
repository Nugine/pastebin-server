rm server.log
kill `ps -e | grep pastebin-server | awk '{print $1}'`
