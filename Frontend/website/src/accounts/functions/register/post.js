import { accounts as emailAccounts } from '../../../protos/accounts/register/email/request.ts';
import { accounts as verificationAccounts } from '../../../protos/accounts/register/verification/request.ts';
import { accounts as detailsAccounts } from '../../../protos/accounts/register/details/request.ts';
import { getCookie } from '../../../utils/cookies';

const base_url = process.env.ACCOUNTS_URL;

/** @param {string} email The user's email address */
export const postEmail = async (email) => {
  const request = new emailAccounts.register.email.request.Request({
    email: email
  });
  const buffer = request.serializeBinary();

  const response = await fetch(base_url + 'register/email', {
    method: 'POST',
    mode: 'cors',
    headers: {
      'Content-Type': 'application/x-protobuf'
    },
    body: buffer
  });

  return response.arrayBuffer();
};

/** @param {string} code The user's token */
export const postVerification = async (code) => {
  const request =
    new verificationAccounts.register.verification.request.Request({
      verification_code: code
    });
  const buffer = request.serializeBinary();

  const response = await fetch(base_url + 'register/verify', {
    method: 'POST',
    mode: 'cors',
    headers: {
      'Content-Type': 'application/x-protobuf',
      'Register-Email-Token': getCookie('register_email_token') ?? ''
    },
    body: buffer
  });

  return response.arrayBuffer();
};

/**
 * @param {string} username The user's username
 * @param {string} password The user's password
 * @param {string} passwordConfirmation Confirmation of the user's password
 * @param {string} firstName The user's first name (optional)
 * @param {string} lastName The user's last name (optional)
 */
export const postDetails = async (
  username,
  password,
  passwordConfirmation,
  firstName,
  lastName
) => {
  const request = new detailsAccounts.register.details.request.Request({
    username: username,
    password: password,
    password_confirmation: passwordConfirmation,
    first_name: firstName,
    last_name: lastName
  });
  const buffer = request.serializeBinary();

  const response = await fetch('http://127.0.0.1:8000/register/details', {
    method: 'POST',
    mode: 'cors',
    headers: {
      'Content-Type': 'application/x-protobuf',
      'Register-Verification-Token':
        getCookie('register_verification_token') ?? ''
    },
    body: buffer
  });

  return response.arrayBuffer();
};
