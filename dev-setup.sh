LGREEN='\033[1;32m'
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

echo "${LGREEN}• Installing refinery${NC}"
cargo install refinery_cli

# Maybe we should also install nats CLI?
# curl -sf https://binaries.nats.dev/nats-io/natscli/nats@latest | sh

echo "\n✔ ${GREEN} You now may start local environment using 'dev-up.sh'${NC}"
