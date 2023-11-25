LGREEN='\033[1;32m'
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

echo "${LGREEN}• Booting up docker containers${NC}"
docker compose up -d

echo "\n${LGREEN}• Applying migrations${NC}"
refinery migrate -c ./server/tei-data/refinery.toml -p ./server/tei-data/migrations

echo "\n✔ ${GREEN}Environment successfully created${NC}"