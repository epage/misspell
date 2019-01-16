cat assets/words.go | rg '"' | cut -d'"' -f2-4 | tr -d '"' > assets/words.csv
