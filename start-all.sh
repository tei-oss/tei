./dev-up.sh

cd server

trap stopall INT

cargo watch -x "run --bin tei-api" &
pid1=$!
cargo watch -x "run --bin tei-indexd" &
pid2=$!

stopall() {

  echo "Stopping background tasks"

  kill -2 $pid1
  kill -2 $pid2

  pkill tei-api
  pkill tei-index

  echo "Stopped all background processes"
}

wait
