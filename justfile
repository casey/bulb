watch:
	cargo watch --clear --exec test

run:
	cargo run

fmt:
	cargo +nightly fmt --all

ping:
	ping6 bulb.tulip.farm

deploy:
	./bin/deploy

ssh:
	ssh root@bulb.tulip.farm

install-unit:
	scp bulb.service root@bulb.tulip.farm:/etc/systemd/system/bulb.service

install-sudoers:
	scp bulb.sudoers root@bulb.tulip.farm:/etc/sudoers.d/bulb
	ssh root@bulb.tulip.farm chmod 0440 /etc/sudoers.d/bulb
