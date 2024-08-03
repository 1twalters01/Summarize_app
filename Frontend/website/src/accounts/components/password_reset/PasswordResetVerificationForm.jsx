import { createSignal } from 'solid-js';
import { getCookie, setCookie, deleteCookie } from '../../../utils/cookies';
import { postVerification } from '../../functions/password_reset/post.js';
const {
  Request: passwordResetRequest
} = require('../../../protos/accounts/password_reset/verification/request.ts');
const {
  Response: passwordResetResponse
} = require('../../../protos/accounts/password_reset/verification/response.ts');

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

/** @typedef {Object} props
 * @property {Function} emailMode - go to the first screen
 * @property {Function} passwordMode - go to next screen
 */

/**
 * @param {Accessor<string>} token The user's token
 * @param {props} props
 */
const postPasswordReset = async (token, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postVerification(token).then((arrayBuffer) => {
    let uint8Array = new Uint8Array(arrayBuffer);

    let response, token, error;
    try {
      response = passwordResetResponse.deserializeBinary(uint8Array);
      error = response.getError();
      if (response.hasToken()) {
        token = response.getToken();
        setCookie('password_reset_verification_token', token, 1800);
        deleteCookie('password_reset_email_token');
        props.passwordMode();
      }
    } catch (decodeError) {
      console.error('Error decoding response:', decodeError);
      throw decodeError;
    }
  });

  return response;
};

/** @param {props} props */
const PasswordResetVerificationForm = (props) => {
  /** @type {Signal<String>} */
  const [token, setToken] = createSignal('');

  /** @param {SubmitEvent} e */
  function PostPasswordReset(e) {
    e.preventDefault();
    console.log('token: ', token());

    let response = postPasswordReset(token, props);
    response.then((response) => console.log('response: ', response));
  }

  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode()}>
        x
      </button>
      <form onSubmit={PostPasswordReset}>
        <input
          type="text"
          placeholder="token"
          onInput={(e) => setToken(e.target.value)}
          required
        />
        <input type="submit" value="Submit" />
      </form>
    </>
  );
};

export default PasswordResetVerificationForm;
