#!/bin/bash

echo "Reminder: make sure you have set up your configuration.rs file in src/ before running this script (press any key to continue)"
read

echo "Please enter the directory for the static files you have set (recommended: /var/www/static) (no leading slash)"
read STATIC
echo "Please enter the directory for the logs you have set (recommended: /var/www/logs) (no leading slash)"
read LOGS

echo "rm -rf $STATIC/"
rm -rf $STATIC/
echo "mkdir $STATIC/"
mkdir $STATIC/
echo "mkdir -p $LOGS/"
mkdir -p $LOGS/ 
echo "cp ./media/* $STATIC/"
cp ./media/* $STATIC/
echo "Finished."
