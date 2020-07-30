cargo build --release
sudo cp target/release/vault_api /usr/sbin/vault_api
sudo cp config/vault_api.service /etc/systemd/system/vault_api.service
sudo cp config/vault_api.conf /etc/rsyslog.d/vault_api.conf
sudo mkdir /var/log/vault_api
sudo chown syslog:adm /var/log/vault_api
sudo systemctl daemon-reload
sudo systemctl restart rsyslog
sudo systemctl enable vault_api
sudo systemctl start vault_api