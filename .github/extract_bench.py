#!/usr/bin/python3

import os
import sys
import json
import requests

from glob import glob

commit_hash = os.environ.get('GITHUB_SHA')
git_branch = os.environ.get('GITHUB_REF').split('/')[2]
token = os.environ.get('API_KEY')
os = os.environ.get('OS_SPEC')

directory = "target/criterion"
globed = glob("target/criterion/*/new")
data = {
	"branch": git_branch,
	"commit_hash": commit_hash,
	"os": os,
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

url = "https://perf-ci.speculare.cloud/api/ingest"
req = requests.post(url, data=data_json, headers={'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json'})
