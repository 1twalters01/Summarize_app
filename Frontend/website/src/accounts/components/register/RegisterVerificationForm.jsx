import { createSignal } from 'solid-js';
import { setCookie, deleteCookie } from '../../../utils/cookies';
import { postVerification } from '../../functions/register/post.js';
const {
  Response: registerResponse
} = require('../../../protos/accounts/register/verification/response.ts');

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

/** @typedef {Object} props
 * @property {Function} emailMode - go to first screen
 * @property {Function} detailsMode - go to the next screen
 */

/**
 * @param {Accessor<string>} token The user's token
 * @param {props} props
 */
const postRegister = async (token, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postVerification(token).then((arrayBuffer) => {
    let uint8Array = new Uint8Array(arrayBuffer);

    let response, token, error;
    try {
      response = registerResponse.deserializeBinary(uint8Array);
      error = response.getError();
      console.log(response);
      console.log('token:', response.getToken());
      if (response.hasToken()) {
        token = response.getToken();
        setCookie('register_verification_token', token, 1800);
        deleteCookie('register_email_token');
        props.detailsMode();
      }
    } catch (decodeError) {
      console.error('Error decoding response:', decodeError);
      throw decodeError;
    }
  });

  return response;
};

/** @param {props} props */
const RegisterVerificationForm = (props) => {
  /** @type {Signal<String>} */
  const [token, setToken] = createSignal('');

  /** @param {SubmitEvent} e */
  function PostRegister(e) {
    e.preventDefault();
    console.log('token: ', token());

    let response = postRegister(token, props);
    response.then((response) => console.log('response: ', response));
  }

  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode()}>
        x
      </button>

      <form onSubmit={PostRegister}>
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

export default RegisterVerificationForm;
