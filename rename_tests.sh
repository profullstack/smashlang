#!/bin/bash

# Create backup directory
mkdir -p /home/ettinger/src/profullstack.com/smashlang/tests/backup

# Backup original test files
cp /home/ettinger/src/profullstack.com/smashlang/tests/codegen_tests.rs /home/ettinger/src/profullstack.com/smashlang/tests/backup/
cp /home/ettinger/src/profullstack.com/smashlang/tests/compiler_tests.rs /home/ettinger/src/profullstack.com/smashlang/tests/backup/
cp /home/ettinger/src/profullstack.com/smashlang/tests/lexer_parser_tests.rs /home/ettinger/src/profullstack.com/smashlang/tests/backup/

# Rename test files to follow the .test.rs naming convention
cp /home/ettinger/src/profullstack.com/smashlang/tests/codegen_tests.rs /home/ettinger/src/profullstack.com/smashlang/tests/codegen.test.rs
cp /home/ettinger/src/profullstack.com/smashlang/tests/compiler_tests.rs /home/ettinger/src/profullstack.com/smashlang/tests/compiler.test.rs
cp /home/ettinger/src/profullstack.com/smashlang/tests/lexer_parser_tests.rs /home/ettinger/src/profullstack.com/smashlang/tests/lexer_parser.test.rs

# Don't remove original files yet until we confirm the new ones work
echo "Test files have been renamed. Original files are backed up in tests/backup/"
echo "New test files:"
ls -la /home/ettinger/src/profullstack.com/smashlang/tests/*.test.rs
