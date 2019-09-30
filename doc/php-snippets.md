# PHP configuration snippets

Only PHP-FPM support over Unix sockets is provided right now.

## Usage

You should use:
- `php7.3-fpm.conf` - When on Debian
- `php-arch-fpm.conf` - When on Arch Linux

Only difference between them is `fastcgi_pass` Unix socket path

You need to supply your own:
- `try_files`
- `root` - Of course!

### Example configuration

```nginx
server {
    # ...

    location / {
        try_files $uri $uri/ =404;
    }

    location ~ [^/]\.php(/|$) {
        include snippets/php/php7.3-fpm.conf;
        try_files $fastcgi_script_name =404;
    }
}
```

### Changing Unix socket path or using TCP

Set `fastcgi_pass` and include `_php_fcgi_base`

## Tips

### File uploads
If you have file uploads, then consider setting:
1) `fastcgi_param PHP_VALUE "upload_max_filesize = 100M \n post_max_size=105M";` - Note: PHP POST max size should always be larger than upload max filesize
2) `client_max_body_size 110M;` - Note: this has to be bit larger than PHP's

### `cgi.fix_pathinfo`
It's common practice to keep the `cgi.fix_pathinfo` value `0` in PHP FPM config file, since PHP-FPM doesn't really take advantage of this and is considered a potential security issue.

See: https://serverfault.com/questions/627903/is-the-php-option-cgi-fix-pathinfo-really-dangerous-with-nginx-php-fpm

```
$ sed -i "s/;cgi.fix_pathinfo=1/cgi.fix_pathinfo=0/g" /etc/php/*/fpm/php.ini
```

This shouldn't break anything, unless you happen to run things that use `$_SERVER["PHP_SELF"]`
