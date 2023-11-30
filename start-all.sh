./dev-up.sh

trap stopall INT

cd server

cargo watch -x "run --bin tei-api" &
pid1=$!
cargo watch -x "run --bin tei-indexd" &
pid2=$!

cd ../client/tei-web
npm i
npm run dev &
pid3=$!

stopall() {

  echo "Stopping background tasks"

  kill -2 $pid1
  kill -2 $pid2
  kill -3 $pid3

  pkill tei-api
  pkill tei-index

  echo "Stopped all background processes"
}

wait
