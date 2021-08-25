#!/bin/bash

rm -f dmesg.info
sleep 3
for i in {1..1000}
do
	dmesg -c >> dmesg.info
	sleep 0.001
done
