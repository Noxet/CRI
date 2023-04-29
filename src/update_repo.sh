#!/bin/bash

# TODO: remove this
echo -ne "testing: $1"
exit 0

#source run_or_fail.sh

bash rm -f .commit_id

# Check if the repo folder actually exists
#run_or_fail "Repository folder not found!" pushd $1 1> /dev/null
#run_or_fail "Could not reset git" git reset --hard HEAD

pushd $1 1> /dev/null
git reset --hard HEAD

# Get latest commit
COMMIT=$(git log -n1)
if [ $? != 0 ]; then
	echo "Could not call 'git log' on repository"
	exit 1
fi

COMMIT_ID=`echo $COMMIT | awk '{ print $2 }'`

# Pull and check if there is any new commit
git pull

COMMIT=$(git log -n1)
if [ $? != 0 ]; then
	echo "Could not call 'git log' on repository"
	exit 1
fi

NEW_COMMIT_ID=`echo $COMMIT | awk '{ print $2 }'`

if [ $NEW_COMMIT_ID != $COMMIT_ID ]; then
	popd 1> /dev/null
	echo $NEW_COMMIT_ID > .commit_id
fi
