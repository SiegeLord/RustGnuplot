#!/usr/bin/env python3

import argparse
import fileinput
import re
import os
import glob
import time
import toml
import json
from shutil import copy, rmtree
from subprocess import check_call, check_output, CalledProcessError

def split(s):
	ret = s.split('\n')
	return filter(lambda v: v, ret)

crate_list=split("""
gnuplot
""")

parser = argparse.ArgumentParser(description='Perform an operation on all crates.')
parser.add_argument('--version', metavar='VERSION', default='', help='set the version to VERSION')
parser.add_argument('--publish', action='store_true', help='publish the crates')
parser.add_argument('--build', action='store_true', help='build the crates')
parser.add_argument('--test', action='store_true', help='test the crates')
parser.add_argument('--clean', action='store_true', help='clean the crates')
parser.add_argument('--doc', action='store_true', help='build the documentation')
parser.add_argument('--format', action='store_true', help='format all the non-sys crates')
parser.add_argument('--verbose', action='store_true', help='pass --verbose to cargo')
parser.add_argument('--num_retries', type=float, default=5, help='number of retries when publishing')

args = parser.parse_args()

def cargo_cmd(command):
	return ['cargo', command] + (['--verbose'] if args.verbose else [])

if len(args.version) > 0:
	crates_and_doc = ['doc']
	crates_and_doc.extend(crate_list)

	for crate in crates_and_doc:
		cargo_toml = crate + '/Cargo.toml'
		print('Processing', cargo_toml)

		for line in fileinput.input(cargo_toml, inplace=1):
			line = re.sub('version = "(=?).*" #auto', 'version = "\g<1>' + args.version + '" #auto', line)
			print(line, end='')

if args.publish:
	for crate in crate_list:
		print('Publishing crate inside', crate)
		metadata = json.loads(
			check_output(
				'cargo metadata --format-version=1 --no-deps'.split(' '),
				cwd=crate,
			)
		)

		package_metadata = metadata['packages'][0]
		new_version = package_metadata['version']
		crate_name = package_metadata['name']

		search_output = check_output(
			f'cargo search {crate_name} --limit 9999'.split(' ')
		).decode('utf8')

		search_result = toml.loads(search_output)
		old_version = search_result[crate_name]
		if old_version == new_version:
			print(f'Version {new_version} already published, skipping.')
			continue

		for i in range(args.num_retries):
			try:
				check_call(cargo_cmd('publish'), cwd=crate)
				break
			except CalledProcessError:
				print(f'Try {i} failed')
			time.sleep(1. + i)

if args.build:
	check_call(cargo_cmd('build'), cwd='.')

if args.format:
	for crate in crate_list:
		check_call(cargo_cmd('fmt'), cwd=crate)

if args.test:
	crates_no_examples = filter(lambda crate: crate != 'examples', crate_list)
	for crate in crates_no_examples:
		check_call(cargo_cmd('test') + ['-p', crate], cwd='.')

if args.clean:
	crates_and_doc = ['doc']
	crates_and_doc.extend(crate_list)
	for crate in crates_and_doc:
		print('Cleaning', crate)
		lock = crate + '/Cargo.lock'
		if os.path.exists(lock):
			os.remove(lock)
		check_call(cargo_cmd('clean'), cwd=crate)

if args.doc:
	rmtree('doc/target/doc', ignore_errors=True)
	print('Building docs')
	check_call(['cargo', 'doc'], cwd='doc')
	print('Fixing up the search index')
	found = False
	for line in fileinput.input('doc/target/doc/search-index.js', inplace=1):
		new_line = re.sub(r'"delete_me".*', r'\\', line)
		if new_line != line:
			found = True
		else:
			print(new_line, end='')
	if not found:
		raise Exception("Couldn't find the line in search-index.js!")
	found = False
	for line in fileinput.input('doc/target/doc/source-files.js', inplace=1):
		new_line = re.sub(r'sourcesIndex\["delete_me"\].*', r'', line)
		if new_line != line:
			found = True
		else:
			print(new_line, end='')
	if not found:
		raise Exception("Couldn't find the line in source-files.js!")
	print('Copying new CSS')
	copy('doc/rustdoc.css', 'doc/target/doc/rustdoc.css')
	copy('doc/light.css', 'doc/target/doc/light.css')
	copy('doc/dark.css', 'doc/target/doc/dark.css')
