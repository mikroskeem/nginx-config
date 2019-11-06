# Another example configuration

```
server {
	include snippets/listen_80.conf;
	server_name domain.tld;

	include snippets/https_redirect.conf;
}

server {
	include snippets/listen_80.conf;
	server_name www.domain.tld;

	#include snippets/wwwless_https_redirect.conf;
	return 301 https://domain.tld$request_uri;
}

server {
	include snippets/listen_443.conf;
	server_name www.domain.tld;

	ssl_certificate /etc/letsencrypt/live/www.domain.tld/fullchain.pem;
	ssl_trusted_certificate /etc/letsencrypt/live/www.domain.tld/chain.pem;
	ssl_certificate_key /etc/letsencrypt/live/www.domain.tld/privkey.pem;
	include snippets/full_hsts.conf;

	#include snippets/wwwless_https_redirect.conf;
	return 301 https://domain.tld$request_uri;
}

server {
	include snippets/listen_443.conf;
	server_name domain.tld;

	ssl_certificate /etc/letsencrypt/live/domain.tld/fullchain.pem;
	ssl_trusted_certificate /etc/letsencrypt/live/domain.tld/chain.pem;
	ssl_certificate_key /etc/letsencrypt/live/domain.tld/privkey.pem;
	include snippets/full_hsts.conf;

	root /var/www/html/domain.tld;
	index index.html;

	location / {
		try_files $uri $uri/ =404;
	}
}
```
