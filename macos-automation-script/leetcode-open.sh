#!/bin/zsh 

BROWSER=${1:-"Google Chrome"}
open -a "$BROWSER" "https://leetcode.com/problemset/"
open -a "$BROWSER" "https://neetcode.io/"
open -a "$BROWSER" "https://docs.google.com/spreadsheets/d/1bp-YmKgI06kWqMnw0tzadOiAHPcVfObTCdcUoxrvMA0/edit?gid=0#gid=0"

echo "Study Environment Launched in $Browser"
