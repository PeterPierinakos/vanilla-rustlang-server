#!/bin/bash

STATIC=/var/www/static
LOGS=/var/www/logs

echo "rm -rf $STATIC/"
rm -rf $STATIC/
echo "mkdir $STATIC/"
mkdir $STATIC/
echo "mkdir -p $LOGS/"
mkdir -p $LOGS/ 
echo "cp ./media/* $STATIC/"
cp ./media/* $STATIC/
echo "Finished."
