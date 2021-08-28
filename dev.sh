#!/bin/bash
docker run -v "${PWD}/config.yml:/usr/src/config.yml" spotify-crawler
