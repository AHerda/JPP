docker stop jpp
docker rm jpp
docker rmi $(docker images --filter “dangling=true” -q --no-trunc)
docker build . -t jpp --network host
docker run --name jpp -d -i -t jpp /bin/sh
docker exec -it jpp bash
