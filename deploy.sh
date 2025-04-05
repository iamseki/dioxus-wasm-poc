#!/bin/bash
set -e

dx bundle --platform web
mv dist/public/* docs/