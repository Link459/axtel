#! /usr/bin/env bash

num_times=$1

# Loop to call curl the specified number of times
for ((i=1; i<=$num_times; i++))
do
   curl -X GET "127.0.0.1:3000" &
done
