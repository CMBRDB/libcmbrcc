#!/bin/bash

set -xe

wget "https://theweekinchess.com/zips/twic1544g.zip"
yes | unzip twic1544g.zip

wget "https://www.chessgames.com/nodejs/game/downloadGamePGN/fischer_spassky_1992.pgn?gid=1129672"
mv "fischer_spassky_1992.pgn?gid=1129672" fischer_spassky_1992.pgn

mv *.pgn data
rm -fdr *.zip