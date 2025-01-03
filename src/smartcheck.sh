#!/bin/bash

if sudo true; then
	true
else
	echo 'Root privileges required'

	exit 1
fi

for drive in /dev/nvme[0-3]n1; do
	if [[ ! -e $drive ]]; then continue; fi

	echo -n "$drive "

	smart=$(
		sudo smartctl -H $drive 2>/dev/null |
			grep '^SMART overall' |
			awk '{ print $6 }'
	)
	stats=$(
		sudo smartctl -a $drive | tail -n 26 | head -n 23 | grep -v "Temperature" | grep -v "^$"
	)

	[[ "$smart" == "" ]] && smart='unavailable'

	echo "$smart"
	echo -e "$stats\n"
done
