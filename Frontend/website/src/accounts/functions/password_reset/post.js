import { accounts as emailAccounts } from '../../../protos/accounts/password_reset/email/request.ts';
import { accounts as verificationAccounts } from '../../../protos/accounts/password_reset/verification/request.ts';
import { accounts as passwordsAccounts } from '../../../protos/accounts/password_reset/password/request.ts';
import { getCookie } from '../../../utils/cookies.js';

const base_url = process.env.ACCOUNTS_URL;

/** @param {string} email The user's email address */
export const postEmail = async (email) => {
  const request = new emailAccounts.password_reset.email.request.Request({
    email: email
  });
  const buffer = request.serializeBinary();

  const response = await fetch(base_url + 'password-reset/email', {
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
    new verificationAccounts.password_reset.verification.request.Request({
      verification_code: code
    });
  const buffer = request.serializeBinary();

  const response = await fetch(base_url + 'password-reset/verify', {
    method: 'POST',
    mode: 'cors',
    headers: {
      'Content-Type': 'application/x-protobuf',
      'Password-Reset-Email-Token':
        getCookie('password_reset_email_token') ?? ''
    },
    body: buffer
  });

  return response.arrayBuffer();
};

/**
 * @param {string} password The user's password
 * @param {string} passwordConfirmation Confirmation of the user's password
 */
export const postPasswords = async (password, passwordConfirmation) => {
  const request = new passwordsAccounts.password_reset.password.request.Request(
    {
      password: password,
      password_confirmation: passwordConfirmation
    }
  );
  const buffer = request.serializeBinary();

  const response = await fetch(base_url + 'password-reset/password', {
    method: 'POST',
    mode: 'cors',
    headers: {
      'Content-Type': 'application/x-protobuf',
      'password-Reset-Verification-Token':
        getCookie('password_reset_verification_token') ?? ''
    },
    body: buffer
  });

  return response.arrayBuffer();
};
