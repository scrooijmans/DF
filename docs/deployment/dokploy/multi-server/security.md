# Security | Dokploy
Security features of Dokploy

Dokploy provides comprehensive security recommendations to protect your remote server. Our security checks ensure your server follows best practices for a secure deployment environment.

### [Operating System](#operating-system)

*   Currently supports Ubuntu/Debian OS (Experimental)
*   Regular system updates recommended

### [UFW (Uncomplicated Firewall)](#ufw-uncomplicated-firewall)

UFW is an essential security component that manages incoming and outgoing network traffic.

**Recommended Configuration:**

*   ✅ UFW should be installed
*   ✅ UFW should be active
*   ✅ Default incoming policy should be set to 'deny'
*   ✅ Only necessary ports should be opened

### [SSH Security](#ssh-security)

Secure Shell (SSH) configuration is crucial for safe remote server access.

**Best Practices:**

*   ✅ SSH service should be enabled
*   ✅ Key-based authentication should be enabled
*   ❌ Password authentication should be disabled
*   ❌ PAM should be disabled when using key-based authentication
*   ✅ Use non-standard SSH port (optional)

### [Fail2Ban Protection](#fail2ban-protection)

Fail2Ban helps prevent brute force attacks by temporarily banning IPs that show malicious behavior.

**Recommended Setup:**

*   ✅ Fail2Ban should be installed
*   ✅ Service should be enabled and running
*   ✅ SSH protection should be enabled
*   ✅ Use aggressive mode for enhanced security

Dokploy automatically validates these security configurations and provides recommendations:

![Security](https://docs.dokploy.com/_next/image?url=%2Fassets%2Fimages%2Fserver-security.png&w=3840&q=75)

These security measures are essential baseline recommendations. Depending on your specific use case, additional security measures might be necessary.