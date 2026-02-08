#!/bin/bash

MYSO_FRAMEWORK_DIR="../../../../crates/myso-framework/packages/myso-framework/**/*.move"
STDLIB_DIR="../../../../myso-framework/packages/move-stdlib/**/*.move"

tree-sitter generate --no-bindings
tree-sitter parse -q -t tests/*.move
tree-sitter parse -q -t tree-sitter $MYSO_FRAMEWORK_DIR
