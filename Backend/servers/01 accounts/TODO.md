# Todo
* Don't allow access tokens to go to users - Store them as a key: opaque_token, value: access_token and handle in authentication middleware to put the uuid in the req
* Add a tag when saving things to redis for the route that saved it to differentiate them?
* Think through flow for adding regular user to an oauth account
* Change queries to match new structure for accounts
* oauth2
* Write tests for:
    * models
    * queries
    * routes
    * services
    * views
* Real world testing

# CheckList
1. [ ] User Management
    * [ ] User Registration - Endpoints for new users to sign up, including:
        * [x] form validations
        * [x] captcha verfication
        * [x] email confirmation
        * [ ] phone number confirmation
    * [ ] User Login - Mechanisms for users to log in using:
        * [x] email and password
        * [x] - guest login
        * [ ] OAuth/social logins
            * [ ] Google
            * [ ] Apple
    * [x] Password Recovery - This includes secure token generation

2. [ ] Authentication
    * [x] Password Authentication using Argon2
    * [ ] MFA
        * [ ] 2FA
            * [x] TOTP authenticator app
            * [ ] OTP SMS
            * [ ] biometrics
        * [ ] login via email link
    * [ ] OAuth Connect
    * [ ] OpenID Connect
    * [ x] Session Management - Secure handling of user sessions, including:
        * [x] session expiration
        * [x] session renewal
        * [ ] session invalidation
    * [x] Token-Based Authentication - Support for JWTs for stateless authentication
    * [ ] Single Sign-On (SSO) integration

3. [x] Authorization
    * [x] Role-Based Access Control (RBAC) - Implementation of roles and permissions to manage access control
    * [x] Attribute-Based Access Control (ABAC) - More granular access control based on user attributes and context
    * [x] Access Control Lists (ACLs) - Management of permissions for different resources

4. [ ] Security
    * [x] Rate Limiting - To protect against brute force attacks
    * [x] CAPTCHA - To prevent automated abuse during:
        * [x] login
        * [x] registration
        * [x] password reset
    * [ ] Audit Logging - Logging of security-related events such as:
        * [ ] Logins
        * [ ] Failed login attempts
        * [ ] MFA setup

5. [ ] Compliance
    * [ ] GDPR/CCPA Compliance - ensure user data handling complies with data protection regulations
    * [ ] Audit Trails - keep detailed records of user actions for compliance and security audits

6. [ ] Administrative Tools

7. [ ] APIs
    * [ ] Public APIs
    * [ ] Admin APIs
        * [ ] managing users
        * [ ] managing roles
        * [ ] managing permissions
    * [x] Security APIs
        * [x] Token issuance

8. [ ] Notifications
    * [ ] Email/SMS Notifications - For account-related activities such as:
        * [x] registration confirmation
        * [x] password reset
        * [ ] login alerts

9. [ ] Documentation and Support
    * [ ] User Guides - Instructions for end-users on managing their accounts
    * [ ] API Documentation: Detailed documentation for developers using the authentication and user management APIs.

10. [ ] Integration and Extensibility
