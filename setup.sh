#!/bin/bash

echo "Reminder: make sure you have set up your configuration.rs file in src/ before running this script (press any key to continue)"
read

echo "Please enter the directory for the static files you have set (recommended: /var/www/static) (no leading slash)"
read STATIC
echo "Please enter the directory for the logs you have set (recommended: /var/www/logs) (no leading slash)"
read LOGS

echo "mkdir -p $STATIC/html/"
mkdir -p $STATIC/html/
echo "mkdir -p $LOGS/html/"
mkdir -p $LOGS 
echo "cp ./media/* $STATIC/html/"
cp ./media/* $STATIC/html/ 
echo "Finished."