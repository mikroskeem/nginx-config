# Deployment

## Requirements
As mentioned in README, you need mainline nginx (>= 1.17.0) and also latest openssl.
Security **IS** important, do not leave systems outdated ;)

Obvious requirements are:
1) openssl
2) nginx
3) curl (for debugging)
4) git (for cloning this repository)

### Installing newer nginx

While I don't see a reason for describing all the installation steps (you must know how to do things yourself and not
rely on third party scripts doing all the fun instead of you), I add few notes about nginx here for sure.

#### Official nginx binary builds

nginx provides binary builds for certain distributions, [check them out](https://nginx.org/en/linux_packages.html)

#### Unofficial, but distribution supported packages

Arch Linux provides [nginx-mainline](https://www.archlinux.org/packages/community/x86_64/nginx-mainline/) package by themselves, however
it might lag behind mainline releases as it's not maintained by nginx team.

#### Build from source

Pretty straightforward if you've built packages from sources before. Use your favourite search engine and resources to figure out, if you haven't
done this before.

## Actually deploying it

**NB!** Assuming that you are on clean installation (tested with Debian 10)

```sh
git clone https://github.com/mikroskeem/nginx-config
cd nginx-config
sudo scripts/deploy.sh
```

And that's it. Script checks for required files and does most of the needed stuff on its own.  
After that you only need to enable/start `nginx.service` and start setting up the first vhost
