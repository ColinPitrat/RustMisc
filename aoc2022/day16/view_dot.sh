#!/bin/bash

DOT_FILE=$1
PNG_FILE=$(echo $1 | sed 's/.dot/.png/')

dot -Tpng $DOT_FILE > $PNG_FILE
qiv $PNG_FILE
