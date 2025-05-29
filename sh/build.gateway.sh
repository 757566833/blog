#!/bin/bash



PACKAGE_JSON_FILE="apps/gateway/Cargo.toml"

VERSION=$(grep 'version = ' "$PACKAGE_JSON_FILE" | head -n 1 | sed -E 's/.*version = *"(.*)".*/\1/')

# # 输出版本号
echo "The version of the project is: $VERSION"

CURRENT_TIME=$(date -u +"%Y-%m-%dT%H-%M-%SZ")

echo "time is: $CURRENT_TIME"

mkdir docker.log
docker build -f apps/gateway/Dockerfile  --build-arg VERSION=${VERSION}_${CURRENT_TIME} -t ai-flx-gateway:${VERSION}_${CURRENT_TIME}  .  >  docker.log/ai-flx-gateway.build.log 2>&1
docker tag ai-flx-gateway:${VERSION}_${CURRENT_TIME} harbor.fzcode.com/ai/ai-flx-gateway:${VERSION}_${CURRENT_TIME}
echo "docker push harbor.fzcode.com/ai/ai-flx-gateway:${VERSION}_${CURRENT_TIME}"



# docker push harbor.nd.tevat.dev/z-web/ai-flx-gateway:${VERSION}_${CURRENT_TIME}



