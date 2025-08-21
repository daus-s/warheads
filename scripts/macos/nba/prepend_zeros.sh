dir=$(pwd)

find . -type f | while IFS= read -r file; do
    base_name=$(basename "$file")

    if [[ "$base_name" != 00* ]]; then
        # echo "NO ZEROES: $base_name DIRECTORY: $(pwd)"
        # echo $file

        src="$file"
        dst="$(dirname "$file")/00$(basename "$file")"

        mv "$src" "$dst"

    fi

done
