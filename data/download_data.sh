#!/bin/bash

set -xe

rm -fdr *.zip

wget "https://theweekinchess.com/zips/twic1544g.zip"
yes | unzip twic1544g.zip