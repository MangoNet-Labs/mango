---
title: Build node
description: Build node
sidebar_position: 3
---


# Build node
First you need to download[Genesis](https://github.com/MangoNet-Labs/mango-genesis)，then download[release](https://github.com/MangoNet-Labs/Mango/releases)
# Start full node

1. Prepare what you need

```
root@iZt4n0235h02f2iwp3mlslZ:~/mango# ll
total 99708
drwxr-xr-x 3 root root      4096 Mar 19 17:24 ./
drwx------ 9 root root      4096 Mar 20 09:54 ../
-rw-r--r-- 1 root root    256633 Mar 19 16:53 genesis.blob
-rwxr-xr-x 1 root root 101822080 Mar 19 16:50 mgo-node*
-rw-r--r-- 1 root root      1155 Mar 19 17:24 node.yaml


Usage: mgo-node [OPTIONS] --config-path <CONFIG_PATH>

Options:
      --config-path <CONFIG_PATH>                              
      --listen-address <LISTEN_ADDRESS>                        Specify address to listen on
      --run-with-range-epoch <RUN_WITH_RANGE_EPOCH>            
      --run-with-range-checkpoint <RUN_WITH_RANGE_CHECKPOINT>  
  -h, --help                                                   Print help
  -V, --version                                                Print version

```

2. Start node
```
Granted permission
chmod +x mgo-node
export PATH=/root/mango:$PATH

mgo-node --config-path ./node.yaml

---
db-path: "/root/mango/db"
network-address: "/ip4/0.0.0.0/tcp/8080/http"
metrics-address: "0.0.0.0:9184"
json-rpc-address: "0.0.0.0:9000"
enable-event-processing: true
p2p-config:
  listen-address: "0.0.0.0:8084"
  external-address: /ip4/<ip>/udp/8084
  seed-peers:
    - peer-id: 9eed9adca100cdf5d4cf71078bdcbf752a4eb3001e550b5f643be6fea2a80168
      address: /dns/validator1.devnet.mangonetwork.io/udp/8084/http
    - peer-id: ff550ebd44ba4b4521289045f1beb99447c60cda7606ac7cb059a11e14394f70
      address: /dns/validator2.devnet.mangonetwork.io/udp/8084/http
    - peer-id: 87671fae328e75d22bef6aa5790f42844dddfb1abe486351a289d649453aeb53
      address: /dns/validator3.devnet.mangonetwork.io/udp/8084/http
    - peer-id: a8ff5bac98c0626979276c32dd002c8c460e927fee7f48d45b2bcc9dab7d74a3
      address: /dns/validator4.devnet.mangonetwork.io/udp/8084/http
genesis:
  genesis-file-location: "/root/mango/genesis.blob"
authority-store-pruning-config:
  num-latest-epoch-dbs-to-retain: 3
  epoch-db-pruning-period-secs: 3600
  num-epochs-to-retain: 1
  max-checkpoints-in-batch: 10
  max-transactions-in-batch: 1000
  pruning-run-delay-seconds: 60

```
access http://ip:9000/
Appear Connection header did not include 'upgrade'
