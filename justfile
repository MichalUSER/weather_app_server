set dotenv-load := false

rasp:
	cross build --target armv7-unknown-linux-gnueabihf --release
	scp -rp bin pi@192.168.0.110:~/weather-forecast/server
