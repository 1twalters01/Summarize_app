import { postEmail, postVerification, postDetails } from './post.js';
import { setCookie, deleteCookie } from '../../../utils/cookies';
import { getKeyByValue } from '../../../utils/objects.js';
import {
    accounts as emailAccounts
} from '../../../protos/accounts/register/email/response.ts';
import {
    accounts as verificationAccounts
} from '../../../protos/accounts/register/verification/response.ts';
import {
    accounts as detailsAccounts
} from '../../../protos/accounts/register/details/response.ts';

/** @typedef { import ('../../types/Props').RegisterProps } Props */

/**
 * @param {SubmitEvent} e
 * @param {string} email The user's email address
 * @param {Props} props
 */
export async function handlePostEmail (e, email, props) {
    e.preventDefault();

    let arrayBuffer = await postEmail(email);
    let uint8Array = new Uint8Array(arrayBuffer);
    let response = emailAccounts.register.email.response.Response.deserializeBinary(uint8Array);

    if (response.has_token) {
        let token = response.token;
        setCookie('register_email_token', token, 5);
        props.verificationMode?.();
    } else {
        if (response.has_error) {
            console.error(
                getKeyByValue(emailAccounts.register.email.response.Error, response.error)
            );
        } else {
            console.error('Client Error');
        }
    }
}

/**
 * @param {SubmitEvent} e
 * @param {string} code The user's code
 * @param {Props} props
 */
export async function handlePostVerification(e, code, props) {
    e.preventDefault();

    let arrayBuffer = await postVerification(code);
    let uint8Array = new Uint8Array(arrayBuffer);
    let response = verificationAccounts.register.verification.response.Response.deserializeBinary(uint8Array);

    if (response.has_token) {
        let token = response.token;
        setCookie('register_verification_token', token, 1800);
        deleteCookie('register_email_token');
        props.detailsMode?.();
    } else {
        if (response.has_error) {
            console.error(
                getKeyByValue(verificationAccounts.register.verification.response.Error, response.error)
            );
        } else {
            console.error('Client Error');
        }
    }
}

/**
 * @param {SubmitEvent} e
 * @param {string} username The user's username
 * @param {string} password The user's password
 * @param {string} passwordConfirmation Confirmation of the user's password
 * @param {string} firstName The user's first name (optional)
 * @param {string} lastName The user's last name (optional)
 * @param {import('solid-js').Setter<String>} setEmail Function to change the email
 * @param {Function} navigate
 */
export async function handlePostDetails(
  e,
  username,
  password,
  passwordConfirmation,
  firstName,
  lastName,
  setEmail,
  navigate
) {
    e.preventDefault();

    let arrayBuffer = await postDetails(
    username,
    password,
    passwordConfirmation,
    firstName,
    lastName
    );
    let uint8Array = new Uint8Array(arrayBuffer);
    let response = detailsAccounts.register.details.response.Response.deserializeBinary(uint8Array);

    if (response.has_success) {
        deleteCookie('register_verify_token');
        setEmail('');
        navigate('/login', { replace: true });
    } else {
        if (response.has_error) {
            console.error(
                getKeyByValue(detailsAccounts.register.details.response.Error, response.error)
            );
        } else {
            console.error('Client Error');
        }
    }
}

