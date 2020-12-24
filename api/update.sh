cargo build --release
sudo systemctl stop vault_api
sudo cp target/release/vault_api /usr/sbin/vault_api
sudo cp ../targets.txt /etc/vault_api/targets.txt
sudo systemctl start vault_api