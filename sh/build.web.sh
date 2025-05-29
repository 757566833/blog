#!/bin/bash

PACKAGE_JSON_FILE="apps/web/package.json"

VERSION=$(grep '"version":' "$PACKAGE_JSON_FILE" | sed -E 's/.*"version": "(.*)".*/\1/')

# # 输出版本号
echo "The version of the project is: $VERSION"

CURRENT_TIME=$(date -u +"%Y-%m-%dT%H-%M-%SZ")

echo "time is: $CURRENT_TIME"

mkdir docker.log
docker build -f apps/web/Dockerfile  --build-arg VERSION=${VERSION}_${CURRENT_TIME} -t ai-flx-web:${VERSION}_${CURRENT_TIME}  .  >  docker.log/ai-flx-web.build.log 2>&1
docker tag ai-flx-web:${VERSION}_${CURRENT_TIME} harbor.fzcode.com/ai/ai-flx-web:${VERSION}_${CURRENT_TIME}
echo "docker push harbor.fzcode.com/ai/ai-flx-web:${VERSION}_${CURRENT_TIME}"



# docker push harbor.nd.tevat.dev/z-web/ai-flx-web:${VERSION}_${CURRENT_TIME}



