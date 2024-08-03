import { postEmail, postPassword, postTotp } from './post';
import { setCookie, deleteCookie } from '../../../utils/cookies';
import { getKeyByValue } from '../../../utils/objects';
import { accounts as emailAccounts } from '../../../protos/accounts/login/email/response.ts';
import { accounts as passwordAccounts } from '../../../protos/accounts/login/password/response.ts';
import { accounts as totpAccounts } from '../../../protos/accounts/login/totp/response.ts';

/** @typedef { import('../types/Props').LoginProps } Props */

/**
 * @param {SubmitEvent} e
 * @param {String} email
 * @param {Props} props
 */
export async function handlePostEmail(e, email, props) {
  e.preventDefault();

  let arrayBuffer = await postEmail(email);
  let uint8Array = new Uint8Array(arrayBuffer);
  let response =
    emailAccounts.login.email.response.Response.deserializeBinary(uint8Array);

  if (response.has_token) {
    let token = response.token;
    setCookie('login_email_token', token, 5);
    props.passwordMode?.();
  }

  if (response.has_error) {
    console.error(
      getKeyByValue(emailAccounts.login.email.response.Error, response.error)
    );
  } else {
    console.error('Client Error');
  }
}

/**
 * @param {SubmitEvent} e
 * @param {String} password
 * @param {Boolean} rememberMe
 * @param {Props} props
 * @param {import('solid-js').Setter<String>} setEmail
 * @param {Function} navigate
 */
export async function handlePostPassword(
  e,
  password,
  rememberMe,
  setEmail,
  navigate,
  props
) {
  e.preventDefault();

  let array_buffer = await postPassword(password, rememberMe);
  let uint8Array = new Uint8Array(array_buffer);
  let response =
    passwordAccounts.login.password.response.Response.deserializeBinary(
      uint8Array
    );

  if (response.has_success) {
    let success = response.success;

    if (success.requires_totp) {
      setCookie('login_password_token', success.token.response, 1800);
      props.totpMode?.();
    } else {
      let tokens = success.token.tokens;
      let bearer_token = 'Bearer ' + tokens.access;
      setCookie('Authorization', bearer_token, 1800);

      if (tokens.has_refresh) {
        setCookie('Refresh', tokens.refresh, 18000);
      }
    }

    setEmail('');
    deleteCookie('login_email_token');
    navigate('/home/', { replace: true });
  }

  if (response.has_error) {
    console.error(
      getKeyByValue(
        passwordAccounts.login.password.response.Error,
        response.error
      )
    );
  } else {
    console.error('Client Error');
  }
}

/**
 * @param {SubmitEvent} e/
 * @param {number[]} digits The totp's digits
 * @param {Function} setEmail Function to change the email
 * @param {Function} navigate
 */
export const handlePostTotp = async (e, digits, setEmail, navigate) => {
  e.preventDefault();
  let array_buffer = await postTotp(
    digits[0],
    digits[1],
    digits[2],
    digits[3],
    digits[4],
    digits[5]
  );
  let uint8Array = new Uint8Array(array_buffer);
  let response =
    totpAccounts.login.totp.response.Response.deserializeBinary(uint8Array);

  if (response.has_tokens) {
    let tokens = response.tokens;
    let bearer_token = 'Bearer ' + tokens.access;
    setCookie('Authorization', bearer_token, 1800);

    if (tokens.has_refresh) {
      setCookie('Refresh', tokens.refresh, 18000);
    }

    setEmail('');
    deleteCookie('login_email_token');
    navigate('/home/', { replace: true });
  }
  if (response.has_error) {
    console.error(
      getKeyByValue(totpAccounts.login.totp.response.Error, response.error)
    );
  } else {
    console.error('Client Error');
  }
};
