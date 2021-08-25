#!/bin/bash

ls | grep "\.c" | while read file
do
    target=$(echo $file | cut -d "." -f1)
    #echo $file-$target
    gcc $file -o $target
done