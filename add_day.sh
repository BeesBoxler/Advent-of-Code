#! /bin/bash

curr_path=$(cd $(dirname "${BASH_SOURCE[0]}") && pwd);
number='^[0-9]+$';

add_day(){
	printf -v day "%02d" $1;
	today="$curr_path/src/days/day$day.rs";
	mod="$curr_path/src/days/mod.rs";

	if cp -n "$curr_path/src/days/template.rs" $today && ! grep "mod day$day;" $mod >> /dev/null; then
		tmp="$curr_path/TEMP.tmp";

		cat $mod | sed \
			-e "s/pub.*/mod day$day;\n&/" \
			-e "s/unwrap_or([0-9]*)/unwrap_or($1)/" \
			-e "s/_ => .*/$1 => day$day::run(input),\n        &/" \
			-e "/^$/d" \
			-e "s/pub.*/\n&/" \
			-e "s/#!.*/&\n/" > $tmp;
		
		cat $tmp > $mod;

		rm $tmp;
	else
		echo -e "\n\033[31mDay $day already exists. Doing nothing.\033[0m"
	fi
}

if [[ $1 =~ $number ]]; then 
	add_day $1;
elif [[ -z $1 ]]; then
	tmp=TEMP.tmp
	days=$(ls -1 src/days);
	echo $days | sed -e 's/ /\n/g' -e 's/day//g' -e 's/.rs//g' > $tmp;
	max=0;

	while read file; do 
		file=${file#0}
		if [[ $file =~ $number ]]  && [[ $file -gt $max ]]; then
			max=$file;
		fi
	done < $tmp

	add_day $(($max + 1))
else 
	echo -e "\n\033[31mThis function only accepts a single parameter of a day number.\033[0m" 
fi