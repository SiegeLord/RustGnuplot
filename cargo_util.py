#!/usr/bin/env python

import argparse
import fileinput
import re
import os
import glob
from shutil import copy, rmtree
from subprocess import check_call

crate_list="""
.
"""

parser = argparse.ArgumentParser(description='Perform an operation on all crates.')
parser.add_argument('--version', metavar='VERSION', default='', help='set the version to VERSION')
parser.add_argument('--publish', action='store_true', help='publish the crates')
parser.add_argument('--build', action='store_true', help='build the crates')
parser.add_argument('--test', action='store_true', help='test the crates')
parser.add_argument('--clean', action='store_true', help='clean the crates')
parser.add_argument('--doc', action='store_true', help='build the documentation')

args = parser.parse_args()

crate_list = crate_list.split('\n')
crate_list = filter(lambda crate: len(crate) > 0, crate_list)

if len(args.version) > 0:
	crates_and_doc = ['doc']
	crates_and_doc.extend(crate_list)

	for crate in crates_and_doc:
		cargo_toml = crate + '/Cargo.toml'
		print 'Processing', cargo_toml

		for line in fileinput.input(cargo_toml, inplace=1):
			line = re.sub('version = "(=?).*" #auto', 'version = "\g<1>' + args.version + '" #auto', line)
			print line,

if args.publish:
	for crate in crate_list:
		print 'Publishing', crate
		check_call(['cargo', 'publish'], cwd=crate)

if args.build:
	check_call(['cargo', 'build'], cwd='slr_config')

if args.test:
	crates_no_examples = filter(lambda crate: crate != 'examples', crate_list)
	for crate in crates_no_examples:
		check_call(['cargo', 'test'], cwd=crate)

if args.clean:
	crates_and_doc = ['doc']
	crates_and_doc.extend(crate_list)
	for crate in crates_and_doc:
		print 'Cleaning', crate
		lock = crate + '/Cargo.lock'
		if os.path.exists(lock):
			os.remove(lock)
		check_call(['cargo', 'clean'], cwd=crate)

if args.doc:
	rmtree('doc/target/doc', ignore_errors=True)
	print 'Building docs'
	check_call(['cargo', 'doc'], cwd='doc')
	print 'Fixing up the search index'
	found = False
	for line in fileinput.input('doc/target/doc/search-index.js', inplace=1):
		new_line = re.sub(r'searchIndex\["delete_me"\].*', '', line)
		if new_line != line:
			found = True
		print new_line,
	if not found:
		raise Exception("Couldn't find the line in search-index.js!")
	print 'Copying new CSS'
	for path in glob.glob('doc/target/doc/*.css'):
		os.remove(path)
	copy('doc/main.css', 'doc/target/doc/main.css')
