#!/bin/bash

disk=${!#}

if [[ ! -e "$disk" ]]; then
	echo "$disk does not exist"
	exit 1
fi

echo -n "$disk "

smart=$(
	sudo smartctl -H "$disk" 2>/dev/null |
		grep "^SMART overall" |
		awk "{ print $6 }"
)
stats=$(
	sudo smartctl -a "$disk" | tail -n 26 | head -n 23 | grep -v "Temperature" | grep -v "^$"
)

[[ "$smart" == "" ]] && smart="unavailable"

echo "$smart"
echo -e "$stats\n"
