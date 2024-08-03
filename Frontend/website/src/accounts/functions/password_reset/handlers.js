import { postEmail, postVerification, postPasswords } from './post';
import { deleteCookie, setCookie } from '../../../utils/cookies';
import { getKeyByValue } from '../../../utils/objects';
import { accounts as emailAccounts } from '../../../protos/accounts/password_reset/email/response.ts';
import { accounts as verificationAccounts } from '../../../protos/accounts/password_reset/verification/response.ts';
import { accounts as passwordAccounts } from '../../../protos/accounts/password_reset/password/response.ts';

/** @typedef { import('../../types/Props').PasswordResetProps } Props */

/**
 * @param {SubmitEvent} e
 * @param {string} email The user's email address
 * @param {Props} props
 */
export async function handlePostEmail(e, email, props) {
    e.preventDefault();

    let arrayBuffer = await postEmail(email);
    let uint8Array = new Uint8Array(arrayBuffer);
    let response
        = emailAccounts.password_reset.email.response.Response.deserializeBinary(uint8Array);

    if (response.has_token) {
        let token = response.token;
        setCookie('password_reset_email_token', token, 5);
        props.verificationMode?.();
    } else {
        if (response.has_error) {
            console.error(
                getKeyByValue(emailAccounts.password_reset.email.response.Error, response.error)
            );
        } else {
            console.error('Client Error');
        }
    }
}

/**
 * @param {SubmitEvent} e
 * @param {string} token The user's token
 * @param {Props} props
 */
export const handlePostVerification = async (e, token, props) => {
  e.preventDefault();

  let arrayBuffer = await postVerification(token);
  let uint8Array = new Uint8Array(arrayBuffer);
  let response = verificationAccounts.password_reset.verification.response.Response.deserializeBinary(uint8Array);

    if (response.has_token) {
        let token = response.token;
        setCookie('password_reset_verification_token', token, 1800);
        deleteCookie('password_reset_email_token');
        props.passwordMode?.();
    } else {
        if (response.has_error) {
            console.error(
                getKeyByValue(
                    verificationAccounts.password_reset.verification.response.Error,
                response.error)
            );
        } else {
            console.error('Client Error');
        }
    }
}

/**
 * @param {SubmitEvent} e
 * @param {string} password The user's new password
 * @param {string} passwordConfirmation Confirmation of the user's new password
 * @param {Function} navigate
 */
export const handlePostPasswords = async (e, password, passwordConfirmation, navigate) => {
    e.preventDefault();
    let arrayBuffer = await postPasswords(password, passwordConfirmation);
    let uint8Array = new Uint8Array(arrayBuffer);
    let response = passwordAccounts.password_reset.password.response.Response.deserializeBinary(uint8Array);

    if (response.has_success) {
        deleteCookie('password_reset_verification_token');
        navigate('/login/', { replace: true });
    } else {
        if (response.has_error) {
            console.error(
                getKeyByValue(passwordAccounts.password_reset.password.response.Error, response.error));
        } else {
            console.error('Client Error');
        }
    }
}
