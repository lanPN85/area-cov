set -e

INP_DIR=./data/ega
INP_FILES=$INP_DIR/*.in
OUT_DIR=./out
LOG_FILE=$OUT_DIR/logs
EXE_FILE=./target/release/area_cov
PY_TARGET=./src/plot/plot.py

# Clear log file
echo "" > $LOG_FILE
cargo build --release

for FIN in $INP_FILES
do
	IFS='/' read -r -a array1 <<< "$FIN"
	IFS='.' read -r -a array2 <<< "${array1[-1]}"
	NAME="${array2[0]}"
	FOUT="$OUT_DIR/$NAME.out"
	FPNG="$OUT_DIR/$NAME.png"

	cmd1="$EXE_FILE $FIN -o $FOUT -i 1000 --size 50  2>>$LOG_FILE"
	echo "$cmd1"
	eval $cmd1

	cmd2="python $PY_TARGET $FOUT $FPNG"
	echo "$cmd2"
	eval $cmd2
done
