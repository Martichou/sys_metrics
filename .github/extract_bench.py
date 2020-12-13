#!/usr/bin/python3

import os
import sys
import json
import requests

from glob import glob

if len(sys.argv) < 4:
	print("Missing args (commit_hash, git_branch, token)")
	exit(0)

commit_hash = sys.argv[1]
git_branch = sys.argv[2]
token = sys.argv[3]

directory = "target/criterion"
globed = glob("target/criterion/*/new")
data = {
	"branch": git_branch,
	"commit_hash": commit_hash,
	"datas": [],
}

for res_dir in globed:
	name_bench = res_dir.split('/')[2]
	loca_bench = res_dir + "/estimates.json"
	with open(loca_bench) as json_file:
		data_bench = json.load(json_file)
		# Extract mean/median/slope
		temp = {
			"bench": name_bench,
		}
		for k in data_bench:
			if k not in ["mean", "median", "slope"]: continue
			temp[k] = data_bench[k]['point_estimate']
		data["datas"].append(temp)
data_json = json.dumps(data)

url = "https://perf-ci.speculare.cloud"
req = requests.post(url, data=data_json, headers={'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json'})
