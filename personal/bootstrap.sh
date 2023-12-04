#!/bin/env bash
if [ -f /.dockerenv ]; then
    for k in github google twitter reddit
	do
		[ -e "/inputs/$k/$(TZ='EST' date +"%Y")/.done" ] || [ "$(TZ='EST' date +"%m")" -ne "12" ] && [ -e "/inputs/$k/$(($(date +"%Y") - 1))/.done" ] && echo "All downloaded for $k user for now" && continue
		token=""
		[ -e "/inputs/${k}/.token" ] && token=$(cat /inputs/${k}/.token)
		if [ -z $token ] || [ "$(curl -s --cookie "session=$token" https://adventofcode.com/2015/day/1/input)" == "Puzzle inputs differ by user.  Please log in to get your puzzle input." ]
		then
			echo "${k} Login with https://adventofcode.com/auth/${k}"
			echo ""
			echo "Then open the dev tools and copy the cookie named 'session' here (leave empty to not login with ${k}) :"
			read -p "Session token : " token
			echo "$token" > "/inputs/${k}/.token"
		fi
		if [ -n "$token" ]
		then 
			for y in $(seq 2015 $(date +"%Y"))
			do 
				yearfolder="/inputs/$k/$y"
				[ -e "${yearfolder}/.done" ] && continue
				mkdir -p "$yearfolder"
				for d in $(seq 1 25)
				do
					d=$(printf "%02d" $d)
					dayfolder="${yearfolder}/${d}"
					[ -e "${dayfolder}/.done" ] && continue
					mkdir -p "$dayfolder"
					AOC_SESSION=$token aocd $d $y > "${dayfolder}/input.txt" && touch "${dayfolder}/.done" && echo "Fetched AoC of $y/$d"|| break
				done
				[ -e "${yearfolder}/25/.done" ] && touch "${yearfolder}/.done"
			done
		fi
		echo "Done fetching input with $k user"
		echo "" 
	done
else
    echo "Building and running docker container..."
	docker run --rm --init -it -v ./inputs:/inputs $(docker build -q .)
	echo "Done"
fi
