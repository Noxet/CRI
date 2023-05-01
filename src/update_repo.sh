#!/bin/bash

source src/run_or_fail.sh

rm -f .commit_id

# Check if the repo folder actually exists
run_or_fail "Repository folder not found!" pushd $1 1> /dev/null
run_or_fail "Could not reset git" git reset --hard HEAD

# Get latest commit
COMMIT=$(run_or_fail "Could not call 'git log 1'" git log -n1)
pwd
if [ $? != 0 ]; then
	echo "Could not call 'git log' on repository 1"
	exit 1
fi

# get first line, second row (commit id
# # get first line, second row (commit id)
COMMIT_ID=`echo $COMMIT | awk 'NR==1 { print $2 }'`

# Pull and check if there is any new commit
run_or_fail "Could not git pull" git pull

COMMIT=$(run_or_fail "Could not call 'git log'" git log -n1)
if [ $? != 0 ]; then
	echo "Could not call 'git log' on repository"
	exit 1
fi

NEW_COMMIT_ID=`echo $COMMIT | awk 'NR==1 { print $2 }'`

if [ $NEW_COMMIT_ID != $COMMIT_ID ]; then
	popd 1> /dev/null
	echo $NEW_COMMIT_ID > .commit_id
fi
