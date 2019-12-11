filename=$PWD

parentdir_v1="${filename%/*}"
parentdir_v2="$(dirname "$filename")"

basename_v1="${filename##*/}"
basename_v2="$(basename "$filename")"

echo "$parentdir_v1"
echo "$parentdir_v2"
echo "$basename_v1"
echo "$basename_v2"
