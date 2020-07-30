cargo build --release
sudo systemctl stop vault_api
sudo cp target/release/vault_api /usr/sbin/vault_api
sudo systemctl start vault_api