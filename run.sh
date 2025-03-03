#!/bin/sh

larger=/usr/share/dict/words

export ENV_SHORTER_TEXT_FILENAME=./sample.d/input.txt

geninput(){
	echo generating input data...

	mkdir -p ./sample.d

	cat $0 |
		sed 's/[^a-zA-Z]/ /g' |
		tr '[:upper:]' '[:lower:]' |
		grep \
		--only-matching \
		'\S\{1,\}' |
		tr ' ' '\n' |
		grep ... |
		sort -u |
		cat > "${ENV_SHORTER_TEXT_FILENAME}"
}

test -f "${ENV_SHORTER_TEXT_FILENAME}" || geninput

export ENV_SHORTER_TEXT_FILE_SIZE_LIMIT=1048576

cat "${larger}" |
	./rs-find-missing |
	wc -l

cat "${larger}" |
	wc -l

cat "${ENV_SHORTER_TEXT_FILENAME}" |
	wc -l

export ENV_SHORTER_TEXT_FILE_SIZE_LIMIT=16777216

cat "${ENV_SHORTER_TEXT_FILENAME}" |
	ENV_SHORTER_TEXT_FILENAME="${larger}" ./rs-find-missing |
	cat -n
