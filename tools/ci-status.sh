#!/usr/bin/env bash
# 查最新一次 GitHub Actions 打包狀態；失敗時印出出錯的步驟與錯誤訊息
R=https://api.github.com/repos/tomogeo/xiao-claude-pet
run=$(curl -sS "$R/actions/runs?per_page=1")
id=$(echo "$run" | python3 -c "import json,sys;r=json.load(sys.stdin)['workflow_runs'];print(r[0]['id'] if r else '')")
[ -z "$id" ] && { echo "no runs yet"; exit 0; }
echo "$run" | python3 -c "import json,sys;r=json.load(sys.stdin)['workflow_runs'][0];print(r['id'],r['head_sha'][:7],r['status'],r['conclusion'])"
curl -sS "$R/actions/runs/$id/jobs" | python3 -c "
import json,sys
for j in json.load(sys.stdin)['jobs']:
  print('job',j['id'],j['status'],j['conclusion'])
  for s in j['steps']: print('  ',s['number'],s['name'],s['status'],s['conclusion'])
"
