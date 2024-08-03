import { useEmailContext } from '../../context/EmailContext';
import { setCookie } from '../../../utils/cookies';
import { postEmail } from '../../functions/register/post.js';
const {
  Response: registerResponse
} = require('../../../protos/accounts/register/email/response.ts');

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import('solid-js').Setter<T> } Setter */
/** @template T @typedef { import('solid-js').Signal<T> } Signal */

/** @typedef {Object} props
 * @property {Function} verificationMode - go to next screen
 */

/**
 * @param {Accessor<string>} email The user's email address
 * @param {props} props
 */
const postRegister = async (email, props) => {
  /** @type {Promise<number|void|Response>} */
  let response = postEmail(email()).then((arrayBuffer) => {
    let uint8Array = new Uint8Array(arrayBuffer);

    let response, token, error;
    try {
      response = registerResponse.deserializeBinary(uint8Array);
      error = response.getError();
      if (response.hasToken()) {
        token = response.getToken();
        setCookie('register_email_token', token, 5);
        props.verificationMode();
      }
    } catch (decodeError) {
      console.error('Error decoding response:', decodeError);
      throw decodeError;
    }
  });

  return response;
};

/** @param {props} props */
const RegisterEmailForm = (props) => {
  const { email, setEmail } = useEmailContext();

  /** @param {SubmitEvent} e */
  function PostRegister(e) {
    e.preventDefault();
    postRegister(email, props);
  }

  return (
    <>
      <form onSubmit={PostRegister}>
        <input
          type="email"
          placeholder="email"
          onInput={(e) => setEmail(e.target.value)}
          value={email()}
          required
        />
        <input type="submit" value="Continue" />
      </form>
    </>
  );
};

export default RegisterEmailForm;
