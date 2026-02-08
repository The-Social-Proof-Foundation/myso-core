# Run a MySo Node using Systemd

Tested using:
- Ubuntu 20.04 (linux/amd64) on bare metal
- Ubuntu 22.04 (linux/amd64) on bare metal

## Prerequisites and Setup

1. Add a `myso` user and the `/opt/myso` directories

```shell
sudo useradd myso
sudo mkdir -p /opt/myso/bin
sudo mkdir -p /opt/myso/config
sudo mkdir -p /opt/myso/db
sudo mkdir -p /opt/myso/key-pairs
sudo chown -R myso:myso /opt/myso
```

2. Install the MySo Node (myso-node) binary, two options:
    
- Pre-built binary stored in Amazon S3:
        
```shell
wget https://releases.myso.io/$MYSO_SHA/myso-node
chmod +x myso-node
sudo mv myso-node /opt/myso/bin
```

- Build from source:

```shell
git clone https://github.com/MystenLabs/myso.git && cd myso
git checkout $MYSO_SHA
cargo build --release --bin myso-node
mv ./target/release/myso-node /opt/myso/bin/myso-node
```

3. Copy your key-pairs into `/opt/myso/key-pairs/` 

If generated during the Genesis ceremony these will be at `MySoExternal.git/myso-testnet-wave3/genesis/key-pairs/`

Make sure when you copy them they retain `myso` user permissions. To be safe you can re-run: `sudo chown -R myso:myso /opt/myso`

4. Update the node configuration file and place it in the `/opt/myso/config/` directory.

Add the paths to your private keys to validator.yaml. If you chose to put them in `/opt/myso/key-pairs`, you can use the following example: 

```
protocol-key-pair: 
  path: /opt/myso/key-pairs/protocol.key
worker-key-pair: 
  path: /opt/myso/key-pairs/worker.key
network-key-pair: 
  path: /opt/myso/key-pairs/network.key
```

5. Place genesis.blob in `/opt/myso/config/` (should be available after the Genesis ceremony)

6. Copy the myso-node systemd service unit file 

File: [myso-node.service](./myso-node.service)

Copy the file to `/etc/systemd/system/myso-node.service`.

7. Reload systemd with this new service unit file, run:

```shell
sudo systemctl daemon-reload
```

8. Enable the new service with systemd

```shell
sudo systemctl enable myso-node.service
```

## Connectivity

You may need to explicitly open the ports outlined in [MySo for Node Operators](../myso_for_node_operators.md#connectivity) for the required MySo Node connectivity.

## Start the node

Start the Validator:

```shell
sudo systemctl start myso-node
```

Check that the node is up and running:

```shell
sudo systemctl status myso-node
```

Follow the logs with:

```shell
journalctl -u myso-node -f
```

## Updates

When an update is required to the MySo Node software the following procedure can be used. It is highly **unlikely** that you will want to restart with a clean database.

- assumes myso-node lives in `/opt/myso/bin/`
- assumes systemd service is named myso-node
- **DO NOT** delete the MySo databases

1. Stop myso-node systemd service

```
sudo systemctl stop myso-node
```

2. Fetch the new myso-node binary

```shell
wget https://releases.myso.io/${MYSO_SHA}/myso-node
```

3. Update and move the new binary:

```
chmod +x myso-node
sudo mv myso-node /opt/myso/bin/
```

4. start myso-node systemd service

```
sudo systemctl start myso-node
```
