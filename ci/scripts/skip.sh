#!/bin/bash

grep -q "chore(release)" <<< $(git log -1 --pretty=format:"%s") && \
  echo "::set-output name=skip::true" || \
  echo "::set-output name=skip::false"
