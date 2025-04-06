# LLM via REST API v1 

This crate combines `warp` and `candle example llm` to serve llm via REST API. 


Example;

```sh
cargo run --features cuda -- --prompt "who am i talking to?" --which "7b"

cargo run --features cuda -- --prompt "who am i talking to?" --which "2.5-corder:14B-q4"
```

Once it's running, one can interact with it via REST API. For example,

```sh
curl -vvv http://localhost:8000/generate

curl  -X POST -H "Content-Type: application/json" -d "{\"prompt\":\"Who are you?\",\"temperature\":0}"  http://localhost:8000/generate

```

