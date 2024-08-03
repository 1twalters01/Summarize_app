export interface LoginProps {
    /** Go to the email screen */
    emailMode?: () => number;
    /** Go to the password screen */
    passwordMode?: () => number;
    /** Go to the totp screen */
    totpMode?: () => number;
}

