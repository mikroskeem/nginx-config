# Example configuration

Deployed on [nginxtest.rip.ee](https://nginxtest.rip.ee)

```
server {
	include snippets/listen_80.conf;
	server_name nginxtest.rip.ee;

	return 301 https://nginxtest.rip.ee$request_uri;
}

server {
	include snippets/listen_443.conf;
	server_name nginxtest.rip.ee;

	ssl_certificate /etc/letsencrypt/live/nginxtest.rip.ee/fullchain.pem;
	ssl_trusted_certificate /etc/letsencrypt/live/nginxtest.rip.ee/chain.pem;
	ssl_certificate_key /etc/letsencrypt/live/nginxtest.rip.ee/privkey.pem;
	include snippets/full_hsts.conf;

	root /var/www/html/nginxtest.rip.ee;
	index index.html;

	location / {
		try_files $uri $uri/ =404;
	}
}
```
