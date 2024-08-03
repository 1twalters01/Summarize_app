export interface LoginProps {
    /** Go to the email screen */
    emailMode?: () => number;
    /** Go to the password screen */
    passwordMode?: () => number;
    /** Go to the totp screen */
    totpMode?: () => number;
}

export interface PasswordResetProps {
    /** Go to the email screen */
    emailMode?: () => number;
    /** Go to the verification screen */
    verificationMode?: () => number;
    /** Go to the password screen */
    passwordMode?: () => number;
}

export interface RegisterProps {
    /** Go to the email screen */
    emailMode?: () => number;
    /** Go to the verification screen */
    verificationMode?: () => number;
    /** Go to the details screen */
    detailsMode?: () => number;
}

