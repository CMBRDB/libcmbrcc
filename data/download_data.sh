#!/bin/bash

set -xe

wget "https://theweekinchess.com/zips/twic1544g.zip"
yes | unzip twic1544g.zip

wget "https://www.chessgames.com/nodejs/game/downloadGamePGN/fischer_spassky_1992.pgn?gid=1129672"
mv "fischer_spassky_1992.pgn?gid=1129672" fischer_spassky_1992.pgn

wget "https://usc1.contabostorage.com/716916dc83654e5eb4d2059cde9bd53d:ajedrez/AJ-CORR-PGN-000.7z"
7z e AJ-CORR-PGN-000.7z

mv *.pgn data
rm -fdr *.zip
rm -fdr *.7z
