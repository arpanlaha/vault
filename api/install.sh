cargo build --release
sudo cp target/release/vault_api /usr/sbin/vault_api
sudo cp ../targets.txt /etc/vault_api/targets.txt
sudo cp config/systemd/vault_api.service /etc/systemd/system/vault_api.service
sudo cp config/syslog/vault_api.conf /etc/rsyslog.d/vault_api.conf
sudo mkdir /var/log/vault_api
sudo chown syslog:adm /var/log/vault_api
sudo cp config/cron/vault_api /etc/cron.daily/vault_api
sudo chmod 755 /etc/cron.daily/vault_api
sudo systemctl daemon-reload
sudo systemctl restart rsyslog
sudo systemctl restart cron
sudo systemctl enable vault_api
sudo systemctl start vault_api