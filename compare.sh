#/usr/bin/env bash

DAY_LIMIT=12

function compare() {
    day=$(printf "%02d" $1)
    diff <(cd rust; cargo run $day 2>/dev/null) <(cd python; ./src/day$day.py)
}

if [[ $1 = "all" ]]; then
    for day in $(seq 1 $DAY_LIMIT); do
        printf "Checking day %02d... " $day

        compare $day

        if [[ $? = 0 ]]; then
            echo "match!"
        else
            echo "no match :("
        fi
    done
else
    compare $1

    if [[ $? = 0 ]]; then
        echo "Rust and Python match!"
    else
        echo "X - Rust and Python don't match :("
    fi
fi
