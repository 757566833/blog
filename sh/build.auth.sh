#!/bin/bash



PACKAGE_JSON_FILE="apps/auth/Cargo.toml"

VERSION=$(grep 'version = ' "$PACKAGE_JSON_FILE" | head -n 1 | sed -E 's/.*version = *"(.*)".*/\1/')

# # 输出版本号
echo "The version of the project is: $VERSION"

CURRENT_TIME=$(date -u +"%Y-%m-%dT%H-%M-%SZ")

echo "time is: $CURRENT_TIME"

mkdir docker.log
docker build -f apps/auth/Dockerfile  --build-arg VERSION=${VERSION}_${CURRENT_TIME} -t ai-flx-auth:${VERSION}_${CURRENT_TIME}  .  >  docker.log/ai-flx-auth.build.log 2>&1
docker tag ai-flx-auth:${VERSION}_${CURRENT_TIME} harbor.fzcode.com/ai/ai-flx-auth:${VERSION}_${CURRENT_TIME}
echo "docker push harbor.fzcode.com/ai/ai-flx-auth:${VERSION}_${CURRENT_TIME}"



# docker push harbor.nd.tevat.dev/z-web/ai-flx-auth:${VERSION}_${CURRENT_TIME}



