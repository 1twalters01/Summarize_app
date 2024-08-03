import { accounts as emailAccounts } from '../../../protos/accounts/login/email/request.ts';
import { accounts as passwordAccounts } from '../../../protos/accounts/login/password/request.ts';
import { accounts as totpAccounts } from '../../../protos/accounts/login/totp/request.ts';
import { getCookie } from '../../../utils/cookies';

const base_url = process.env.ACCOUNTS_URL;

/** @param {string} email The user's email address */
export const postEmail = async (email) => {
  const request = new emailAccounts.login.email.request.Request({
    email: email
  });
  const buffer = request.serializeBinary();

  const response = await fetch(base_url + 'login/email', {
    method: 'POST',
    mode: 'cors',
    headers: {
      'Content-Type': 'application/x-protobuf'
    },
    body: buffer
  });

  return response.arrayBuffer();
};

/**
 * @param {string} password The user's password
 * @param {boolean} rememberMe The user's remember me status
 */
export const postPassword = async (password, rememberMe) => {
  const request = new passwordAccounts.login.password.request.Request({
    password: password,
    remember_me: rememberMe
  });
  const buffer = request.serializeBinary();

  const response = await fetch(base_url + 'login/password', {
    method: 'POST',
    mode: 'cors',
    headers: {
      'Content-Type': 'application/x-protobuf',
      'Login-Email-Token': getCookie('login_email_token') ?? ''
    },
    body: buffer
  });

  return response.arrayBuffer();
};

/**
 * @param {number} digit1 totp's first digit
 * @param {number} digit2 totp's second digit
 * @param {number} digit3 totp's third digit
 * @param {number} digit4 totp's fourth digit
 * @param {number} digit5 totp's fith digit
 * @param {number} digit6 totp's sixth digit
 */
export const postTotp = async (
  digit1,
  digit2,
  digit3,
  digit4,
  digit5,
  digit6
) => {
  const request = new totpAccounts.login.totp.request.Request({
    digit1: digit1,
    digit2: digit2,
    digit3: digit3,
    digit4: digit4,
    digit5: digit5,
    digit6: digit6
  });
  const buffer = request.serializeBinary();

  const response = await fetch(base_url + 'login/totp', {
    method: 'POST',
    mode: 'cors',
    headers: {
      'Content-Type': 'application/x-protobuf',
      'Login-Password-Token': getCookie('login_password_token') ?? ''
    },
    body: buffer
  });

  return response.arrayBuffer();
};
