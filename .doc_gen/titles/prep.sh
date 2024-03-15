cat ../metadata/*_metadata.yaml | yq '.*.services as $svc ireduce ({}; . * $svc )' | while read line; do
  SVC=$(echo "$line" | cut -f 1 -d :)
  ENTS=$(cat ./ids/ids.json | jq ".[] | select(.service == \"$SVC\") | \"Entities: \" + .longname + \" \" + .shortname" | tr -d '"')
  echo "---\nprompt: >-\n  \"$line\n  $ENTS\"\n---" >./$SVC.md
done
