Start the faucet service

### mgo-faucet

##### 1. Prepare to compile binary files
```
1. mgo-faucet
cargo build -p mgo-faucet -r
```
##### 2. The default address needs to have sufficient gas

```
#The faucet receives the gas from the default address
mgo client gas
```
##### 3. Add active node rpc

```
Usage: mgo client new-env [OPTIONS] --alias <ALIAS> --rpc <RPC>

Options:
      --alias <ALIAS>  
      --rpc <RPC>      
      --ws <WS>        
      --json           Return command outputs in json format
  -h, --help           Print help
  -V, --version        Print version

mgo client envs
```
##### 4. Start the faucet service
```
Usage: mgo-faucet [OPTIONS] --write-ahead-log <WRITE_AHEAD_LOG>

Options:
      --port <PORT>
          [default: 5003]
      --host-ip <HOST_IP>
          [default: 127.0.0.1]
      --amount <AMOUNT>
          [default: 1000000000]
      --num-coins <NUM_COINS>
          [default: 1]
      --request-buffer-size <REQUEST_BUFFER_SIZE>
          [default: 10]
      --max-request-per-second <MAX_REQUEST_PER_SECOND>
          [default: 10]
      --wallet-client-timeout-secs <WALLET_CLIENT_TIMEOUT_SECS>
          [default: 60]
      --write-ahead-log <WRITE_AHEAD_LOG>
          
      --wal-retry-interval <WAL_RETRY_INTERVAL>
          [default: 300]
      --max-request-queue-length <MAX_REQUEST_QUEUE_LENGTH>
          [default: 10000]
      --batch-request-size <BATCH_REQUEST_SIZE>
          [default: 500]
      --ttl-expiration <TTL_EXPIRATION>
          [default: 300]
      --batch-enabled <BATCH_ENABLED>
          [default: false] [possible values: true, false]
  -h, --help
          Print help

MAX_CONCURRENCY=100000000 mgo-faucet --port 8090 --host-ip 0.0.0.0 --amount 1000000000 --write-ahead-log /root/write-ahead-log

```
![blob](https://image.devnet.mangonetwork.io/img/faucets.png "Start Service")

##### 5. Collect gas

![blob](https://image.devnet.mangonetwork.io/img/faucet.png "Claim")

![blob](https://image.devnet.mangonetwork.io/img/gas.png "Query and claim successful balance")
