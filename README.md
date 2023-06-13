# fluvio to Redis connector (experimental)

The connector dump records (assumed JSON, see fluvio smart modules) from fluvio topics into Redis JSON 

to run locally using local Redis installation 

## Install Fluvio and Fluvio Connector Development Kit (CDK)
Install Fluvio:
```
curl -fsS https://packages.fluvio.io/v1/install.sh | bash
```
following tutorial [here](https://www.fluvio.io/connectors/cdk/overview/) install CDK
```
fluvio install cdk
```

## Build connector 

Build connector:
```
cdk build
```

## Run connector
```
RUST_BACKTRACE=1 cdk test -c config-example.yaml --secrets secrets.txt
```
change REDIS_URL inside secrets.txt

For running in the infinyon cloud set REDIS_URL via `fluvio cloud secret` command
```
fluvio cloud secret set -c REDIS_URL redis://
```
to check:
```
fluvio cloud secret list
 Secret Name  Last Modified 
 REDIS_URL    2023-05-01    
```

change topic in config-example.yaml
```
meta:
  version: 0.1.0
  name: my-redis-connector-sink-test-connector
  type: redis-connector-sink-sink
  topic: hackernews
redis:
  prefix: hackernews
  url:
    secret:
      name: "REDIS_URL"
  to_hash: false
```
if you want to listen to the topic different to hackers news 
Change `to_hash` to `true` to save records as Hash instead of JSON. 

Follow up [quickstart](https://www.fluvio.io/connectors/cdk/overview/) to build you own connector 

# Mac test for RedisTimeSeries
To test on mac:
redis-server /opt/homebrew/Caskroom/redis-stack-server/6.2.6-v6/etc/redis-stack-service.conf

Config
```
cat /opt/homebrew/Caskroom/redis-stack-server/6.2.6-v6/etc/redis-stack-service.conf                                                                                
port 6379
daemonize no
protected-mode no
loadmodule /opt/homebrew/Caskroom/redis-stack-server/6.2.6-v6/lib/redisearch.so
loadmodule /opt/homebrew/Caskroom/redis-stack-server/6.2.6-v6/lib/redisgraph.so
loadmodule /opt/homebrew/Caskroom/redis-stack-server/6.2.6-v6/lib/redistimeseries.so DUPLICATE_POLICY LAST
loadmodule /opt/homebrew/Caskroom/redis-stack-server/6.2.6-v6/lib/rejson.so
loadmodule /opt/homebrew/Caskroom/redis-stack-server/6.2.6-v6/lib/redisbloom.so
```
