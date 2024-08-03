import { createSignal } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { deleteCookie } from '../../../utils/cookies';
import { useEmailContext } from '../../context/EmailContext';
import { postDetails } from '../../functions/register/post.js';
const {
  Response: registerResponse
} = require('../../../protos/accounts/register/details/response.ts');

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */

/** @typedef {Object} props
 * @property {Function} emailMode - go to the first screen
 */

/**
 * @param {Accessor<string>} username The user's username
 * @param {Accessor<string>} password The user's password
 * @param {Accessor<string>} passwordConfirmation Confirmation of the user's password
 * @param {Accessor<string>} firstName The user's first name (optional)
 * @param {Accessor<string>} lastName The user's last name (optional)
 * @param {Function} setEmail Function to change the email
 */
const handlepostDetails = async (
  username,
  password,
  passwordConfirmation,
  firstName,
  lastName,
  setEmail
) => {
  postDetails(
    username(),
    password(),
    passwordConfirmation(),
    firstName(),
    lastName()
  ).then((arrayBuffer) => {
    let uint8Array = new Uint8Array(arrayBuffer);

    let response, success, error;
    try {
      response = registerResponse.deserializeBinary(uint8Array);
      error = response.getError();
      if (response.hasSuccess()) {
        success = response.getSuccess();
      }
    } catch (decodeError) {
      console.error('Error decoding response:', decodeError);
      throw decodeError;
    }

    if (response.hasSuccess()) {
      deleteCookie('register_verify_token');
      setEmail('');

      const navigate = useNavigate();
      navigate('/login', { replace: true });
    }
  });
};

/** @param {props} props */
const RegisterDetailsForm = (props) => {
  const [username, setUsername] = createSignal('');
  const [password, setPassword] = createSignal('');
  const [passwordConfirmation, setPasswordConfirmation] = createSignal('');
  const [firstName, setFirstName] = createSignal('');
  const [lastName, setLastName] = createSignal('');
  const { setEmail } = useEmailContext();

  /** @param {SubmitEvent} e */
  function PostRegister(e) {
    e.preventDefault();
    handlepostDetails(
      username,
      password,
      passwordConfirmation,
      firstName,
      lastName,
      setEmail
    );
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
          placeholder="username"
          onInput={(e) => setUsername(e.target.value)}
          required
        />
        <input
          type="password"
          placeholder="password"
          onInput={(e) => setPassword(e.target.value)}
          required
        />
        <input
          type="password"
          placeholder="password confirmation"
          onInput={(e) => setPasswordConfirmation(e.target.value)}
          required
        />
        <input
          type="text"
          placeholder="first name"
          onInput={(e) => setFirstName(e.target.value)}
        />
        <input
          type="text"
          placeholder="last name"
          onInput={(e) => setLastName(e.target.value)}
        />
        <input type="submit" value="Login" />
      </form>
    </>
  );
};

export default RegisterDetailsForm;
