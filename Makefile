static=/var/www/static
logs=/var/www/logs

migrate:
	rm -rf $(static)
	mkdir -p $(static)
	mkdir -p $(logs)
	cp ./media/* $(static)
