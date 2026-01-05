How to Self-host Supabase: A complete guide
#
supabase
#
tutorial
#
nginx
#
ssl
In this guide, I will explain how to self-host Supabase. Including how to purchase and secure a virtual private server (VPS), install and set up Supabase, Set up a reverse proxy using Nginx, and get a secure socket layer (SSL) certificate to accept HTTPS traffic.

This is my first time writing a complementary guide, so if there is anything unclear, or you have any feedback, please leave it in the comment below.

What you will achieve
If you follow the steps, you will end up having a password-protected Supabase server hosted on a VPS, a reverse proxy that redirects traffic from the domain name to the respective endpoint on the Supabase back end with an SSL certificate.

To continue, you will need:

A domain name, free or paid, that you can set the A or CNAME record

A VPS running Ubuntu 20+, if you don't already this guild will also introduce one

Buy a VPS and set up
To self-host a Supabase back end, obviously, you need a VPS. If you already have one, you can move on to the section about securing the VPS and see if there is more you can do to secure your VPS. If not, the following guide will talk about how to purchase one.

Purchasing a VPS
I will be using OxideHost VPS because they are relatively cheap. I've been using them and they are pretty reliable. For those interested, this is my affiliate link.

After choosing the VPS and purchasing it with Ubuntu 22.04 installed, an email is sent to give me information about how to connect to my VPS. The most important part is the hostname, IP, and default password.

Connect to it
ssh {username}@{ip-address}

To connect to the VPS I just purchased, the simplest way is to use the ssh command in the command line. However, using this method, we will have to type this command and enter the password every time. A better method would be to use SSH clients such as PuTTY which is free. I will be using Termius since it has a nice interface and is free for students.



I will create a new host, inputting the hostname given by the email, and username which is root by default, and the password was given to me. It will ask you a question about do you want to connect to it, click yes. Then I will have connected to the VPS for the first time.



Securing the VPS
Now that we are connected to our VPS, we want to make sure we are the only people having access to it, so we need to take measures to secure it.

update its software
apt update
apt dist-upgrade

The first task is to update it since updated software usually has its security vulnerabilities patches. This also upgrades the Linux distribution. There will be a few questions the command prompt asks you, you can type y and then press enters to say yes to them. and if it is a multiple choice, you can press tab to navigate around, and press enters to confirm the selection.



Add another user
Now that you acquired your VPS, hackers also want to get access to it. There are tons of robots on the internet cracking passwords of random VPS, they usually try to log in as root since it is the default configuration. So we will add a new user, pass the root permission to it, and disable root login so those bad robots won't be able to log in to our VPS.

adduser {username}
usermod -aG sudo {username}

The username cannot have spaces within, so you can use an underscore like learn_supabase or learnsupabase . Then it will ask us some questions, we can input blank if we don't want to answer them. But we need to fill in a secure password , I recommend inputting a randomly generated password using some secure password generators so that there is no way for anyone to guess it. Then we can give the user Sudo privileges. And try to log in using the new username and password by creating a new host and connecting it.

Disable the root login
nano /etc/ssh/sshd_config



Now we need to change the Port to some random number to make it harder to guess, the number must be smaller than 65535. And we also change the link PermitRootLogin to no, so we can no longer log in as root.

After making the change, we can do ctrl + o to write out the changes, and ctrl + x to exit nano. Then we do the following command to restart the sshd service, so the new configuration will come into effect.

systemctl restart sshd

Install Supabase
Install docker
We need docker for this, a tutorial about how to install docker can be found here. In this tutorial, I will summarize what commands we need to run. This command assumes you are using Ubuntu, if you are using other distros, please refer to the above tutorial link.

sudo apt install apt-transport-https ca-certificates curl gnupg lsb-release
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt update
sudo apt install docker-ce docker-ce-cli containerd.io
sudo apt install docker-compose-plugin

Then, we also need to enable the user to use docker.

sudo usermod -aG docker {username}

Clone Supabase
Then we want to clone the Supabase from Github.

git clone --depth 1 https://github.com/supabase/supabase

Set up Supabase secrets
cd supabase/docker/
cp .env.example .env

Now we get inside the docker directory and copy the file for the environment variables.

Generate passwords
We open the .env file, and we will see a lot of settings.

nano .env



We want to set the passwords for the above fields. For POSTGRES_PASSWORD , you can use a randomly generated password. For the JWT secret, we have to use the custom key generator. you can copy the JWT secret and paste it into the file, and then we can select ANON_KEY , hit generate, copy it to the ANON_KEY field, then the SERVICE_KEY field.



nano volumes/api/kong.yml

Then we also copy the same key to the kong config, pasting the new anon key and service key.



Start docker
Now we open a new screen and name it so that we can connect to it later

screen

Here you will see the introduction of the screen, hit enter . Then we can do ctrl + a then ctrl + d to disconnect from the screen. In the console, you will see your screen number [detached from 3775963.pts-0.hostname] , the 3775963 number is the screen number.

screen -S {screen_number} -X sessionname supabase
screen -r supabase

Now we have reconnected to screen supabase , when we disconnect from it, we use ctrl + a then ctrl + d .

cd supabase/docker/
docker compose up

Now the supabase docker is started, it will download a bunch of things, and you can connect to your domain in port 3000.

Domain mapping
Now I want to point a custom domain to the backend. I used Namecheap, and I create an A record, pointing a subdomain to the IP address of my VPS.



Reverse proxy
Reverse proxy points the connections to the correct port. The part is explained very well in the Linode blog post, so I am only going to briefly talk about what commands needed to be run, you can refer to their blog post for a more detailed explanation.

We first install Nginx for reverse proxy.

sudo apt install nginx
sudo systemctl status nginx

If we connect to our domain, we will see the Nginx welcome page.



sudo nano /etc/nginx/sites-available/{your_domain_name}

then we copy the following content (from the Linode guide) into the file, replacing the example.com with your custom domain.

map $http_upgrade $connection_upgrade {
    default upgrade;
    '' close;
}

upstream supabase {
    server localhost:3000;
}

upstream kong {
    server localhost:8000;
}

server {
    listen 80;
    server_name example.com;

    # REST
    location ~ ^/rest/v1/(.*)$ {
        proxy_set_header Host $host;
        proxy_pass http://kong;
        proxy_redirect off;
    }

    # AUTH
    location ~ ^/auth/v1/(.*)$ {
        proxy_set_header Host $host;
        proxy_pass http://kong;
        proxy_redirect off;
    }

    # REALTIME
    location ~ ^/realtime/v1/(.*)$ {
        proxy_redirect off;
        proxy_pass http://kong;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection $connection_upgrade;
        proxy_set_header Host $host;
    }

    # STUDIO
    location / {
        proxy_set_header Host $host;
        proxy_pass http://supabase;
        proxy_redirect off;
        proxy_set_header Upgrade $http_upgrade;
    }
}

After that, we can do ctrl + o and ctrl + x to write and exit. Then we can create a symbolic link of the config from sites-available to sites-enabled, link the default config, and then restart Nginx.

sudo ln -s /etc/nginx/sites-available/{your_domain_name} /etc/nginx/sites-enabled/{your_domain_name}
sudo unlink /etc/nginx/sites-enabled/default
sudo systemctl restart nginx

Now, we can connect to Supabase using the domain name.

Get SSL
In this part, we will use Certbot to give our Supabase an SSL certificate. Their official website have pretty good instructions, so you can follow that. But I'll also let you know what commands I used.

We first install core, then certbot. Then we configure it, and at last try how it auto-renews, and then restart Nginx for it to take effect.

sudo snap install core; sudo snap refresh core
sudo snap install --classic certbot
sudo ln -s /snap/bin/certbot /usr/bin/certbot
sudo certbot --nginx
sudo certbot renew --dry-run
sudo systemctl restart nginx



Secure Supabase
The Supabase dashboard everyone can connect to, to secure it, we do this to generate the password. It will ask you to type in the password two times, so it is best you generate a password from a secure password generator and then paste it there twice.

sudo htpasswd -c /etc/apache2/.htpasswd {admin_username}

Then we can check the password with

cat /etc/apache2/.htpasswd

Then we edit the Nginx config to password protect

sudo nano /etc/nginx/sites-available/{domain name}

and make it so that in the studio

    # STUDIO
    location / {
        proxy_set_header Host $host;
        proxy_pass http://supabase;
        proxy_redirect off;
        proxy_set_header Upgrade $http_upgrade;

        auth_basic "Administrators Area";
        auth_basic_user_file /etc/apache2/.htpasswd;
    }

at last restart

sudo service nginx restart



Here you can fill in the username as the {admin_username} you put in, and the password as the password you filled in.

Reconfigure Supabase
Now, we want to set the domain for the Supabase backend and external API.

screen -r supabase

Then we press ctrl + c to end the Supabase docker instance. Then we edit the .env file.

nano .env

We need to change the

...
API_EXTERNAL_URL=https://{your_domain}
...
SUPABASE_PUBLIC_URL=https://{your_domain} # replace if you intend to use Studio outside of localhost

Then we can do ctrl + o and ctrl + x to save the changes, then do the following to get Supabase back up.

docker compose up

Then we can exit the screen by doing ctrl + a then ctrl + d to disconnect without terminating the process inside.