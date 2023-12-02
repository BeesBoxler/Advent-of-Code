#! /bin/bash

curr_path=$(cd $(dirname "${BASH_SOURCE[0]}") && pwd);
number='^[0-9]+$';

add_day(){
	printf -v day_padded "%02d" $day;
	today="$curr_path/src/days/day$day_padded.rs";
	today_folder="$curr_path/src/days/day$day_padded"
	mod="$curr_path/src/days/mod.rs";

	if [[ -d $today_folder ]] || [[ -f $today ]]; then 
		echo -e "\nFile or folder \`day$day_padded\` already exists"
		return 1;
	fi

	if [[ $folder ]]; then 
		today="$today_folder/mod.rs"
		mkdir $today_folder
	fi

	if cp -n "$curr_path/src/days/template.rs" $today && ! grep "mod day$day;" $mod >> /dev/null; then
		tmp="$curr_path/TEMP.tmp";

		cat $mod | sed \
			-e "s/pub.*/mod day$day_padded;\n&/" \
			-e "s/unwrap_or([0-9]*)/unwrap_or($day)/" \
			-e "s/_ => .*/$day => day$day_padded::run(input),\n        &/" \
			-e "/^$/d" \
			-e "s/pub.*/\n&/" \
			-e "s/#!.*/&\n/" > $tmp;
		
		cat $tmp > $mod;

		rm $tmp;
	else
		echo -e "\n\033[31mDay $day already exists. Doing nothing.\033[0m"
	fi
}

get_next_day() {
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

	echo $(($max + 1))
}

while [[ $# -gt 0 ]]; do 
	if [[ $1 =~ $number ]]; then 
		day=$1;
	elif [[ $1 == *"--"* ]]; then 
		v="${1/--/}";
    	declare $v=true;
	fi
	shift;
done

if [[ -z $day ]]; then
	day=$(get_next_day)
fi

add_day