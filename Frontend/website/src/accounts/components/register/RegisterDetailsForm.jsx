import { createSignal } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { useEmailContext } from '../../context/EmailContext';
import { handlePostDetails } from '../../functions/register/handlers';

/** @typedef { import ('../../types/Props').RegisterProps } Props */

/** @param {Props} props */
const RegisterDetailsForm = (props) => {
  const [username, setUsername] = createSignal('');
  const [password, setPassword] = createSignal('');
  const [passwordConfirmation, setPasswordConfirmation] = createSignal('');
  const [firstName, setFirstName] = createSignal('');
  const [lastName, setLastName] = createSignal('');
  const { setEmail } = useEmailContext();
  const navigate = useNavigate();

  return (
    <>
      <br />

      <button class="return" onclick={() => props.emailMode?.()}>
        x
      </button>

      <form onSubmit={(e) => handlePostDetails(e, username(), password(), passwordConfirmation(), firstName(), lastName(), setEmail, navigate)}>
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

