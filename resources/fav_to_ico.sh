#!/bin/sh
# ImageMagick
#convert +antialias -background transparent fav.svg -define icon:auto-resize=256,128,64,48,32,16 ../assets/favicon.ico
#convert +antialias -background transparent -density 300 fav.svg -resize 256x256 ../assets/app.png

resvg -w 256 -h 256 fav.svg ../assets/app.png
convert +antialias -background transparent ../assets/app.png -define icon:auto-resize=256,128,64,48,32,16 ../assets/favicon.ico
