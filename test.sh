curl --header "Content-Type: application/json" \
	 --request POST \
	 --data '{"y":2021,"m":11,"d":26,"h":17,"averageTemp":"23"}' \
	 http://192.168.0.100:8080/add_last_temp
