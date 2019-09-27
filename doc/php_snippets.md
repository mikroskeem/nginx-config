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


### Tips
If you have file uploads, then consider setting:
1) `fastcgi_param PHP_VALUE "upload_max_filesize = 100M \n post_max_size=100M";`
2) `client_max_body_size 105M;` - Note: this has to be bit larger than PHP's

### Changing Unix socket path or using TCP

Set `fastcgi_pass` and include `_php_fcgi_base`
